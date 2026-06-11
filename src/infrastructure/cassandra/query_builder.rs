use crate::domain::{ExecutableQuery, SourceKind, DataCategory, Mapping};
use crate::application::ports::Command;
use crate::domain::filters::FilterNode;
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use chrono::{DateTime, Utc, TimeZone, Datelike, Timelike};
use chrono_tz::Tz;

const DEFAULT_YEARS_ADJUSTMENT: i32 = 20;

pub struct CassandraQueryBuilder {
    table_mappings: HashMap<String, String>,
    keyspace: String,
}

impl CassandraQueryBuilder {
    pub fn new(table_mappings: HashMap<String, String>, keyspace: Option<String>) -> Self {
        Self {
            table_mappings,
            keyspace: keyspace.unwrap_or_else(|| "ts".to_string()),
        }
    }

    pub fn build_queries(&self, command: &Command) -> Result<Vec<ExecutableQuery>> {
        let mappings = &command.mappings;
        if mappings.is_empty() {
            return Err(anyhow!("cannot build Cassandra query without mappings"));
        }

        let mut queries = Vec::new();
        for mapping in mappings {
            if mapping.source != SourceKind::Cassandra {
                continue;
            }

            let table = self.resolve_table(mapping)
                .ok_or_else(|| anyhow!("no Cassandra table mapping for {:?}", mapping.data_category))?;

            let (statement, arguments) = self.build_statement(&table, mapping, command)?;

            let mut parameters = HashMap::new();
            parameters.insert("projection_columns".to_string(), serde_json::to_value(self.cassandra_projection_columns(&command.columns)).unwrap());

            queries.push(ExecutableQuery {
                id: mapping.id,
                data_category: self.data_category_for_query(command.data_category, mapping),
                source: SourceKind::Cassandra,
                filters: command.filters.clone(),
                index_range: command.index_range.clone(),
                statement,
                parameters,
                arguments,
            });
        }

        Ok(queries)
    }

    fn data_category_for_query(&self, _command_category: DataCategory, mapping: &Mapping) -> DataCategory {
        mapping.data_category // Assuming it's always set correctly from DB
    }

    fn resolve_table(&self, mapping: &Mapping) -> Option<String> {
        let id = mapping.cassandra_id.as_ref()?.trim().to_lowercase();
        for (key, table) in &self.table_mappings {
            if id.contains(&key.to_lowercase()) && !table.trim().is_empty() {
                return Some(self.qualified_table(table));
            }
        }
        None
    }

    fn qualified_table(&self, table: &str) -> String {
        let table = table.trim();
        if table.contains('.') || self.keyspace.is_empty() {
            table.to_string()
        } else {
            format!("{}.{}", self.keyspace, table)
        }
    }

    fn build_statement(&self, table: &str, mapping: &Mapping, command: &Command) -> Result<(String, Vec<serde_json::Value>)> {
        let cassandra_id = mapping.cassandra_id.as_ref()
            .ok_or_else(|| anyhow!("mapping {} has no Cassandra ID", mapping.id))?;

        let mut quote_indices = command.quote_indices.clone();
        let mut force_no_rows = false;
        if quote_indices.is_empty() {
            quote_indices.push(1);
            force_no_rows = true;
        }

        let mut arguments = vec![
            serde_json::Value::String(cassandra_id.clone()),
            serde_json::to_value(&quote_indices).unwrap(),
        ];
        
        let mut where_clauses = vec![
            "ts_id = ?".to_string(),
            "quote_index IN ?".to_string(),
        ];
        
        let columns = "ts_id, qte_y, qte_m, qte_d, quote_index, publish_time, del_y, del_m, del_d, del_h, del_min, del_offset, value";

        // Filter Logic Stub (similar to Go's buildDeliveryFilters)
        let timezone = "Europe/Zurich"; // Should come from mapping logic
        let (delivery_cql, mut delivery_arguments, no_rows) = self.build_delivery_filters(&command.filters.nodes, timezone, quote_indices[0])?;

        if force_no_rows {
            where_clauses.push("(del_y, del_m, del_d, del_h) = (?, ?, ?, ?)".to_string());
            arguments.extend(vec![
                serde_json::Value::Number(1.into()),
                serde_json::Value::Number(1.into()),
                serde_json::Value::Number(1.into()),
                serde_json::Value::Number(0.into()),
            ]);
        } else if !no_rows && !delivery_cql.is_empty() {
            where_clauses.push(delivery_cql);
            arguments.append(&mut delivery_arguments);
        }

        let statement = format!(
            "SELECT {} FROM {} WHERE {}",
            columns,
            table,
            where_clauses.join(" AND ")
        );

        Ok((statement, arguments))
    }

    fn cassandra_projection_columns(&self, columns: &[String]) -> Vec<String> {
        if !columns.is_empty() {
            return columns.to_vec();
        }
        vec![
            "Identifier".to_string(),
            "ReferenceTime".to_string(),
            "DeliveryStart".to_string(),
            "DeliveryEnd".to_string(),
            "Value".to_string(),
            "LegacyDeliveryBucketNumber".to_string(),
        ]
    }

