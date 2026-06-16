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
use chrono::{TimeZone, Utc};
use scylla::frame::value::LegacySerializedValues as SerializedValues;

pub struct ScyllaRepository {
    session: Arc<Session>,
}

impl ScyllaRepository {
    pub async fn new(hosts: &[String], keyspace: &str, max_connections: usize) -> Result<Self> {
        let mut builder = SessionBuilder::new()
            .known_nodes(hosts)
            .use_keyspace(keyspace, true);

        if max_connections > 0 {
            builder = builder.pool_size(scylla::transport::session::PoolSize::PerHost(
                std::num::NonZeroUsize::new(max_connections).unwrap()
            ));
        }

        let session = builder.build().await?;
        
        Ok(Self {
            session: Arc::new(session),
        })
    }
}

fn build_serialized_values(args: &[serde_json::Value]) -> anyhow::Result<SerializedValues> {
    let mut vals = SerializedValues::new();
    for (i, val) in args.iter().enumerate() {
        if i == 0 {
            if let Some(s) = val.as_str() {
                vals.add_value(&s).map_err(|e| anyhow::anyhow!("Scylla serialization error: {:?}", e))?;
            } else {
                vals.add_value(&"").map_err(|e| anyhow::anyhow!("Scylla serialization error: {:?}", e))?;
            }
        } else if i == 1 {
            let mut list = Vec::new();
            if let Some(arr) = val.as_array() {
                for item in arr {
                    if let Some(n) = item.as_i64() {
                        list.push(n as i32);
                    }
                }
            }
            vals.add_value(&list).map_err(|e| anyhow::anyhow!("Scylla serialization error: {:?}", e))?;
        } else {
            let offset = i - 2;
            match offset % 4 {
                0 => {
                    let n = val.as_i64().unwrap_or(1) as i16;
                    vals.add_value(&n).map_err(|e| anyhow::anyhow!("Scylla serialization error: {:?}", e))?;
                }
                1 => {
                    let n = val.as_i64().unwrap_or(1) as i8;
                    vals.add_value(&n).map_err(|e| anyhow::anyhow!("Scylla serialization error: {:?}", e))?;
                }
                2 => {
                    let n = val.as_i64().unwrap_or(1) as i8;
                    vals.add_value(&n).map_err(|e| anyhow::anyhow!("Scylla serialization error: {:?}", e))?;
                }
                _ => {
                    let n = val.as_i64().unwrap_or(0) as i8;
                    vals.add_value(&n).map_err(|e| anyhow::anyhow!("Scylla serialization error: {:?}", e))?;
                }
            }
        }
    }
    Ok(vals)
}

