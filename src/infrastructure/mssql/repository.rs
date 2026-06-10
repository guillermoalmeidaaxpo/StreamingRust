use async_trait::async_trait;
use crate::application::ports::Repository;
use crate::domain::{DataItem, ExecutableQuery};
use anyhow::Result;
use futures::Stream;
use std::pin::Pin;
use tiberius::Config;
use bb8_tiberius::ConnectionManager;
use bb8::Pool;
use async_stream::try_stream;
use futures::StreamExt;
use std::collections::HashMap;

pub struct MssqlRepository {
    pool: Pool<ConnectionManager>,
}

impl MssqlRepository {
    pub async fn new(connection_string: &str) -> Result<Self> {
        let config = Config::from_ado_string(connection_string)?;
        let manager = ConnectionManager::new(config);
        let pool = Pool::builder().build(manager).await?;
        
        Ok(Self { pool })
    }
}

#[async_trait]
impl Repository for MssqlRepository {
    async fn execute(&self, query: ExecutableQuery) -> Result<Vec<DataItem>> {
        tracing::info!("Executing MSSQL query for ID: {}", query.id);
        
        let mut conn = self.pool.get().await?;
        let _result = conn.simple_query(query.statement).await?;
        
        let items = Vec::new();
        // Tiberius result processing
        // result.into_first_result().await? ...
        
        Ok(items)
    }

    async fn stream(&self, query: ExecutableQuery) -> Result<Pin<Box<dyn Stream<Item = Result<DataItem>> + Send>>> {
        tracing::info!("Streaming MSSQL query for ID: {}", query.id);
        
        let pool = self.pool.clone();
        let id = query.id;

        let stream = try_stream! {
            let mut conn = pool.get().await?;
            let mut stream = conn.simple_query(query.statement).await?;

            
            while let Some(item) = stream.next().await {
                let _row = item?;
                // Map row to DataItem
                yield DataItem {
                    id,
                    fields: HashMap::new(),
                };
            }
        };
        
        Ok(Box::pin(stream))
    }
}
