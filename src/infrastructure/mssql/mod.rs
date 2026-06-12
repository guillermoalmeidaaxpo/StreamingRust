pub mod mapping_resolver;
pub mod repository;
pub mod query_builder;
pub mod statistics;

use async_trait::async_trait;
use azure_core::auth::TokenCredential;
use azure_identity::DefaultAzureCredential;
use bb8::ManageConnection;
use std::sync::Arc;
use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

pub struct ConnectionManager {
    connection_string: String,
    credential: Arc<DefaultAzureCredential>,
}

impl ConnectionManager {
    pub fn new(connection_string: &str) -> anyhow::Result<Self> {
        let credential = DefaultAzureCredential::create(Default::default())
            .map_err(|e| anyhow::anyhow!("Failed to create DefaultAzureCredential: {}", e))?;
        Ok(Self {
            connection_string: connection_string.to_string(),
            credential: Arc::new(credential),
        })
    }

    async fn build_config(&self) -> anyhow::Result<Config> {
        let mut config = Config::from_ado_string(&self.connection_string)
            .map_err(|e| anyhow::anyhow!("Failed to parse ADO connection string: {}", e))?;

        if self.connection_string.contains("ActiveDirectoryDefault") || self.connection_string.contains("fedauth") {
            tracing::info!("Azure Active Directory Authentication: Fetching fresh access token...");
            let token_res = self.credential
                .get_token(&["https://database.windows.net/.default"])
                .await
                .map_err(|e| anyhow::anyhow!("Failed to retrieve Azure AD access token: {}", e))?;
            config.authentication(AuthMethod::AADToken(token_res.token.secret().to_string()));
        }
        Ok(config)
    }
}

#[async_trait]
impl ManageConnection for ConnectionManager {
    type Connection = Client<Compat<TcpStream>>;
    type Error = anyhow::Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let config = self.build_config().await?;
        let tcp = TcpStream::connect(config.get_addr()).await?;
        tcp.set_nodelay(true)?;
        let client = Client::connect(config, tcp.compat_write()).await?;
        Ok(client)
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        conn.simple_query("SELECT 1").await?;
        Ok(())
    }

    fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
        false
    }
}
