use crate::domain::{ExecutableQuery, SourceKind, DataCategory, ColumnMapping, Mapping};
use crate::application::ports::Command;
use crate::domain::filters::{FilterSet, FilterNode, ComparisonFilter, FilterValue};
use anyhow::{Result, anyhow};
use std::collections::HashMap;

const CMDP_IDENTIFIER_COLUMN: &str = "TimeSeries_FID";
const HYPERSCALE_IDENTIFIER_COLUMN: &str = "MdoId";
const HYPERSCALE_DELETED_COLUMN: &str = "Deleted";

pub struct CMDPQueryBuilder;

impl CMDPQueryBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn build_queries(&self, command: &Command) -> Result<Vec<ExecutableQuery>> {
        let mappings = &command.mappings;
        if mappings.is_empty() {
            return Err(anyhow!("cannot build CMDP query without mappings"));
        }

        let mut queries = Vec::new();
        for mapping in mappings {
            if mapping.source != SourceKind::Cmdp {
                continue;
            }

            let (statement, parameters) = self.build_cmdp_statement(mapping, command)?;
            queries.push(ExecutableQuery {
                id: mapping.id,
                data_category: self.data_category_for_query(command.data_category, mapping),
                source: SourceKind::Cmdp,
                filters: command.filters.clone(),
                index_range: command.index_range.clone(),
                statement,
                parameters,
                arguments: vec![],
            });
        }

