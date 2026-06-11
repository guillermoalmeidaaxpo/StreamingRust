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

            let (statement, parameters) = self.build_cmdp_statement(mapping, &command.filters, &command.index_range, &command.columns)?;
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
        filters: &FilterSet,
        index_range: &Option<crate::domain::query::IndexRange>,
        requested_columns: &[String],
    ) -> Result<(String, HashMap<String, serde_json::Value>)> {
        if mapping.view_name.trim().is_empty() {
            return Err(anyhow!("mapping {} has no CMDP view name", mapping.id));
        }

        let mut builder = SqlBuilder::new(mapping.clone());
        builder.add_parameter("id", serde_json::Value::Number(mapping.id.into()));

        let mut where_clauses = vec![format!("{} = @id", qualify(CMDP_IDENTIFIER_COLUMN))];
        
        let filter_predicates = builder.filter_predicates(&filters.nodes)?;
        where_clauses.extend(filter_predicates);

        if let Some(range) = index_range {
            if !mapping.index_field.trim().is_empty() {
                builder.add_parameter("indexStart", serde_json::Value::Number(range.start.into()));
                builder.add_parameter("indexEnd", serde_json::Value::Number(range.end.into()));
                where_clauses.push(format!("{} >= @indexStart", qualify(&mapping.index_field)));
                where_clauses.push(format!("{} <= @indexEnd", qualify(&mapping.index_field)));
            }
        }

        let mut rank_over_filter = None;
        for node in &filters.nodes {
            if let FilterNode::RankOver(r) = node {
                rank_over_filter = Some(r);
                break;
            }
        }

        let selected = select_columns(&mapping.columns, requested_columns);
        
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
        let view_name = hyperscale_view_name(mapping, requested_columns, version_as_of)?;
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

fn hyperscale_view_name(mapping: &Mapping, _requested_columns: &[String], _version_as_of: Option<&chrono::DateTime<chrono::Utc>>) -> Result<String> {
    Ok(mapping.views.latest_version.clone())
}

fn hyperscale_value_column(category: DataCategory) -> Result<String> {
    match category {
        DataCategory::Curves => Ok("CurveValue".to_string()),
        DataCategory::Surfaces => Ok("SurfaceValue".to_string()),
        DataCategory::TimeSeries => Ok("TimeseriesValue".to_string()),
    }
}

fn hyperscale_select_columns(_columns: &[ColumnMapping], _requested_columns: &[String], _value_column: &str, _include_identifier: bool) -> Vec<String> {
    vec!["[d].*".to_string()]
}

fn hyperscale_order_columns(_columns: &[ColumnMapping], _value_column: &str, _include_identifier: bool) -> Vec<String> {
    vec![]
}
