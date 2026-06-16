use crate::domain::{ExecutableQuery, SourceKind, DataCategory, Mapping};
use crate::application::ports::Command;
use crate::domain::filters::FilterNode;
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use chrono::{Datelike, Timelike};
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
            parameters.insert("include_offset".to_string(), serde_json::Value::Bool(command.include_offset));
            let timezone = if mapping.timezone.is_empty() {
                crate::application::strategy::StrategySelector::get_cassandra_timezone(mapping.id)
            } else {
                mapping.timezone.clone()
            };
            parameters.insert("timezone".to_string(), serde_json::Value::String(timezone));

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

        // Resolve timezone dynamically
        let timezone = crate::application::strategy::StrategySelector::get_cassandra_timezone(mapping.id);
        let (delivery_cql, mut delivery_arguments, no_rows) = self.build_delivery_filters(&command.filters.nodes, &timezone, quote_indices[0])?;

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
        let tz: Tz = match timezone.to_uppercase().as_str() {
            "" | "CET" => chrono_tz::Europe::Zurich,
            "UTC" => chrono_tz::UTC,
            _ => timezone.parse().map_err(|_| anyhow!("invalid Cassandra timezone {}", timezone))?,
        };

        let delivery_win = build_delivery_window(nodes, &tz, timezone)?;
        let rdp_win = build_rdp_window(nodes, quote_index)?;

        if delivery_win.is_none() && rdp_win.is_none() {
            return Ok((String::new(), vec![], false));
        }

        let unified = match (delivery_win, rdp_win) {
            (None, Some(r)) => Some(r),
            (Some(d), None) => Some(d),
            (Some(d), Some(r)) => intersect(d, r),
            (None, None) => unreachable!(),
        };

        let unified = match unified {
            Some(u) if !u.is_empty() => u,
            _ => return Ok((String::new(), vec![], true)), // empty range, force no rows
        };

        if unified.is_point() {
            let l = unified.lower.unwrap();
            let cql = "(del_y, del_m, del_d, del_h) = (?, ?, ?, ?)".to_string();
            let args = vec![
                serde_json::Value::Number((l.year() as i16).into()),
                serde_json::Value::Number((l.month() as i8).into()),
                serde_json::Value::Number((l.day() as i8).into()),
                serde_json::Value::Number((l.hour() as i8).into()),
            ];
            return Ok((cql, args, false));
        }

        let mut clauses = Vec::new();
        let mut args = Vec::new();

        if let Some(l) = unified.lower {
            let op = if unified.lower_inclusive { ">=" } else { ">" };
            clauses.push(format!("(del_y, del_m, del_d, del_h) {} (?, ?, ?, ?)", op));
            args.extend(vec![
                serde_json::Value::Number((l.year() as i16).into()),
                serde_json::Value::Number((l.month() as i8).into()),
                serde_json::Value::Number((l.day() as i8).into()),
                serde_json::Value::Number((l.hour() as i8).into()),
            ]);
        }

        if let Some(u) = unified.upper {
            let op = if unified.upper_inclusive { "<=" } else { "<" };
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

#[derive(Debug, Copy, Clone)]
struct LocalHourWindow {
    lower: Option<chrono::NaiveDateTime>,
    lower_inclusive: bool,
    upper: Option<chrono::NaiveDateTime>,
    upper_inclusive: bool,
}

impl LocalHourWindow {
    fn is_empty(&self) -> bool {
        if let (Some(l), Some(u)) = (self.lower, self.upper) {
            l > u || (l == u && !(self.lower_inclusive && self.upper_inclusive))
        } else {
            false
        }
    }

    fn is_point(&self) -> bool {
        if let (Some(l), Some(u)) = (self.lower, self.upper) {
            l == u && self.lower_inclusive && self.upper_inclusive
        } else {
            false
        }
    }
}

fn tighten_lower(lo: &mut Option<chrono::NaiveDateTime>, lo_inc: &mut bool, cand: chrono::NaiveDateTime, cand_inc: bool) {
    match lo {
        None => {
            *lo = Some(cand);
            *lo_inc = cand_inc;
        }
        Some(current) => {
            if cand > *current {
                *lo = Some(cand);
                *lo_inc = cand_inc;
            } else if cand == *current {
                *lo_inc = *lo_inc && cand_inc;
            }
        }
    }
}

fn tighten_upper(up: &mut Option<chrono::NaiveDateTime>, up_inc: &mut bool, cand: chrono::NaiveDateTime, cand_inc: bool) {
    match up {
        None => {
            *up = Some(cand);
            *up_inc = cand_inc;
        }
        Some(current) => {
            if cand < *current {
                *up = Some(cand);
                *up_inc = cand_inc;
            } else if cand == *current {
                *up_inc = *up_inc && cand_inc;
            }
        }
    }
}

fn add_years(dt: chrono::NaiveDateTime, years: i32) -> Option<chrono::NaiveDateTime> {
    let target_year = dt.year() + years;
    if let Some(new_dt) = dt.with_year(target_year) {
        Some(new_dt)
    } else {
        // If it failed (likely leap year Feb 29), set day to 28 first
        dt.with_day(28)?.with_year(target_year)
    }
}

fn build_delivery_window(nodes: &[FilterNode], tz: &Tz, timezone_str: &str) -> Result<Option<LocalHourWindow>> {
    let mut lower: Option<chrono::NaiveDateTime> = None;
    let mut lower_inc = false;
    let mut upper: Option<chrono::NaiveDateTime> = None;
    let mut upper_inc = false;
    let mut saw_any = false;

    for node in nodes {
        if let FilterNode::Comparison(f) = node {
            let is_delivery = f.field.eq_ignore_ascii_case("DeliveryStart") 
                || f.field.eq_ignore_ascii_case("DeliveryEnd");
            if !is_delivery {
                continue;
            }
            saw_any = true;

            let utc_dt = crate::domain::timeexpr::point_in_time::parse_point_in_time(&f.value.raw, timezone_str)?;
            let local_dt = utc_dt.with_timezone(tz);
            let mut local = local_dt.naive_local();

            if f.field.eq_ignore_ascii_case("DeliveryEnd") {
                local = local - chrono::Duration::hours(1);
            }

            match f.operator.as_str() {
                "=" => {
                    tighten_lower(&mut lower, &mut lower_inc, local, true);
                    tighten_upper(&mut upper, &mut upper_inc, local, true);
                }
                ">=" => {
                    tighten_lower(&mut lower, &mut lower_inc, local, true);
                }
                ">" => {
                    tighten_lower(&mut lower, &mut lower_inc, local, false);
                }
                "<=" => {
                    tighten_upper(&mut upper, &mut upper_inc, local, true);
                }
                "<" => {
                    tighten_upper(&mut upper, &mut upper_inc, local, false);
                }
                _ => {}
            }
        }
    }

    if !saw_any {
        return Ok(None);
    }

    if lower.is_none() && upper.is_some() {
        let up_val = upper.unwrap();
        let low_val = add_years(up_val, -DEFAULT_YEARS_ADJUSTMENT)
            .ok_or_else(|| anyhow!("invalid date subtraction"))?;
        lower = Some(low_val);
        lower_inc = true;
    } else if upper.is_none() && lower.is_some() {
        let low_val = lower.unwrap();
        let up_val = add_years(low_val, DEFAULT_YEARS_ADJUSTMENT)
            .ok_or_else(|| anyhow!("invalid date addition"))?;
        upper = Some(up_val);
        upper_inc = true;
    }

    Ok(Some(LocalHourWindow {
        lower,
        lower_inclusive: lower_inc,
        upper,
        upper_inclusive: upper_inc,
    }))
}

fn build_rdp_window(nodes: &[FilterNode], quote_index: i32) -> Result<Option<LocalHourWindow>> {
    let mut rdp_nodes = Vec::new();
    for node in nodes {
        if let FilterNode::Comparison(f) = node {
            if f.field.eq_ignore_ascii_case("RelativeDeliveryPeriod") {
                rdp_nodes.push(f);
            }
        }
    }

    if rdp_nodes.is_empty() {
        return Ok(None);
    }

    let qy = quote_index / 10000;
    let qm = (quote_index / 100) % 100;
    let qd = quote_index % 100;

    let ref_local = chrono::NaiveDate::from_ymd_opt(qy, qm as u32, qd as u32)
        .ok_or_else(|| anyhow!("invalid quote index date"))?
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| anyhow!("invalid midnight time"))?;

    let mut lower = None;
    let mut lower_inc = false;
    let mut upper = None;
    let mut upper_inc = false;

    for f in rdp_nodes {
        let hours = f.value.raw.parse::<i64>()
            .map_err(|_| anyhow!("Invalid RelativeDeliveryPeriod value '{}'. Must be integer hours.", f.value.raw))?;
        
        let local = ref_local + chrono::Duration::hours(hours);

        match f.operator.as_str() {
            "=" => {
                tighten_lower(&mut lower, &mut lower_inc, local, true);
                tighten_upper(&mut upper, &mut upper_inc, local, true);
            }
            ">=" => {
                tighten_lower(&mut lower, &mut lower_inc, local, true);
            }
            ">" => {
                tighten_lower(&mut lower, &mut lower_inc, local, false);
            }
            "<=" => {
                tighten_upper(&mut upper, &mut upper_inc, local, true);
            }
            "<" => {
                tighten_upper(&mut upper, &mut upper_inc, local, false);
            }
            _ => {
                return Err(anyhow!("Unsupported comparer '{}'. Use one of: =, <, <=, >, >=", f.operator));
            }
        }
    }

    Ok(Some(LocalHourWindow {
        lower,
        lower_inclusive: lower_inc,
        upper,
        upper_inclusive: upper_inc,
    }))
}

fn intersect(a: LocalHourWindow, b: LocalHourWindow) -> Option<LocalHourWindow> {
    let mut lower = None;
    let mut lower_inc = false;
    let mut upper = None;
    let mut upper_inc = false;

    if let Some(l) = a.lower {
        tighten_lower(&mut lower, &mut lower_inc, l, a.lower_inclusive);
    }
    if let Some(l) = b.lower {
        tighten_lower(&mut lower, &mut lower_inc, l, b.lower_inclusive);
    }

    if let Some(u) = a.upper {
        tighten_upper(&mut upper, &mut upper_inc, u, a.upper_inclusive);
    }
    if let Some(u) = b.upper {
        tighten_upper(&mut upper, &mut upper_inc, u, b.upper_inclusive);
    }

    let merged = LocalHourWindow {
        lower,
        lower_inclusive: lower_inc,
        upper,
        upper_inclusive: upper_inc,
    };

    if merged.is_empty() {
        None
    } else {
        Some(merged)
    }
}