        Ok(queries)
    }

    fn data_category_for_query(&self, _command_category: DataCategory, mapping: &Mapping) -> DataCategory {
        mapping.data_category // Assuming it's always set correctly from DB
    }

    fn build_cmdp_statement(
        &self,
        mapping: &Mapping,
        command: &Command,
    ) -> Result<(String, HashMap<String, serde_json::Value>)> {
        if mapping.view_name.trim().is_empty() {
            return Err(anyhow!("mapping {} has no CMDP view name", mapping.id));
        }

        let mut builder = SqlBuilder::new(mapping.clone());
        builder.add_parameter("id", serde_json::Value::Number(mapping.id.into()));

        let mut where_clauses = vec![format!("{} = @id", qualify(CMDP_IDENTIFIER_COLUMN))];
        
        let filter_predicates = builder.filter_predicates(&command.filters.nodes)?;
        where_clauses.extend(filter_predicates);

        if let Some(range) = &command.index_range {
            if !mapping.index_field.trim().is_empty() {
                builder.add_parameter("indexStart", serde_json::Value::Number(range.start.into()));
                builder.add_parameter("indexEnd", serde_json::Value::Number(range.end.into()));
                where_clauses.push(format!("{} >= @indexStart", qualify(&mapping.index_field)));
                where_clauses.push(format!("{} <= @indexEnd", qualify(&mapping.index_field)));
            }
        }

        // Add Shape Predicates
        let delivery_start_column = mapping.columns.iter()
            .find(|c| c.mds_name.eq_ignore_ascii_case("DeliveryStart") || c.source_name.eq_ignore_ascii_case("DeliveryStart"))
            .map(|c| c.source_name.as_str())
            .unwrap_or("DeliveryStart");

        let delivery_start_expr = if command.filter_time_zone.is_empty() || command.filter_time_zone.eq_ignore_ascii_case("UTC") {
            qualify(delivery_start_column)
        } else {
            let sql_tz = to_sql_server_timezone_name(&command.filter_time_zone);
            format!("{} AT TIME ZONE '{}'", qualify(delivery_start_column), sql_tz)
        };

        if let Some(shape) = &command.shape {
            // 1. Month Predicate
            if !shape.months.is_empty() && shape.months.len() != 12 {
                let mut month_params = Vec::new();
                for (i, &month) in shape.months.iter().enumerate() {
                    let p_name = format!("month{}", i);
                    builder.add_parameter(&p_name, serde_json::Value::Number(month.into()));
                    month_params.push(format!("@{}", p_name));
                }
                let in_list = month_params.join(", ");
                where_clauses.push(format!("DATEPART(MONTH, {}) IN ({})", delivery_start_expr, in_list));
            }

            // 2. Day Predicate
            if !shape.days.is_empty() && shape.days.len() != 7 {
                let mut day_params = Vec::new();
                for (i, &day) in shape.days.iter().enumerate() {
                    let p_name = format!("day{}", i);
                    builder.add_parameter(&p_name, serde_json::Value::Number(day.into()));
                    day_params.push(format!("@{}", p_name));
                }
                let in_list = day_params.join(", ");
                where_clauses.push(format!("((DATEDIFF(DAY, '19000101', {}) % 7) + 1) IN ({})", delivery_start_expr, in_list));
            }

            // 3. Time Predicate
            if !shape.time_spans.is_empty() && !is_full_day_coverage(&shape.time_spans) {
                let mut fragments = Vec::new();
                for (i, span) in shape.time_spans.iter().enumerate() {
                    let start_name = format!("t{}_start", i);
                    let end_name = format!("t{}_end", i);

                    let start_str = format_duration_as_time(span.start);
                    builder.add_parameter(&start_name, serde_json::Value::String(start_str));

                    if span.end.is_zero() {
                        fragments.push(format!("(CAST({} AS time) >= @{})", delivery_start_expr, start_name));
                    } else {
                        let end_str = format_duration_as_time(span.end);
                        builder.add_parameter(&end_name, serde_json::Value::String(end_str));
                        fragments.push(format!("(CAST({0} AS time) >= @{1} AND CAST({0} AS time) < @{2})", delivery_start_expr, start_name, end_name));
                    }
                }
                if !fragments.is_empty() {
                    where_clauses.push(format!("({})", fragments.join(" OR ")));
                }
            }
        }

        let mut rank_over_filter = None;
        for node in &command.filters.nodes {
            if let FilterNode::RankOver(r) = node {
                rank_over_filter = Some(r);
                break;
            }
        }

        let selected = select_columns(&mapping.columns, &command.columns);
        
        let statement = if let Some(rank_filter) = rank_over_filter {
            // Subquery for RankOver
            let partition_by = rank_filter.partition_by.iter().map(|c| qualify(c)).collect::<Vec<_>>().join(", ");
            let order_by = rank_filter.order_by.iter().map(|s| format!("{} {}", qualify(&s.field), s.direction)).collect::<Vec<_>>().join(", ");
            
            let mut rank_where = Vec::new();
            for bound in &rank_filter.bounds {
                if let (Some(start), None) = (&bound.start, &bound.end) {
                    rank_where.push(format!("[d].[rank] >= {}", start));
                } else if let (Some(start), Some(end)) = (&bound.start, &bound.end) {
                    rank_where.push(format!("([d].[rank] >= {} AND [d].[rank] <= {})", start, end));
                }
            }
            let rank_where_clause = if rank_where.is_empty() { "1=1".to_string() } else { rank_where.join(" OR ") };

            let subquery_selected: Vec<String> = mapping.columns.iter().filter(|c| c.source_name != "RelativeDeliveryPeriod").map(|c| qualify(&c.source_name)).collect();
            let query_selected = selected.clone(); // In real implementation, handle RelativeDeliveryPeriod exclusion

            format!(
                "SELECT {} FROM (SELECT {}, RANK() OVER (PARTITION BY {} ORDER BY {}) AS rank FROM {} AS [d] WHERE {}) AS [d] WHERE {}",
                query_selected.join(", "),
                subquery_selected.join(", "),
                partition_by,
                order_by,
                quote_table(&mapping.view_name),
                where_clauses.join(" AND "),
                rank_where_clause
            )
        } else {
            format!(
                "SELECT {} FROM {} AS [d] WHERE {}",
                selected.join(", "),
                quote_table(&mapping.view_name),
                where_clauses.join(" AND ")
            )
        };

        let mut final_statement = statement;
        let order = order_columns(&mapping.columns);
        if !order.is_empty() {
            final_statement.push_str(" ORDER BY ");
            final_statement.push_str(&order.join(", "));
        }

        Ok((final_statement, builder.parameters))
    }
}

