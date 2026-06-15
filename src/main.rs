use std::sync::Arc;
use std::collections::HashMap;
use streaming_rust::domain::SourceKind;
use streaming_rust::application::pipeline::Pipeline;
use streaming_rust::application::validator::{RequestValidator, RequestValidationStrategyResolver, TransactionalDataValidationStrategy, DataRowsNumberValidator};
use streaming_rust::application::filter_engine::FilterProvider;
use streaming_rust::infrastructure::antlr_parser::AntlrFilterParser;
use streaming_rust::infrastructure::mssql::mapping_resolver::MssqlMappingResolver;
use streaming_rust::infrastructure::mssql::repository::MssqlRepository;
use streaming_rust::infrastructure::cassandra::repository::ScyllaRepository;
use streaming_rust::infrastructure::http::router::{create_router, AppState};
use streaming_rust::infrastructure::config::AppConfig;
use tracing_subscriber::prelude::*;

use streaming_rust::infrastructure::throttling::{RedisCmdpGlobalConnectionGate, NullConnectionGate, ConnectionGate};
use fred::clients::RedisClient;
use fred::interfaces::ClientLike;

#[tokio::main]
async fn main() {
    // Initialize Logger (Dynamic: structured JSON for AKS/productive/validation/migration, pretty ANSI for local dev)
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    let is_json = std::env::var("LOG_FORMAT").map(|v| v.to_lowercase() == "json").unwrap_or(false)
        || std::env::var("OUTBOUND_ENV").map(|v| {
            let l = v.to_lowercase();
            l == "production" || l == "productive" || l == "validation" || l == "migration"
        }).unwrap_or(false);

    // Create the formatting layer
    let fmt_layer = if is_json {
        tracing_subscriber::fmt::layer()
            .json()
            .flatten_event(true)
            .boxed()
    } else {
        tracing_subscriber::fmt::layer()
            .with_ansi(true)
            .boxed()
    };

    let registry = tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer);

    // Optional OpenTelemetry (OTLP) layer, active if OTLP endpoint is set (standard for AKS App Insights)
    let otel_endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .unwrap_or_else(|_| "NOT SET".to_string());

    if otel_endpoint != "NOT SET" && !otel_endpoint.is_empty() {
        use opentelemetry_otlp::WithExportConfig;
        use opentelemetry::trace::TracerProvider as _;
        
        let provider = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(
                opentelemetry_otlp::new_exporter()
                    .tonic()
                    .with_endpoint(otel_endpoint.clone()),
            )
            .install_batch(opentelemetry_sdk::runtime::Tokio)
            .expect("Failed to initialize OpenTelemetry OTLP provider");

        let tracer = provider.tracer("streaming-rust");
        let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);
        registry.with(telemetry_layer).init();
        tracing::info!("OpenTelemetry tracing enabled via OTLP endpoint: {}", otel_endpoint);
    } else {
        registry.init();
    }

    // 0. Load Configuration
    let config = AppConfig::load().expect("Failed to load configuration");
    tracing::info!("Configuration loaded successfully: build {}, stage {}", config.meta.build_number, config.meta.stage);

    // Initialize Redis & Connection Gate
    let gate: Arc<dyn ConnectionGate> = if config.datastores.redis.address.trim() != "NOT SET" && !config.datastores.redis.address.is_empty() {
        let redis_config = fred::types::RedisConfig::from_url(&config.datastores.redis.address).expect("Invalid Redis URL");
        let redis_client = RedisClient::new(redis_config, None, None, None);
        redis_client.connect();
        
        // Wait for connection or fail open gracefully in background
        let _ = redis_client.wait_for_connect().await;
        
        Arc::new(RedisCmdpGlobalConnectionGate::new(
            redis_client,
            config.execution.max_sql_parallel, // Global limit
            300, // 5 minutes TTL
            10, // Max retries
            500, // Retry base delay MS
        ))
    } else {
        tracing::warn!("Redis address NOT SET. Global connection gating is DISABLED.");
        Arc::new(NullConnectionGate)
    };

    // 1. Initialize Adapters
    let parser = Arc::new(AntlrFilterParser::new());
    let resolver = Arc::new(MssqlMappingResolver::new(
        &config.datastores.mapping_sql.dsn,
        &config.datastores.mds_sql.dsn,
        &config.datastores.cmdp_sql.dsn,
        config.execution.max_sql_connections,
    ).await.expect("Failed to initialize mapping resolver"));
    
    let cmdp_repo = Arc::new(MssqlRepository::new(
        &config.datastores.cmdp_sql.dsn,
        config.execution.max_sql_connections,
        gate.clone(),
    ).await.expect("Failed to initialize CMDP repository"));

    let mds_repo = Arc::new(MssqlRepository::new(
        &config.datastores.mds_sql.dsn,
        config.execution.max_sql_connections,
        gate,
    ).await.expect("Failed to initialize MDS repository"));
    
    let mut cassandra_hosts: Vec<String> = config.datastores.cassandra.data_centers.values()
        .flat_map(|v| v.clone())
        .collect();
    if cassandra_hosts.is_empty() {
        cassandra_hosts.push("localhost".to_string());
    }
    let scylla_repo = Arc::new(ScyllaRepository::new(&cassandra_hosts, &config.datastores.cassandra.keyspace, config.execution.max_cassandra_connections).await.expect("Failed to initialize Scylla repository"));
    
    let mut repositories: HashMap<SourceKind, Arc<dyn streaming_rust::application::ports::Repository>> = HashMap::new();
    repositories.insert(SourceKind::Cmdp, cmdp_repo.clone());
    repositories.insert(SourceKind::Cassandra, scylla_repo.clone());
    repositories.insert(SourceKind::Hyperscale, mds_repo.clone());

    // 2. Initialize Validation Matrix
    let validator = Arc::new(RequestValidator::new());
    let strategies: Vec<Arc<dyn streaming_rust::application::ports::RequestValidationStrategy>> = vec![
        Arc::new(TransactionalDataValidationStrategy),
        Arc::new(streaming_rust::application::validator::GenericRequestValidationStrategy {
            details_validator: streaming_rust::application::validator::GenericRequestDetailsValidator::new(parser.clone()),
        }),
    ];
    let validation_resolver = Arc::new(RequestValidationStrategyResolver::new(strategies));
    let stats_service = Arc::new(
        streaming_rust::infrastructure::mssql::statistics::MssqlStatisticsService::new(
            &config.datastores.mapping_sql.dsn,
            resolver.clone(),
            config.meta.stage.clone(),
            config.execution.max_sql_connections,
        )
        .await
        .expect("Failed to initialize MSSQL statistics service")
    );
    let row_validator = DataRowsNumberValidator::new(stats_service, parser.clone(), config.execution.batch_size as u64);

    // 3. Initialize Planner
    let filter_provider = Arc::new(FilterProvider::new(cmdp_repo.clone()));
    let cassandra_builder = streaming_rust::infrastructure::cassandra::query_builder::CassandraQueryBuilder::new(
        config.datastores.cassandra.table_mappings.clone(),
        Some(config.datastores.cassandra.keyspace.clone())
    );
    let planner = Arc::new(streaming_rust::application::planner::DefaultPlanner::new(
        resolver.clone(), 
        parser.clone(), 
        filter_provider.clone(),
        cassandra_builder
    ));

    // 4. Initialize Core Pipeline
    let pipeline = Arc::new(Pipeline::new(
        validator,
        validation_resolver.clone(),
        row_validator,
        planner,
        repositories
    ));

    let license_validator: Arc<dyn streaming_rust::application::ports::LicenseValidator> = Arc::new(
        streaming_rust::infrastructure::auth::license::HttpLicenseValidator::new(
            config.authorization_api.base_url.clone(),
            config.authorization_api.authorize_path.clone(),
            config.authorization_api.universe_authorize_path.clone(),
        )
    );

    let state = Arc::new(AppState { 
        pipeline,
        auth_config: config.auth.clone(),
        meta_config: config.meta.clone(),
        execution_config: config.execution.clone(),
        validation_resolver,
        jwk_store: Arc::new(streaming_rust::infrastructure::auth::middleware::JwkStore::new(config.auth.issuer.clone())),
        license_validator,
    });

    // 5. Start HTTP Server
    let app = create_router(state);
    let addr = format!("{}:{}", config.http.host, config.http.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Streaming Rust API listening on {}", addr);
    
    axum::serve(listener, app).await.unwrap();
}
