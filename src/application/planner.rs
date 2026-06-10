use async_trait::async_trait;
use std::sync::Arc;
use crate::domain::{Request, SourceKind, Mapping, Identifier, DataCategory, ExecutableQuery};
use super::ports::{Planner, Plan, PlanStep, RequestContext, MappingResolver, FilterParser, Command};
use super::strategy::StrategySelector;
use super::quote_index::QuoteIndexGenerator;
use anyhow::Result;

pub struct DefaultPlanner {
    resolver: Arc<dyn MappingResolver>,
    parser: Arc<dyn FilterParser>,
}

impl DefaultPlanner {
    pub fn new(resolver: Arc<dyn MappingResolver>, parser: Arc<dyn FilterParser>) -> Self {
        Self { resolver, parser }
    }

    async fn split_hybrid_command(&self, _ctx: &RequestContext, command: Command) -> Result<Vec<Command>> {
        // Only split if it's Cassandra and no aggregations/shape
        if command.source != SourceKind::Cassandra || command.has_aggregations || command.has_shape {
            return Ok(vec![command]);
        }

        let watermark = self.resolver.get_watermark(&command.mappings).await?;
        
        // Find ReferenceTime filter
        let ref_time_filter = command.filters.nodes.iter().find(|node| {
            if let crate::domain::FilterNode::Comparison(f) = node {
                f.field.to_lowercase() == "referencetime"
            } else {
                false
            }
        });

        if let Some(crate::domain::FilterNode::Comparison(f)) = ref_time_filter {
            // Simplified logic: if it's an equality filter, route based on watermark
            if f.operator == "=" {
                if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&f.value.raw) {
                    let dt_utc = dt.with_timezone(&chrono::Utc);
                    if dt_utc < watermark {
                        return Ok(vec![self.with_source(command, SourceKind::Cassandra)]);
                    } else {
                        return Ok(vec![self.with_source(command, SourceKind::Cmdp)]);
                    }
                }
            }
        }

        // Default: no split or complex range not handled yet
        Ok(vec![command])
    }

    fn with_source(&self, mut command: Command, source: SourceKind) -> Command {
        command.source = source;
        for mapping in &mut command.mappings {
            mapping.source = source;
        }
        command
    }

    fn build_query(&self, command: &Command, mapping: &Mapping) -> ExecutableQuery {
        ExecutableQuery {
            id: mapping.id,
            data_category: command.data_category,
            source: command.source,
            filters: command.filters.clone(),
            index_range: None,
            statement: "SELECT ...".to_string(), // In real app, this would be built by a QueryBuilder
            parameters: Default::default(),
            arguments: vec![],
        }
    }
}

#[async_trait]
impl Planner for DefaultPlanner {
    async fn build_plan(&self, ctx: RequestContext, requests: Vec<Request>) -> Result<Plan> {
        let mut steps = Vec::new();

        for request in requests {
            let mappings = self.resolver.resolve_mappings(&request.ids, ctx.data_category, &ctx.stage).await?;
            
            let parsed_filters = if let Some(f) = &request.filters {
                self.parser.parse(&f.expressions, &f.filter_time_zone).await?
            } else {
                crate::domain::FilterSet { expressions: vec![], nodes: vec![] }
            };

            for mapping in mappings {
                let has_aggregations = request.transformations.as_ref().map(|t| t.keys.is_some()).unwrap_or(false);
                let has_shape = request.filters.as_ref().map(|f| f.shape.is_some()).unwrap_or(false);
                let source = StrategySelector::select_source(&mapping, has_aggregations, has_shape, ctx.is_mesap_endpoint);

                let mut command = Command {
                    ids: vec![mapping.id],
                    data_category: ctx.data_category,
                    columns: request.columns.clone().unwrap_or_default(),
                    version_as_of: request.version_as_of,
                    include_deleted: request.include_deleted.unwrap_or(false),
                    include_identifier: true,
                    include_offset: request.transformations.as_ref().and_then(|t| t.offset).unwrap_or(false),
                    filter_time_zone: request.filters.as_ref().and_then(|f| f.filter_time_zone.clone()).unwrap_or_default(),
                    target_time_zone: request.transformations.as_ref().and_then(|t| t.target_time_zone.clone()).unwrap_or_default(),
                    has_aggregations,
                    has_shape,
                    filters: parsed_filters.clone(),
                    mappings: vec![mapping.clone()],
                    source,
                    quote_indices: vec![],
                    index_range: None,
                };

                // 4. Generate Quote Indices
                let limits = self.resolver.get_filter_limits(&[mapping.id], ctx.data_category).await?;
                command.quote_indices = match source {
                    SourceKind::Cassandra => QuoteIndexGenerator::generate_cassandra_indices(&command.filters, &limits, &command.filter_time_zone),
                    _ => QuoteIndexGenerator::generate_cmdp_indices(&command.filters, &limits),
                };

                let hybrid_commands = self.split_hybrid_command(&ctx, command).await?;
                for h_command in hybrid_commands {
                    let query = self.build_query(&h_command, &mapping);
                    steps.push(PlanStep {
                        command: h_command,
                        query,
                    });
                }
            }
        }

        Ok(Plan { steps })
    }
}