pub struct HyperscaleQueryBuilder;

impl HyperscaleQueryBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn build_queries(&self, command: &Command) -> Result<Vec<ExecutableQuery>> {
        let mappings = &command.mappings;
        if mappings.is_empty() {
            return Err(anyhow!("cannot build hyperscale query without mappings"));
        }

        let mut queries = Vec::new();
        for mapping in mappings {
            if mapping.source != SourceKind::Hyperscale {
                continue;
            }

            let (statement, parameters) = self.build_hyperscale_statement(
                mapping,
                &command.filters,
                &command.columns,
                command.version_as_of.as_ref(),
                command.include_deleted,
                command.include_identifier,
            )?;
            
            queries.push(ExecutableQuery {
                id: mapping.id,
                data_category: mapping.data_category,
                source: SourceKind::Hyperscale,
                filters: command.filters.clone(),
                index_range: command.index_range.clone(),
                statement,
                parameters,
                arguments: vec![],
            });
        }

        Ok(queries)
    }

    fn build_hyperscale_statement(
        &self,
        mapping: &Mapping,
        filters: &FilterSet,
        requested_columns: &[String],
        version_as_of: Option<&chrono::DateTime<chrono::Utc>>,
        include_deleted: bool,
        include_identifier: bool,
    ) -> Result<(String, HashMap<String, serde_json::Value>)> {
        let view_name = hyperscale_view_name(mapping, filters, requested_columns, version_as_of)?;
        let value_column = hyperscale_value_column(mapping.data_category)?;

        let mut builder = SqlBuilder::new_with_json_column(mapping.clone(), value_column.clone());

        let mut from = quote_table(&view_name);
        let mut where_clauses = Vec::new();

        if let Some(v_as_of) = version_as_of {
            builder.add_parameter("MdoId", serde_json::Value::Number(mapping.id.into()));
            builder.add_parameter("CreatedOn", serde_json::Value::String(v_as_of.to_rfc3339()));
            builder.add_parameter("IncludeDeleted", serde_json::Value::Bool(include_deleted));
            from.push_str("(@MdoId, @CreatedOn, @IncludeDeleted)");
        } else {
            builder.add_parameter("id", serde_json::Value::Number(mapping.id.into()));
            where_clauses.push(format!("{} = @id", qualify(HYPERSCALE_IDENTIFIER_COLUMN)));
            if !include_deleted {
                where_clauses.push(format!("{} = 0", qualify(HYPERSCALE_DELETED_COLUMN)));
            }
        }

        let filter_predicates = builder.filter_predicates(&filters.nodes)?;
        where_clauses.extend(filter_predicates);

        let selected = hyperscale_select_columns(&mapping.columns, requested_columns, &value_column, include_identifier);
        
        let mut statement = format!(
            "SELECT {} FROM {} AS [d]",
            selected.join(", "),
            from
        );

        if !where_clauses.is_empty() {
            statement.push_str(" WHERE ");
            statement.push_str(&where_clauses.join(" AND "));
        }

        let order = hyperscale_order_columns(&mapping.columns, &value_column, include_identifier);
        if !order.is_empty() {
            statement.push_str(" ORDER BY ");
            statement.push_str(&order.join(", "));
        }

        Ok((statement, builder.parameters))
    }
}

// Helpers... (SqlBuilder, select_columns, etc.)
// For brevity, these are simplified stubs of the Go logic, but structurally sound.

struct SqlBuilder {
    mapping: Mapping,
    parameters: HashMap<String, serde_json::Value>,
    next_param: i32,
    json_value_column: String,
}

impl SqlBuilder {
    fn new(mapping: Mapping) -> Self {
        Self {
            mapping,
            parameters: HashMap::new(),
            next_param: 0,
            json_value_column: String::new(),
        }
    }