fn map_scylla_row(
    row: scylla::frame::response::result::Row,
    query: &ExecutableQuery,
) -> anyhow::Result<DataItem> {
    let id = query.id;
    let (
        _ts_id,
        qte_y,
        qte_m,
        qte_d,
        _quote_index,
        _publish_time,
        del_y,
        del_m,
        del_d,
        del_h,
        del_min,
        del_offset,
        value,
    ) = row.into_typed::<(
        String,
        i16,
        i8,
        i8,
        i32,
        Option<chrono::DateTime<Utc>>,
        i16,
        i8,
        i8,
        i8,
        i8,
        i8,
        f64,
    )>().map_err(|e| anyhow::anyhow!("Failed to decode Scylla row: {:?}", e))?;

    let timezone_str = query.parameters.get("timezone").and_then(|v| v.as_str()).unwrap_or("Europe/Zurich");
    let tz: chrono_tz::Tz = crate::domain::source::parse_timezone(timezone_str).unwrap_or(chrono_tz::Tz::UTC);

    let ref_naive = chrono::NaiveDate::from_ymd_opt(
        qte_y as i32,
        qte_m as u32,
        qte_d as u32,
    ).ok_or_else(|| anyhow::anyhow!("Invalid quote date: {}-{}-{}", qte_y, qte_m, qte_d))?
    .and_hms_opt(0, 0, 0)
    .ok_or_else(|| anyhow::anyhow!("Invalid quote time"))?;

    let ref_time = tz.from_local_datetime(&ref_naive).earliest().unwrap_or_else(|| {
        chrono::Utc.from_local_datetime(&ref_naive).unwrap().with_timezone(&tz)
    });
    let ref_time_str = ref_time.format("%Y-%m-%dT%H:%M:%S.000%:z").to_string();

    let offset_secs = (del_offset as i32) * 3600;
    let offset = chrono::FixedOffset::east_opt(offset_secs).unwrap();
    
    let del_start_naive = chrono::NaiveDate::from_ymd_opt(
        del_y as i32,
        del_m as u32,
        del_d as u32,
    ).ok_or_else(|| anyhow::anyhow!("Invalid delivery start date: {}-{}-{}", del_y, del_m, del_d))?
    .and_hms_opt(
        del_h as u32,
        del_min as u32,
        0,
    ).ok_or_else(|| anyhow::anyhow!("Invalid delivery start time: {}:{}", del_h, del_min))?;
    
    let del_start = chrono::DateTime::<chrono::FixedOffset>::from_naive_utc_and_offset(del_start_naive - chrono::Duration::seconds(offset_secs as i64), offset);
    
    let include_offset = query.parameters.get("include_offset").and_then(|v| v.as_bool()).unwrap_or(false);
    let del_start_str = if include_offset {
        del_start.format("%Y-%m-%dT%H:%M:%S.000%:z").to_string()
    } else {
        del_start.format("%Y-%m-%dT%H:%M:%S.000").to_string()
    };

    let del_end = del_start + chrono::Duration::hours(1);
    let del_end_str = if include_offset {
        del_end.format("%Y-%m-%dT%H:%M:%S.000%:z").to_string()
    } else {
        del_end.format("%Y-%m-%dT%H:%M:%S.000").to_string()
    };

    let value_rounded = (value * 1e10).round() / 1e10;
    let value_json = if value_rounded.fract() == 0.0 {
        serde_json::Value::Number((value_rounded as i64).into())
    } else {
        serde_json::Number::from_f64(value_rounded)
            .map(serde_json::Value::Number)
            .unwrap_or(serde_json::Value::Null)
    };

    let mut fields = HashMap::new();
    fields.insert("Identifier".to_string(), serde_json::Value::Number(id.into()));
    fields.insert("ReferenceTime".to_string(), serde_json::Value::String(ref_time_str));
    fields.insert("DeliveryStart".to_string(), serde_json::Value::String(del_start_str));
    fields.insert("DeliveryEnd".to_string(), serde_json::Value::String(del_end_str));
    fields.insert("Value".to_string(), value_json);
    fields.insert("LegacyDeliveryBucketNumber".to_string(), serde_json::Value::Null);
    fields.insert("RelativeDeliveryPeriod".to_string(), serde_json::Value::Null);

    let mut projected_fields = HashMap::new();
    if let Some(serde_json::Value::Array(cols)) = query.parameters.get("projection_columns") {
        let mut contract_cols = vec![
            "Identifier",
            "ReferenceTime",
            "DeliveryStart",
            "DeliveryEnd",
            "LegacyDeliveryBucketNumber",
            "RelativeDeliveryPeriod",
            "Value",
        ];
        for col_val in cols {
            if let Some(col) = col_val.as_str() {
                if !contract_cols.iter().any(|c| c.eq_ignore_ascii_case(col)) {
                    contract_cols.push(col);
                }
            }
        }
        for col in contract_cols {
            let actual_key = fields.keys().find(|k| k.eq_ignore_ascii_case(col));
            if let Some(key) = actual_key {
                projected_fields.insert(key.clone(), fields[key].clone());
            }
        }
    } else {
        projected_fields = fields;
    }

    Ok(DataItem {
        id,
        fields: projected_fields,
    })
}

#[async_trait]
impl Repository for ScyllaRepository {
    async fn execute(&self, query: ExecutableQuery) -> Result<Vec<DataItem>> {
        tracing::info!("Executing Scylla query for ID: {}. Statement: {} with arguments: {:?}", query.id, query.statement, query.arguments);
        
        let vals = build_serialized_values(&query.arguments)?;
        let result = self.session.query(query.statement.clone(), vals).await?;
        
        let mut items = Vec::new();
        if let Some(rows) = result.rows {
            for row in rows {
                let item = map_scylla_row(row, &query)?;
                items.push(item);
            }
        }
        
        Ok(items)
    }

    async fn stream(&self, query: ExecutableQuery) -> Result<Pin<Box<dyn Stream<Item = Result<DataItem>> + Send>>> {
        tracing::info!("Streaming Scylla query for ID: {}. Statement: {} with arguments: {:?}", query.id, query.statement, query.arguments);
        
        let session = self.session.clone();
        let vals = build_serialized_values(&query.arguments)?;

        let stream = try_stream! {
            let mut pager = session.query_iter(query.statement.clone(), vals).await.map_err(|e| {
                tracing::error!("Scylla query_iter error for ID {}: {:?}", query.id, e);
                e
            })?;
            while let Some(row_result) = pager.next().await {
                let row = row_result.map_err(|e| {
                    tracing::error!("Scylla row_result error for ID {}: {:?}", query.id, e);
                    e
                })?;
                let item = map_scylla_row(row, &query).map_err(|e| {
                    tracing::error!("Scylla map_scylla_row error for ID {}: {:?}", query.id, e);
                    e
                })?;
                yield item;
            }
        };
        
        Ok(Box::pin(stream))
    }
}
