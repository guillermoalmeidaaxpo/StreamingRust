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
use async_trait::async_trait;

struct MockStatsService;
#[async_trait]
impl streaming_rust::application::ports::StatisticsService for MockStatsService {
    async fn estimate_rows(&self, _ids: &[streaming_rust::domain::Identifier], _filters: &streaming_rust::domain::FilterSet) -> anyhow::Result<u64> {
        Ok(0)
    }
}

use streaming_rust::infrastructure::throttling::{RedisCmdpGlobalConnectionGate, NullConnectionGate, ConnectionGate};
use fred::clients::RedisClient;
use fred::interfaces::ClientLike;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

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
    let resolver = Arc::new(MssqlMappingResolver::new(&config.datastores.mapping_sql, &config.datastores.mds_sql).await.expect("Failed to initialize mapping resolver"));
    
    let cmdp_repo = Arc::new(MssqlRepository::new(&config.datastores.cmdp_sql, gate).await.expect("Failed to initialize MSSQL repository"));
    let scylla_repo = Arc::new(ScyllaRepository::new(&config.datastores.cassandra.hosts, &config.datastores.cassandra.keyspace).await.expect("Failed to initialize Scylla repository"));
    
    let mut repositories: HashMap<SourceKind, Arc<dyn streaming_rust::application::ports::Repository>> = HashMap::new();
    repositories.insert(SourceKind::Cmdp, cmdp_repo.clone());
    repositories.insert(SourceKind::Cassandra, scylla_repo.clone());
    repositories.insert(SourceKind::Hyperscale, cmdp_repo.clone());

    // 2. Initialize Validation Matrix
    let validator = Arc::new(RequestValidator::new());
    let strategies: Vec<Arc<dyn streaming_rust::application::ports::RequestValidationStrategy>> = vec![
        Arc::new(TransactionalDataValidationStrategy),
    ];
    let validation_resolver = RequestValidationStrategyResolver::new(strategies);
    let stats_service = Arc::new(MockStatsService);
    let row_validator = DataRowsNumberValidator::new(stats_service, config.execution.batch_size as u64);

    // 3. Initialize Planner
    let cassandra_builder = streaming_rust::infrastructure::cassandra::query_builder::CassandraQueryBuilder::new(
        config.datastores.cassandra.table_mappings.clone(),
        Some(config.datastores.cassandra.keyspace.clone())
    );
    let planner = Arc::new(streaming_rust::application::planner::DefaultPlanner::new(resolver.clone(), parser.clone(), cassandra_builder));

    // 4. Initialize Core Pipeline
    let pipeline = Arc::new(Pipeline::new(
        validator,
        validation_resolver,
        row_validator,
        planner,
        repositories
    ));

    let state = Arc::new(AppState { 
        pipeline,
        auth_config: config.auth.clone(),
    });

    // 5. Start HTTP Server
    let app = create_router(state);
    let addr = format!("{}:{}", config.http.host, config.http.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Streaming Rust API listening on {}", addr);
    
    axum::serve(listener, app).await.unwrap();
}