    fn build_delivery_filters(&self, nodes: &[FilterNode], timezone: &str, quote_index: i32) -> Result<(String, Vec<serde_json::Value>, bool)> {
        if nodes.is_empty() {
            return Ok((String::new(), vec![], false));
        }

        let tz: Tz = match timezone.to_uppercase().as_str() {
            "" | "CET" => chrono_tz::Europe::Zurich,
            "UTC" => chrono_tz::UTC,
            _ => timezone.parse().map_err(|_| anyhow!("invalid Cassandra timezone {}", timezone))?,
        };

        let mut lower: Option<DateTime<Utc>> = None;
        let mut lower_inc = false;
        let mut upper: Option<DateTime<Utc>> = None;
        let mut upper_inc = false;
        let mut found = false;

        // Simplified Delivery Start/End bounds tracker
        for node in nodes {
            if let FilterNode::Comparison(f) = node {
                let is_delivery = f.field.eq_ignore_ascii_case("DeliveryStart") || f.field.eq_ignore_ascii_case("DeliveryEnd");
                if !is_delivery {
                    continue;
                }
                found = true;

                if let Ok(dt) = DateTime::parse_from_rfc3339(&f.value.raw) {
                    let local = dt.with_timezone(&tz);
                    // Extract local Hour
                    let mut local_utc_equiv = Utc.with_ymd_and_hms(local.year(), local.month(), local.day(), local.hour(), 0, 0).unwrap();
                    
                    if f.field.eq_ignore_ascii_case("DeliveryEnd") {
                        local_utc_equiv -= chrono::Duration::hours(1);
                    }

                    match f.operator.as_str() {
                        "=" => {
                            lower = Some(local_utc_equiv); lower_inc = true;
                            upper = Some(local_utc_equiv); upper_inc = true;
                        }
                        ">=" => {
                            if lower.is_none() || local_utc_equiv > lower.unwrap() {
                                lower = Some(local_utc_equiv); lower_inc = true;
                            }
                        }
                        ">" => {
                            if lower.is_none() || local_utc_equiv > lower.unwrap() {
                                lower = Some(local_utc_equiv); lower_inc = false;
                            }
                        }
                        "<=" => {
                            if upper.is_none() || local_utc_equiv < upper.unwrap() {
                                upper = Some(local_utc_equiv); upper_inc = true;
                            }
                        }
                        "<" => {
                            if upper.is_none() || local_utc_equiv < upper.unwrap() {
                                upper = Some(local_utc_equiv); upper_inc = false;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        // RelativeDeliveryPeriod tracker
        let ref_date = Utc.with_ymd_and_hms(
            quote_index / 10000, 
            ((quote_index / 100) % 100) as u32, 
            (quote_index % 100) as u32, 
            0, 0, 0
        ).unwrap();
        for node in nodes {
            if let FilterNode::Comparison(f) = node {
                if f.field.eq_ignore_ascii_case("RelativeDeliveryPeriod") {
                    found = true;
                    if let Ok(hours) = f.value.raw.parse::<i64>() {
                        let local_utc_equiv = ref_date + chrono::Duration::hours(hours);
                        match f.operator.as_str() {
                            "=" => {
                                lower = Some(local_utc_equiv); lower_inc = true;
                                upper = Some(local_utc_equiv); upper_inc = true;
                            }
                            ">=" => {
                                if lower.is_none() || local_utc_equiv > lower.unwrap() {
                                    lower = Some(local_utc_equiv); lower_inc = true;
                                }
                            }
                            ">" => {
                                if lower.is_none() || local_utc_equiv > lower.unwrap() {
                                    lower = Some(local_utc_equiv); lower_inc = false;
                                }
                            }
                            "<=" => {
                                if upper.is_none() || local_utc_equiv < upper.unwrap() {
                                    upper = Some(local_utc_equiv); upper_inc = true;
                                }
                            }
                            "<" => {
                                if upper.is_none() || local_utc_equiv < upper.unwrap() {
                                    upper = Some(local_utc_equiv); upper_inc = false;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        if !found {
            return Ok((String::new(), vec![], false));
        }

        // Check empty window
        if let (Some(l), Some(u)) = (lower, upper) {
            if l > u || (l == u && (!lower_inc || !upper_inc)) {
                return Ok((String::new(), vec![], true)); // Empty
            }
        }

        let mut clauses = Vec::new();
        let mut args = Vec::new();

        if let Some(l) = lower {
            let op = if lower_inc { ">=" } else { ">" };
            clauses.push(format!("(del_y, del_m, del_d, del_h) {} (?, ?, ?, ?)", op));
            args.extend(vec![
                serde_json::Value::Number((l.year() as i16).into()),
                serde_json::Value::Number((l.month() as i8).into()),
                serde_json::Value::Number((l.day() as i8).into()),
                serde_json::Value::Number((l.hour() as i8).into()),
            ]);
        }

        if let Some(u) = upper {
            let op = if upper_inc { "<=" } else { "<" };
            clauses.push(format!("(del_y, del_m, del_d, del_h) {} (?, ?, ?, ?)", op));
            args.extend(vec![
                serde_json::Value::Number((u.year() as i16).into()),
                serde_json::Value::Number((u.month() as i8).into()),
                serde_json::Value::Number((u.day() as i8).into()),
                serde_json::Value::Number((u.hour() as i8).into()),
            ]);
        }

        Ok((clauses.join(" AND "), args, false))
    }
}
