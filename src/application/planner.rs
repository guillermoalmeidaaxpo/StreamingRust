use async_trait::async_trait;
use std::sync::Arc;
use crate::domain::{Request, SourceKind};
use super::ports::{Planner, Plan, PlanStep, RequestContext, MappingResolver, FilterParser, Command};
use super::strategy::StrategySelector;
use super::quote_index::QuoteIndexGenerator;
use crate::infrastructure::mssql::query_builder::{CMDPQueryBuilder, HyperscaleQueryBuilder};
use crate::infrastructure::cassandra::query_builder::CassandraQueryBuilder;
use super::filter_engine::FilterProvider;
use anyhow::Result;

pub struct DefaultPlanner {
    resolver: Arc<dyn MappingResolver>,
    parser: Arc<dyn FilterParser>,
    filter_provider: Arc<FilterProvider>,
    cmdp_builder: CMDPQueryBuilder,
    hyperscale_builder: HyperscaleQueryBuilder,
    cassandra_builder: CassandraQueryBuilder,
}

impl DefaultPlanner {
    pub fn new(
        resolver: Arc<dyn MappingResolver>, 
        parser: Arc<dyn FilterParser>, 
        filter_provider: Arc<FilterProvider>,
        cassandra_builder: CassandraQueryBuilder
    ) -> Self {
        Self { 
            resolver, 
            parser,
            filter_provider,
            cmdp_builder: CMDPQueryBuilder::new(),
            hyperscale_builder: HyperscaleQueryBuilder::new(),
            cassandra_builder,
        }
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
}

#[async_trait]
impl Planner for DefaultPlanner {
    async fn build_plan(&self, ctx: RequestContext, requests: Vec<Request>) -> Result<Plan> {
        let mut steps = Vec::new();

        for request in requests {
            let mappings = self.resolver.resolve_mappings(&request.ids, ctx.data_category, &ctx.stage).await?;
            
            let raw_filters = if let Some(f) = &request.filters {
                self.parser.parse(&f.expressions, &f.filter_time_zone).await?
            } else {
                crate::domain::FilterSet { expressions: vec![], nodes: vec![], has_latest_global_filter: false }
            };

            // 15.1 Requirement: TransactionalDataCommandParser -> FilterProvider.GetFilters
            let runtime_filters = self.filter_provider.get_runtime_filters(raw_filters, &mappings, self.resolver.as_ref()).await?;

            for mapping in mappings {
                let has_aggregations = request.transformations.as_ref().map(|t| t.keys.is_some()).unwrap_or(false);
                let has_shape = request.filters.as_ref().map(|f| f.shape.is_some()).unwrap_or(false);

                if has_shape && mapping.hyperscale_id.is_some() {
                    return Err(anyhow::anyhow!("The Shape filter cannot be used with this Identifier. Data is not hosted in CMDP"));
                }

                let has_rank_over = runtime_filters.nodes.iter().any(|n| matches!(n, crate::domain::filters::FilterNode::RankOver(_)));
                if has_rank_over {
                    if has_aggregations {
                        return Err(anyhow::anyhow!("RankOver filters cannot be combined with aggregations"));
                    }
                    if mapping.data_category == DataCategory::TimeSeries {
                        return Err(anyhow::anyhow!("RankOver filters are not supported for TimeSeries data category"));
                    }
                    if mapping.cassandra_id.is_some() || mapping.hyperscale_id.is_some() {
                        return Err(anyhow::anyhow!("RankOver filters are not supported for this Identifier"));
                    }
                }

                let source = StrategySelector::select_source(&mapping, has_aggregations, has_shape, ctx.is_mesap_endpoint);
                let shape = request.filters.as_ref()
                    .and_then(|f| f.shape.as_ref())
                    .map(|s| s.normalize());

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
                    shape,
                    filters: runtime_filters.clone(),
                    mappings: vec![mapping.clone()],
                    source,
                    quote_indices: vec![],
                    index_range: None,
                };

                // 4. Generate Quote Indices
                let limits = self.resolver.get_filter_limits(&[mapping.id], ctx.data_category).await?;
                command.quote_indices = match source {
                    SourceKind::Cassandra => {
                        let tz = StrategySelector::get_cassandra_timezone(mapping.id);
                        QuoteIndexGenerator::generate_cassandra_indices(&command.filters, &limits, &tz)
                    }
                    _ => QuoteIndexGenerator::generate_cmdp_indices(&command.filters, &limits, &command.filter_time_zone),
                };

                let hybrid_commands = self.split_hybrid_command(&ctx, command).await?;
                for h_command in hybrid_commands {
                    let queries = match h_command.source {
                        SourceKind::Cmdp => self.cmdp_builder.build_queries(&h_command)?,
                        SourceKind::Hyperscale => self.hyperscale_builder.build_queries(&h_command)?,
                        SourceKind::Cassandra => self.cassandra_builder.build_queries(&h_command)?,
                        SourceKind::Mesap => vec![], // Mesap stub
                    };

                    for query in queries {
                        steps.push(PlanStep {
                            command: h_command.clone(),
                            query,
                        });
                    }
                }
            }
        }

        Ok(Plan { steps })
    }
}
