pub mod mapping_resolver;
pub mod repository;
pub mod query_builder;

use azure_core::auth::TokenCredential;
use azure_identity::DefaultAzureCredential;
use tiberius::AuthMethod;

pub async fn get_mssql_config(connection_string: &str) -> anyhow::Result<tiberius::Config> {
    let mut config = tiberius::Config::from_ado_string(connection_string)
        .map_err(|e| anyhow::anyhow!("Failed to parse ADO connection string: {}", e))?;

    if connection_string.contains("ActiveDirectoryDefault") || connection_string.contains("fedauth") {
        tracing::info!("Azure Active Directory Authentication detected. Fetching access token...");
        let credential = DefaultAzureCredential::create(Default::default())
            .map_err(|e| anyhow::anyhow!("Failed to create DefaultAzureCredential: {}", e))?;
        let token_res = credential
            .get_token(&["https://database.windows.net/.default"])
            .await
            .map_err(|e| anyhow::anyhow!("Failed to retrieve Azure AD access token: {}", e))?;
        
        config.authentication(AuthMethod::AADToken(token_res.token.secret().to_string()));
    }

    Ok(config)
}
