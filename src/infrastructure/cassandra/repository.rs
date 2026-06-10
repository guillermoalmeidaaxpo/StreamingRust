use async_trait::async_trait;
use crate::application::ports::Repository;
use crate::domain::{DataItem, ExecutableQuery};
use anyhow::Result;
use futures::Stream;
use std::pin::Pin;
use std::sync::Arc;
use scylla::{Session, SessionBuilder};
use async_stream::try_stream;
use futures::StreamExt;
use std::collections::HashMap;

pub struct ScyllaRepository {
    session: Arc<Session>,
}

impl ScyllaRepository {
    pub async fn new(hosts: &[String], keyspace: &str) -> Result<Self> {
        let session = SessionBuilder::new()
            .known_nodes(hosts)
            .use_keyspace(keyspace, true)
            .build()
            .await?;
        
        Ok(Self {
            session: Arc::new(session),
        })
    }
}

#[async_trait]
impl Repository for ScyllaRepository {
    async fn execute(&self, query: ExecutableQuery) -> Result<Vec<DataItem>> {
        tracing::info!("Executing Scylla query for ID: {}", query.id);
        
        // In a real implementation, we would build the CQL statement here
        // or use the one provided in query.statement if it's already built.
        let result = self.session.query(query.statement, &[]).await?;
        
        let mut items = Vec::new();
        if let Some(rows) = result.rows {
            for row in rows {
                // Mapping Scylla row to DataItem
                // This is a simplified mapping for demonstration
                let item = DataItem {
                    id: query.id,
                    fields: HashMap::new(), // Populate from row
                };
                items.push(item);
            }
        }
        
        Ok(items)
    }

    async fn stream(&self, query: ExecutableQuery) -> Result<Pin<Box<dyn Stream<Item = Result<DataItem>> + Send>>> {
        tracing::info!("Streaming Scylla query for ID: {}", query.id);
        
        let session = self.session.clone();
        let id = query.id;

        let stream = try_stream! {
            let mut pager = session.query_iter(query.statement, &[]).await?;
            while let Some(row_result) = pager.next().await {
                let _row = row_result?;
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
