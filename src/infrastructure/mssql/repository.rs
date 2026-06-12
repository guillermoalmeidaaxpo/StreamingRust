use async_trait::async_trait;
use crate::application::ports::Repository;
use crate::domain::{DataItem, ExecutableQuery};
use anyhow::Result;
use futures::Stream;
use std::pin::Pin;
use bb8_tiberius::ConnectionManager;
use bb8::Pool;
use async_stream::try_stream;
use futures::StreamExt;
use std::collections::HashMap;
use std::sync::Arc;
use crate::infrastructure::throttling::ConnectionGate;
use regex::Regex;

pub struct MssqlRepository {
    pool: Pool<ConnectionManager>,
    gate: Arc<dyn ConnectionGate>,
}

impl MssqlRepository {
    pub async fn new(connection_string: &str, max_connections: u32, gate: Arc<dyn ConnectionGate>) -> Result<Self> {
        let config = super::get_mssql_config(connection_string).await?;
        let manager = ConnectionManager::new(config);
        let pool = Pool::builder()
            .max_size(max_connections)
            .build(manager)
            .await?;

        Ok(Self { pool, gate })
    }
}

#[async_trait]
impl Repository for MssqlRepository {
    async fn execute(&self, query: ExecutableQuery) -> Result<Vec<DataItem>> {
        let (statement_prepared, args) = prepare_query(&query.statement, &query.parameters);
        tracing::info!("Executing MSSQL query for ID: {}. Statement: {} with parameters: {:?}", query.id, statement_prepared, args);

        // 1. Acquire global connection slot
        let mut releaser = self.gate.acquire().await?;

        let mut conn = self.pool.get().await?;
        let mut tib_query = tiberius::Query::new(statement_prepared);
        for val in &args {
            match val {
                serde_json::Value::Null => {
                    tib_query.bind(Option::<&str>::None);
                }
                serde_json::Value::Bool(b) => {
                    tib_query.bind(*b);
                }
                serde_json::Value::Number(num) => {
                    if let Some(i) = num.as_i64() {
                        tib_query.bind(i);
                    } else if let Some(f) = num.as_f64() {
                        tib_query.bind(f);
                    }
                }
                serde_json::Value::String(s) => {
                    tib_query.bind(s.as_str());
                }
                serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
                    tib_query.bind(val.to_string());
                }
            }
        }

        let mut stream = tib_query.query(&mut conn).await?;
        let mut items = Vec::new();
        while let Some(row_res) = stream.next().await {
            let _row = row_res?;
            items.push(DataItem {
                id: query.id,
                fields: HashMap::new(),
            });
        }

        // 2. Release global connection slot
        releaser.release().await;

        Ok(items)
    }

    async fn stream(&self, query: ExecutableQuery) -> Result<Pin<Box<dyn Stream<Item = Result<DataItem>> + Send>>> {
        let pool = self.pool.clone();
        let id = query.id;
        let (statement_prepared, args) = prepare_query(&query.statement, &query.parameters);
        tracing::info!("Streaming MSSQL query for ID: {}. Statement: {} with parameters: {:?}", id, statement_prepared, args);

        // 1. Acquire global connection slot
        let mut releaser = self.gate.acquire().await?;

        let stream = try_stream! {
            let mut conn = pool.get().await?;
            let mut tib_query = tiberius::Query::new(&statement_prepared);
            for val in &args {
                match val {
                    serde_json::Value::Null => {
                        tib_query.bind(Option::<&str>::None);
                    }
                    serde_json::Value::Bool(b) => {
                        tib_query.bind(*b);
                    }
                    serde_json::Value::Number(num) => {
                        if let Some(i) = num.as_i64() {
                            tib_query.bind(i);
                        } else if let Some(f) = num.as_f64() {
                            tib_query.bind(f);
                        }
                    }
                    serde_json::Value::String(s) => {
                        tib_query.bind(s.as_str());
                    }
                    serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
                        tib_query.bind(val.to_string());
                    }
                }
            }

            let mut stream = tib_query.query(&mut conn).await?;

            while let Some(item) = stream.next().await {
                let _row = item?;
                yield DataItem {
                    id,
                    fields: HashMap::new(),
                };
            }

            // 2. Release global connection slot when stream completes
            releaser.release().await;
        };

        Ok(Box::pin(stream))
    }
}

fn prepare_query(statement: &str, params: &HashMap<String, serde_json::Value>) -> (String, Vec<serde_json::Value>) {
    let re = Regex::new(r"@([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();
    let mut new_statement = String::new();
    let mut args = Vec::new();
    let mut last_idx = 0;

    for cap in re.captures_iter(statement) {
        let mat = cap.get(0).unwrap();
        let name = cap.get(1).unwrap().as_str();

        new_statement.push_str(&statement[last_idx..mat.start()]);

        if let Some(val) = params.get(name) {
            args.push(val.clone());
            let param_placeholder = format!("@p{}", args.len());
            new_statement.push_str(&param_placeholder);
        } else {
            new_statement.push_str(mat.as_str());
        }
        last_idx = mat.end();
    }
    new_statement.push_str(&statement[last_idx..]);

    (new_statement, args)
}