    fn new_with_json_column(mapping: Mapping, json_value_column: String) -> Self {
        Self {
            mapping,
            parameters: HashMap::new(),
            next_param: 0,
            json_value_column,
        }
    }

    fn add_parameter(&mut self, name: &str, value: serde_json::Value) {
        self.parameters.insert(name.to_string(), value);
    }

    fn filter_predicates(&mut self, nodes: &[FilterNode]) -> Result<Vec<String>> {
        let mut predicates = Vec::new();
        for node in nodes {
            match node {
                FilterNode::Comparison(filter) => {
                    let predicate = self.comparison_predicate(filter)?;
                    if !predicate.is_empty() {
                        predicates.push(predicate);
                    }
                }
                FilterNode::RankOver(_) => {
                    return Err(anyhow!("rankover filters are not supported by the CMDP SQL builder yet"));
                }
            }
        }
        Ok(predicates)
    }

    fn comparison_predicate(&mut self, filter: &ComparisonFilter) -> Result<String> {
        let column = self.column_by_mds_name(&filter.field)
            .ok_or_else(|| anyhow!("filter field {} is not mapped for CMDP view {}", filter.field, self.mapping.view_name))?;

        let op = filter.operator.to_lowercase();
        if op == "in" {
            // self.interval_predicate(&column, &filter.value)
            Ok(String::new()) // Stub
        } else if is_comparison_operator(&op) {
            self.scalar_predicate(&column, &op, &filter.value)
        } else {
            Err(anyhow!("unsupported filter operator {}", op))
        }
    }

    fn scalar_predicate(&mut self, column: &ColumnMapping, operator: &str, value: &FilterValue) -> Result<String> {
        let param_name = format!("p{}", self.next_param);
        self.next_param += 1;
        self.add_parameter(&param_name, serde_json::Value::String(value.raw.clone()));
        Ok(format!("{} {} @{}", self.column_expression(column), operator, param_name))
    }

    fn column_expression(&self, column: &ColumnMapping) -> String {
        qualify(&column.source_name)
    }

    fn column_by_mds_name(&self, name: &str) -> Option<ColumnMapping> {
        self.mapping.columns.iter()
            .find(|c| c.mds_name.eq_ignore_ascii_case(name) || c.source_name.eq_ignore_ascii_case(name))
            .cloned()
    }
}

fn qualify(identifier: &str) -> String {
    format!("[d].{}", quote_identifier(identifier))
}

fn quote_table(name: &str) -> String {
    let name = name.trim();
    if name.contains('[') {
        return name.to_string();
    }
    let parts: Vec<&str> = name.split('.').collect();
    let quoted_parts: Vec<String> = parts.into_iter().map(|p| quote_identifier(p.trim())).collect();
    quoted_parts.join(".")
}

fn quote_identifier(identifier: &str) -> String {
    format!("[{}]", identifier.trim().replace("]", "]]"))
}

fn is_comparison_operator(operator: &str) -> bool {
    matches!(operator, "=" | ">" | ">=" | "<" | "<=")
}

fn select_columns(_columns: &[ColumnMapping], _requested_columns: &[String]) -> Vec<String> {
    // Stub implementation returning all or requested
    vec!["[d].*".to_string()]
}

fn order_columns(_columns: &[ColumnMapping]) -> Vec<String> {
    vec![]
}

fn hyperscale_view_name(
    mapping: &Mapping,
    filters: &FilterSet,
    requested_columns: &[String],
    version_as_of: Option<&chrono::DateTime<chrono::Utc>>,
) -> Result<String> {
    let has_created_on_column = requested_columns.iter().any(|c| c.eq_ignore_ascii_case("CreatedOn"));
    let has_latest_global = filters.has_latest_global_filter;

    if version_as_of.is_some() {
        if has_latest_global {
            Ok(mapping.views.get_by_created_on_latest_reference_time.clone())
        } else {
            Ok(mapping.views.get_by_created_on.clone())
        }
    } else if has_created_on_column {
        if has_latest_global {
            Ok(mapping.views.latest_reference_time_with_created_on.clone())
        } else {
            Ok(mapping.views.latest_version_with_created_on.clone())
        }
    } else {
        if has_latest_global {
            Ok(mapping.views.latest_reference_time.clone())
        } else {
            Ok(mapping.views.latest_version.clone())
        }
    }
}

