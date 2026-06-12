use anyhow::Result;
use crate::application::ports::{Repository, MappingResolver};
use crate::domain::filters::{FilterSet, FilterNode, ComparisonFilter, FilterValue, FilterValueKind};
use crate::domain::Mapping;
use std::sync::Arc;

pub struct FilterProvider {
    mapper: FilterMapper,
    repository: Arc<dyn Repository>,
}

impl FilterProvider {
    pub fn new(repository: Arc<dyn Repository>) -> Self {
        Self {
            mapper: FilterMapper::new(),
            repository,
        }
    }

    pub async fn get_runtime_filters(
        &self,
        filters: FilterSet,
        mappings: &[Mapping],
        resolver: &dyn MappingResolver,
    ) -> Result<FilterSet> {
        let mut runtime_filters = filters.clone();
        
        if mappings.is_empty() {
            return Ok(runtime_filters);
        }
        
        let mapping = &mappings[0];
        let id = mapping.id;
        let category = mapping.data_category;

        // 1. Check if we need default/latestGlobal filters
        let mut has_latest_global = false;
        let mut has_latest = false;
        let mut has_rank_over = false;
        
        for node in &runtime_filters.nodes {
            match node {
                FilterNode::Comparison(c) => {
                    if matches!(c.value.kind, FilterValueKind::LatestGlobal) {
                        has_latest_global = true;
                    }
                    if matches!(c.value.kind, FilterValueKind::Latest) {
                        has_latest = true;
                    }
                }
                FilterNode::RankOver(_) => {
                    has_rank_over = true;
                }
            }
        }

        let needs_default = (runtime_filters.nodes.is_empty() || has_latest_global) && !has_rank_over && !has_latest;

        if needs_default {
            // Remove any existing LatestGlobal comparison node
            runtime_filters.nodes.retain(|node| {
                if let FilterNode::Comparison(c) = node {
                    !matches!(c.value.kind, FilterValueKind::LatestGlobal)
                } else {
                    true
                }
            });

            // Get limits
            let limits = resolver.get_filter_limits(&[id], category).await?;
            if let Some(max_ref) = limits.max_reference_time {
                let max_ref_str = crate::domain::timeexpr::format_utc(max_ref);
                runtime_filters.nodes.push(FilterNode::Comparison(ComparisonFilter {
                    raw: format!("ReferenceTime = {}", max_ref_str),
                    field: "ReferenceTime".to_string(),
                    operator: "=".to_string(),
                    value: FilterValue {
                        kind: FilterValueKind::PointInTime,
                        raw: max_ref_str,
                        function: None,
                        arithmetic: None,
                        time_zone: None,
                        start: None,
                        end: None,
                        arguments: vec![],
                    },
                }));
            } else {
                let fallback = chrono::Utc::now() - chrono::Duration::days(1);
                let fallback_str = crate::domain::timeexpr::format_utc(fallback);
                runtime_filters.nodes.push(FilterNode::Comparison(ComparisonFilter {
                    raw: format!("ReferenceTime >= {}", fallback_str),
                    field: "ReferenceTime".to_string(),
                    operator: ">=".to_string(),
                    value: FilterValue {
                        kind: FilterValueKind::PointInTime,
                        raw: fallback_str,
                        function: None,
                        arithmetic: None,
                        time_zone: None,
                        start: None,
                        end: None,
                        arguments: vec![],
                    },
                }));
            }
        } else if !mapping.hyperscale_id.is_some() && has_latest {
            // Find and resolve the latest filter
            let mut latest_node_idx = None;
            for (idx, node) in runtime_filters.nodes.iter().enumerate() {
                if let FilterNode::Comparison(c) = node {
                    if matches!(c.value.kind, FilterValueKind::Latest) {
                        latest_node_idx = Some(idx);
                        break;
                    }
                }
            }

            if let Some(idx) = latest_node_idx {
                if let FilterNode::Comparison(c) = runtime_filters.nodes.remove(idx) {
                    let target_dt = chrono::DateTime::parse_from_rfc3339(&c.value.raw)
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .map_err(|e| anyhow::anyhow!("Failed to parse latest target datetime {}: {}", c.value.raw, e))?;
                    
                    let op = c.operator.as_str();
                    
                    if op == ">" || op == ">=" {
                        let limits = resolver.get_filter_limits(&[id], category).await?;
                        if let Some(max_ref) = limits.max_reference_time {
                            let (new_op, new_val) = if max_ref > target_dt {
                                ("=".to_string(), crate::domain::timeexpr::format_utc(max_ref))
                            } else {
                                (">".to_string(), crate::domain::timeexpr::format_utc(max_ref))
                            };
                            runtime_filters.nodes.push(FilterNode::Comparison(ComparisonFilter {
                                raw: format!("ReferenceTime {} {}", new_op, new_val),
                                field: "ReferenceTime".to_string(),
                                operator: new_op,
                                value: FilterValue {
                                    kind: FilterValueKind::PointInTime,
                                    raw: new_val,
                                    function: None,
                                    arithmetic: None,
                                    time_zone: None,
                                    start: None,
                                    end: None,
                                    arguments: vec![],
                                },
                            }));
                        }
                    } else {
                        let latest_ref_time = resolver.get_max_reference_time_before(id, target_dt, op, category).await?;
                        let val_str = crate::domain::timeexpr::format_utc(latest_ref_time);
                        runtime_filters.nodes.push(FilterNode::Comparison(ComparisonFilter {
                            raw: format!("ReferenceTime = {}", val_str),
                            field: "ReferenceTime".to_string(),
                            operator: "=".to_string(),
                            value: FilterValue {
                                kind: FilterValueKind::PointInTime,
                                raw: val_str,
                                function: None,
                                arithmetic: None,
                                time_zone: None,
                                start: None,
                                end: None,
                                arguments: vec![],
                            },
                        }));
                    }
                }
            }
        }

        // 2. Map filters to database columns
        self.mapper.map_to_source(&mut runtime_filters, mappings);

        Ok(runtime_filters)
    }
}

pub struct FilterMapper;

impl FilterMapper {
    pub fn new() -> Self {
        Self
    }

    pub fn map_to_source(&self, _filters: &mut FilterSet, _mappings: &[Mapping]) {
        // Logic to translate domain field names to source column names
    }
}
