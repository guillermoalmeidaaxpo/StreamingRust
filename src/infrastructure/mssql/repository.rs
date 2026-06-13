use async_trait::async_trait;
use crate::application::ports::Repository;
use crate::domain::{DataItem, ExecutableQuery};
use anyhow::Result;
use futures::Stream;
use std::pin::Pin;
use super::ConnectionManager;
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
        let manager = ConnectionManager::new(connection_string)?;
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
            let row = row_res?;
            let fields = map_tiberius_row(&row);
            items.push(DataItem {
                id: query.id,
                fields,
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
                let row = item?;
                let fields = map_tiberius_row(&row);
                yield DataItem {
                    id,
                    fields,
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

fn map_tiberius_row(row: &tiberius::Row) -> HashMap<String, serde_json::Value> {
    let mut fields = HashMap::new();
    for (i, col) in row.columns().iter().enumerate() {
        let name = col.name().to_string();
        let val = match col.column_type() {
            tiberius::ColumnType::Bit => {
                row.try_get::<bool, _>(i).ok().flatten().map(serde_json::Value::Bool).unwrap_or(serde_json::Value::Null)
            }
            tiberius::ColumnType::Int1 => {
                row.try_get::<u8, _>(i).ok().flatten().map(|n| serde_json::Value::Number(n.into())).unwrap_or(serde_json::Value::Null)
            }
            tiberius::ColumnType::Int2 => {
                row.try_get::<i16, _>(i).ok().flatten().map(|n| serde_json::Value::Number(n.into())).unwrap_or(serde_json::Value::Null)
            }
            tiberius::ColumnType::Int4 => {
                row.try_get::<i32, _>(i).ok().flatten().map(|n| serde_json::Value::Number(n.into())).unwrap_or(serde_json::Value::Null)
            }
            tiberius::ColumnType::Int8 => {
                row.try_get::<i64, _>(i).ok().flatten().map(|n| serde_json::Value::Number(n.into())).unwrap_or(serde_json::Value::Null)
            }
            tiberius::ColumnType::Intn => {
                if let Ok(Some(n)) = row.try_get::<i64, _>(i) {
                    serde_json::Value::Number(n.into())
                } else if let Ok(Some(n)) = row.try_get::<i32, _>(i) {
                    serde_json::Value::Number(n.into())
                } else if let Ok(Some(n)) = row.try_get::<i16, _>(i) {
                    serde_json::Value::Number(n.into())
                } else if let Ok(Some(n)) = row.try_get::<u8, _>(i) {
                    serde_json::Value::Number(n.into())
                } else {
                    serde_json::Value::Null
                }
            }
            tiberius::ColumnType::Float4 => {
                row.try_get::<f32, _>(i).ok().flatten().and_then(|f| serde_json::Number::from_f64(f as f64)).map(serde_json::Value::Number).unwrap_or(serde_json::Value::Null)
            }
            tiberius::ColumnType::Float8 => {
                row.try_get::<f64, _>(i).ok().flatten().and_then(|f| serde_json::Number::from_f64(f)).map(serde_json::Value::Number).unwrap_or(serde_json::Value::Null)
            }
            tiberius::ColumnType::Floatn => {
                if let Ok(Some(f)) = row.try_get::<f64, _>(i) {
                    serde_json::Number::from_f64(f).map(serde_json::Value::Number).unwrap_or(serde_json::Value::Null)
                } else if let Ok(Some(f)) = row.try_get::<f32, _>(i) {
                    serde_json::Number::from_f64(f as f64).map(serde_json::Value::Number).unwrap_or(serde_json::Value::Null)
                } else {
                    serde_json::Value::Null
                }
            }
            tiberius::ColumnType::Numericn | tiberius::ColumnType::Decimaln => {
                if let Some(num) = row.try_get::<tiberius::numeric::Numeric, _>(i).ok().flatten() {
                    let val = num.value() as f64 / 10.0f64.powi(num.scale() as i32);
                    serde_json::Number::from_f64(val).map(serde_json::Value::Number).unwrap_or(serde_json::Value::Null)
                } else {
                    serde_json::Value::Null
                }
            }
            tiberius::ColumnType::BigVarChar | tiberius::ColumnType::NVarchar | tiberius::ColumnType::BigChar | tiberius::ColumnType::NChar | tiberius::ColumnType::Text | tiberius::ColumnType::NText => {
                row.try_get::<&str, _>(i).ok().flatten().map(|s| serde_json::Value::String(s.to_string())).unwrap_or(serde_json::Value::Null)
            }
            tiberius::ColumnType::Datetime | tiberius::ColumnType::Datetime2 | tiberius::ColumnType::Datetime4 | tiberius::ColumnType::Datetimen => {
                if let Some(dt) = row.try_get::<chrono::NaiveDateTime, _>(i).ok().flatten() {
                    serde_json::Value::String(dt.format("%Y-%m-%dT%H:%M:%S.000").to_string())
                } else {
                    serde_json::Value::Null
                }
            }
            tiberius::ColumnType::DatetimeOffsetn => {
                if let Some(dt) = row.try_get::<chrono::DateTime<chrono::FixedOffset>, _>(i).ok().flatten() {
                    serde_json::Value::String(dt.to_rfc3339())
                } else {
                    serde_json::Value::Null
                }
            }
            tiberius::ColumnType::Guid => {
                row.try_get::<uuid::Uuid, _>(i).ok().flatten().map(|uid| serde_json::Value::String(uid.to_string())).unwrap_or(serde_json::Value::Null)
            }
            _ => {
                if let Some(s) = row.try_get::<&str, _>(i).ok().flatten() {
                    serde_json::Value::String(s.to_string())
                } else if let Some(f) = row.try_get::<f64, _>(i).ok().flatten() {
                    serde_json::Number::from_f64(f).map(serde_json::Value::Number).unwrap_or(serde_json::Value::Null)
                } else if let Some(n) = row.try_get::<i64, _>(i).ok().flatten() {
                    serde_json::Value::Number(n.into())
                } else {
                    serde_json::Value::Null
                }
            }
        };
        fields.insert(name, val);
    }
    fields
}