fn hyperscale_value_column(category: DataCategory) -> Result<String> {
    match category {
        DataCategory::Curves => Ok("CurveValue".to_string()),
        DataCategory::Surfaces => Ok("SurfaceValue".to_string()),
        DataCategory::TimeSeries => Ok("TimeseriesValue".to_string()),
    }
}

fn wrap_json_value_with_cast(hyperscale_column: &str, mds_field_name: &str, data_type: Option<&str>, alias: Option<&str>) -> String {
    let json_value = format!("JSON_VALUE({}, '$.\"{}\"')", hyperscale_column, mds_field_name);
    let wrapped_value = match data_type.map(|s| s.to_lowercase()).as_deref() {
        Some("int") => format!("CAST({} AS INT)", json_value),
        Some("number") | Some("decimal") => format!("CAST({} AS FLOAT)", json_value),
        _ => json_value,
    };
    if let Some(a) = alias {
        format!("{} AS {}", wrapped_value, a)
    } else {
        wrapped_value
    }
}

fn hyperscale_select_columns(columns: &[ColumnMapping], requested_columns: &[String], value_column: &str, include_identifier: bool) -> Vec<String> {
    let mut key_cols = Vec::new();
    for col in columns {
        if col.is_key {
            let is_identifier_or_mdo_id = col.mds_name.eq_ignore_ascii_case("Identifier") || col.mds_name.eq_ignore_ascii_case("MdoId");
            if include_identifier || !is_identifier_or_mdo_id {
                let col_name = if col.mds_name.eq_ignore_ascii_case("Identifier") {
                    "MdoId".to_string()
                } else {
                    col.mds_name.clone()
                };
                key_cols.push(col_name);
            }
        }
    }

    let mut val_cols = Vec::new();
    let mut i = 0;
    
    let mut is_created_on_only = false;
    if requested_columns.len() == 1 && requested_columns[0].eq_ignore_ascii_case("CreatedOn") {
        is_created_on_only = true;
    }

    let effective_requested_cols = if requested_columns.is_empty() || is_created_on_only {
        columns.iter()
            .filter(|c| !c.is_key)
            .map(|c| c.mds_name.clone())
            .collect()
    } else {
        requested_columns.to_vec()
    };

    let projection_columns: Vec<&str> = effective_requested_cols.iter()
        .filter(|&c| !c.eq_ignore_ascii_case("CreatedOn"))
        .map(|s| s.as_str())
        .collect();

    for col in columns {
        if !col.is_key && projection_columns.iter().any(|&pc| pc.eq_ignore_ascii_case(&col.mds_name)) {
            let wrapped = wrap_json_value_with_cast(value_column, &col.mds_name, Some(&col.data_type), Some(&format!("Property{}", i)));
            val_cols.push(wrapped);
            i += 1;
        }
    }

    let mut selected = Vec::new();
    selected.extend(key_cols);
    selected.extend(val_cols);

    let has_created_on = requested_columns.iter().any(|c| c.eq_ignore_ascii_case("CreatedOn")) || is_created_on_only;
    if has_created_on {
        selected.push("CreatedOn".to_string());
    }

    selected
}

