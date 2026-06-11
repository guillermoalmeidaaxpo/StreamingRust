use serde::Deserialize;
use config::{Config, ConfigError, File, Environment};

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub meta: MetaConfig,
    pub http: HttpConfig,
    pub auth: AuthConfig,
    pub authorization_api: AuthorizationApiConfig,
    pub datastores: DatastoresConfig,
    pub database: DatabaseConfig,
    pub execution: ExecutionConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MetaConfig {
    pub build_number: String,
    pub stage: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HttpConfig {
    pub host: String,
    pub port: u16,
    pub read_header_timeout: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    pub mode: String,
    pub issuer: String,
    pub audiences: Vec<String>,
    pub allowed_roles: Vec<String>,
    pub require_https_metadata: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthorizationApiConfig {
    pub base_url: String,
    pub authorize_path: String,
    pub universe_authorize_path: String,
    pub timeout: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SqlConfig {
    pub dsn: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatastoresConfig {
    pub cmdp_sql: SqlConfig,
    pub mapping_sql: SqlConfig,
    pub mds_sql: SqlConfig,
    pub mesap_mapping_sql: Option<SqlConfig>,
    pub redis: RedisConfig,
    pub cassandra: CassandraConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CassandraConfig {
    pub data_centers: std::collections::HashMap<String, Vec<String>>,
    pub keyspace: String,
    pub port: u16,
    pub max_parallel_queries: usize,
    pub connection_timeout: String,
    pub read_timeout: String,
    pub table_mappings: std::collections::HashMap<String, String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub address: String,
    pub tls: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub connect_retry: RetryConfig,
    pub command_retry: RetryConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RetryConfig {
    pub timeout: Option<String>,
    pub count: usize,
    pub interval: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ExecutionConfig {
    pub batch_size: usize,
    pub stream_batch_size: usize,
    pub optimized_batch_size: usize,
    pub max_sql_parallel: usize,
    pub max_cassandra_parallel: usize,
    pub max_sql_connections: u32,
    pub max_cassandra_connections: usize,
    pub reference_time_split_days: i64,
    pub cassandra_reference_time_split_days: i64,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let env = std::env::var("OUTBOUND_ENV").unwrap_or_else(|_| "development".to_string()).to_lowercase();
        let config_dir = std::env::var("OUTBOUND_CONFIG_DIR").unwrap_or_else(|_| "configs".to_string());

        let default_path = format!("{}/default.yaml", config_dir);
        let env_path = format!("{}/{}.yaml", config_dir, env);

        let s = Config::builder()
            .add_source(File::with_name(&default_path).required(false))
            .add_source(File::with_name(&env_path).required(false))
            .add_source(Environment::with_prefix("OUTBOUND").separator("__"))
            .build()?;

        s.try_deserialize()
    }
}