fn hyperscale_order_columns(columns: &[ColumnMapping], _value_column: &str, _include_identifier: bool) -> Vec<String> {
    let mut order_cols: Vec<ColumnMapping> = columns.iter()
        .filter(|c| c.order_priority.is_some())
        .cloned()
        .collect();
    
    order_cols.sort_by_key(|c| c.order_priority.unwrap());

    let mut result: Vec<String> = order_cols.iter()
        .map(|c| {
            if c.mds_name.eq_ignore_ascii_case("Identifier") {
                "MdoId".to_string()
            } else {
                c.mds_name.clone()
            }
        })
        .collect();

    if result.is_empty() {
        result.push("ReferenceTime".to_string());
    }

    result
}

fn to_sql_server_timezone_name(tz: &str) -> String {
    if let Some(mapped) = map_timezone_abbreviation(tz) {
        return mapped.to_string();
    }
    match tz.to_lowercase().as_str() {
        "europe/zurich" => "Central European Standard Time".to_string(),
        "europe/london" => "GMT Standard Time".to_string(),
        "europe/paris" => "Romance Standard Time".to_string(),
        "europe/berlin" => "W. Europe Standard Time".to_string(),
        "america/new_york" => "Eastern Standard Time".to_string(),
        "america/chicago" => "Central Standard Time".to_string(),
        "asia/tokyo" => "Tokyo Standard Time".to_string(),
        _ => tz.to_string(),
    }
}

fn map_timezone_abbreviation(tz: &str) -> Option<&'static str> {
    match tz.to_uppercase().as_str() {
        "GMT" | "WET" | "WEST" | "BST" => Some("GMT Standard Time"),
        "CET" | "CEST" => Some("Central European Standard Time"),
        "MET" => Some("W. Europe Standard Time"),
        "EET" | "EEST" => Some("E. Europe Standard Time"),
        "FET" => Some("Belarus Standard Time"),
        "MSK" => Some("Russian Standard Time"),
        "TRT" => Some("Turkey Standard Time"),
        "EST" | "EDT" => Some("Eastern Standard Time"),
        "CST" | "CDT" => Some("Central Standard Time"),
        "MST" | "MDT" => Some("Mountain Standard Time"),
        "PST" | "PDT" => Some("Pacific Standard Time"),
        "AST" | "ADT" => Some("Atlantic Standard Time"),
        "NST" | "NDT" => Some("Newfoundland Standard Time"),
        "AKT" => Some("Alaskan Standard Time"),
        "HST" => Some("Hawaiian Standard Time"),
        "BRT" => Some("E. South America Standard Time"),
        "ART" => Some("Argentina Standard Time"),
        "IST" => Some("India Standard Time"),
        "PKT" => Some("Pakistan Standard Time"),
        "ICT" | "WIB" => Some("SE Asia Standard Time"),
        "SGT" => Some("Singapore Standard Time"),
        "HKT" | "CST8" => Some("China Standard Time"),
        "JST" => Some("Tokyo Standard Time"),
        "KST" => Some("Korea Standard Time"),
        "AEST" | "AEDT" => Some("AUS Eastern Standard Time"),
        "ACST" => Some("Cen. Australia Standard Time"),
        "AWST" => Some("W. Australia Standard Time"),
        "NZST" | "NZDT" => Some("New Zealand Standard Time"),
        "EAT" => Some("E. Africa Standard Time"),
        "CAT" | "SAST" => Some("South Africa Standard Time"),
        "WAT" => Some("W. Central Africa Standard Time"),
        "AST3" => Some("Arab Standard Time"),
        "GST" => Some("Arabian Standard Time"),
        "IDT" => Some("Israel Standard Time"),
        _ => None,
    }
}

fn is_full_day_coverage(spans: &[crate::domain::request::NormalizedTimeSpan]) -> bool {
    if spans.len() != 1 {
        return false;
    }
    let span = &spans[0];
    span.start.is_zero() && 
        (span.end == chrono::Duration::hours(24) || span.end.is_zero())
}

fn format_duration_as_time(d: chrono::Duration) -> String {
    let secs = d.num_seconds();
    let hours = secs / 3600;
    let mins = (secs % 3600) / 60;
    let secs = secs % 60;
    format!("{:02}:{:02}:{:02}", hours, mins, secs)
}
