use async_trait::async_trait;
use anyhow::{Result, anyhow};
use crate::application::ports::FilterParser;
use crate::domain::filters::*;

use crate::infrastructure::generated::outboundapilexer::OutboundAPILexer;
use crate::infrastructure::generated::outboundapiparser::*;
use crate::infrastructure::generated::outboundapiparservisitor::OutboundAPIParserVisitor;
use antlr4rust::common_token_stream::CommonTokenStream;
use antlr4rust::InputStream;
use antlr4rust::tree::{ParseTree, Visitable};

pub struct AntlrFilterParser;

impl AntlrFilterParser {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl FilterParser for AntlrFilterParser {
    async fn parse(&self, expressions: &[String], time_zone: &Option<String>) -> Result<FilterSet> {
        let mut all_nodes = Vec::new();

        for expr in expressions {
            if expr.trim().is_empty() {
                continue;
            }

            let lexer = OutboundAPILexer::new(InputStream::new(expr.as_str()));
            let token_stream = CommonTokenStream::new(lexer);
            let mut parser = OutboundAPIParser::new(token_stream);
            let tree = parser.expressionsSection().map_err(|e| anyhow!("Parse error: {:?}", e))?;

            let mut visitor = FilterVisitor::new(time_zone.clone());
            tree.accept(&mut visitor);

            if !visitor.errors.is_empty() {
                return Err(anyhow!("Filter parsing failed: {}", visitor.errors.join("; ")));
            }

            all_nodes.extend(visitor.results);

            tracing::info!("Parsing expression: {}", expr);
        }

        let has_latest_global_filter = all_nodes.iter().any(|node| match node {
            FilterNode::Comparison(c) => matches!(c.value.kind, FilterValueKind::LatestGlobal),
            _ => false,
        });

        Ok(FilterSet {
            expressions: expressions.to_vec(),
            nodes: all_nodes,
            has_latest_global_filter,
        })
    }
}

pub struct FilterVisitor {
    pub time_zone: Option<String>,
    pub results: Vec<FilterNode>,
    pub errors: Vec<String>,
}

impl FilterVisitor {
    pub fn new(time_zone: Option<String>) -> Self {
        Self {
            time_zone,
            results: Vec::new(),
            errors: Vec::new(),
        }
    }

    fn effective_time_zone(&self, provided: Option<String>) -> String {
        provided.filter(|s| !s.trim().is_empty())
            .or_else(|| self.time_zone.clone())
            .unwrap_or_else(|| "UTC".to_string())
    }

    #[allow(dead_code)]
    fn parse_point_in_time(&mut self, raw: &str, tz: Option<String>) -> FilterValue {
        let effective_tz = self.effective_time_zone(tz.clone());
        let result = crate::domain::timeexpr::parse_point_in_time_arithmetic(raw, &effective_tz);
        
        let mut value = FilterValue {
            kind: FilterValueKind::PointInTime,
            raw: raw.to_string(),
            function: None,
            arithmetic: None,
            time_zone: tz,
            start: None,
            end: None,
            arguments: vec![],
        };

        match result {
            Ok(dt) => {
                value.raw = crate::domain::timeexpr::format_utc(dt);
            }
            Err(e) => {
                self.errors.push(format!("Invalid point in time {}: {}", raw, e));
            }
        }
        value
    }
}

impl<'input> antlr4rust::tree::ParseTreeVisitor<'input, OutboundAPIParserContextType> for FilterVisitor {
    fn visit_terminal(&mut self, _node: &antlr4rust::tree::TerminalNode<'input, OutboundAPIParserContextType>) {}
    fn visit_error_node(&mut self, _node: &antlr4rust::tree::ErrorNode<'input, OutboundAPIParserContextType>) {}
}

impl<'input> OutboundAPIParserVisitor<'input> for FilterVisitor {
    fn visit_IdPointInTimeArithmeticComparison(&mut self, ctx: &IdPointInTimeArithmeticComparisonContext<'input>) {
        let field = ctx.ID().map(|id| id.get_text()).unwrap_or_else(|| ctx.keySurfaceColumn().unwrap().get_text());
        let operator = ctx.COMPARISON_OPERATOR().unwrap().get_text();
        let pit = ctx.pointInTimeArithmetic().unwrap();
        self.process_point_in_time_arithmetic(pit.as_ref(), field, operator, false);
    }

    fn visit_IdTimeIntervalIn(&mut self, ctx: &IdTimeIntervalInContext<'input>) {
        let field = ctx.ID().map(|id| id.get_text()).unwrap_or_else(|| ctx.keySurfaceColumn().unwrap().get_text());
        let ti = ctx.timeIntervalArithmetic().unwrap();
        self.process_time_interval_arithmetic(ti.as_ref(), field, false);
    }

    fn visit_IdTimeIntervalToPointInTimeComparison(&mut self, ctx: &IdTimeIntervalToPointInTimeComparisonContext<'input>) {
        let field = ctx.ID().map(|id| id.get_text()).unwrap_or_else(|| ctx.keySurfaceColumn().unwrap().get_text());
        let operator = ctx.COMPARISON_OPERATOR().unwrap().get_text();
        let ti_to_pit = ctx.timeIntervalToPointInTime().unwrap();
        self.process_time_interval_to_point_in_time(ti_to_pit.as_ref(), field, operator, false);
    }

    fn visit_IdNumericComparison(&mut self, ctx: &IdNumericComparisonContext<'input>) {
        let field = ctx.ID().unwrap().get_text();
        let operator = ctx.COMPARISON_OPERATOR().unwrap().get_text();
        let raw_value = if let Some(val) = ctx.SIGNED_INTEGER() {
            val.get_text()
        } else if let Some(val) = ctx.FLOAT() {
            val.get_text()
        } else {
            String::new()
        };

        let value = FilterValue {
            kind: FilterValueKind::Number,
            raw: raw_value,
            function: None,
            arithmetic: None,
            time_zone: None,
            start: None,
            end: None,
            arguments: vec![],
        };

        self.results.push(FilterNode::Comparison(ComparisonFilter {
            raw: ctx.get_text(),
            field,
            operator,
            value,
        }));
    }

    fn visit_IdLatestGlobalComparison(&mut self, ctx: &IdLatestGlobalComparisonContext<'input>) {
        let field = ctx.ID().unwrap().get_text();
        let operator = ctx.COMPARISON_OPERATOR().unwrap().get_text();

        if !field.eq_ignore_ascii_case("ReferenceTime") {
            self.errors.push("Only 'ReferenceTime' can be used as the target of the 'latest' and 'latestGlobal' functions.".to_string());
            return;
        }
        if operator != "=" {
            self.errors.push("Only equality comparisons ('=') are allowed with 'latest' and 'latestGlobal' functions.".to_string());
            return;
        }

        self.results.push(FilterNode::Comparison(ComparisonFilter {
            raw: ctx.get_text(),
            field,
            operator,
            value: FilterValue {
                kind: FilterValueKind::LatestGlobal,
                raw: ctx.latestGlobalFunction().unwrap().get_text(),
                function: Some("latestGlobal".to_string()),
                arithmetic: None,
                time_zone: None,
                start: None,
                end: None,
                arguments: vec![],
            },
        }));
    }

    fn visit_IdLatestComparison(&mut self, ctx: &IdLatestComparisonContext<'input>) {
        let field = ctx.ID().unwrap().get_text();
        let operator = ctx.COMPARISON_OPERATOR().unwrap().get_text();

        if !field.eq_ignore_ascii_case("ReferenceTime") {
            self.errors.push("Only 'ReferenceTime' can be used as the target of the 'latest' and 'latestGlobal' functions.".to_string());
            return;
        }
        if operator != "=" {
            self.errors.push("Only equality comparisons ('=') are allowed with 'latest' and 'latestGlobal' functions.".to_string());
            return;
        }

        let latest_fn = ctx.latestFunction().unwrap();
        let expressions = latest_fn.latestExpression_all();
        if expressions.len() != 1 {
            self.errors.push("latest function only accepts a single expression parameter.".to_string());
            return;
        }

        let expr = &expressions[0];
        let inner_field = expr.ID().unwrap().get_text();
        if !inner_field.eq_ignore_ascii_case("ReferenceTime") {
            self.errors.push("Only 'ReferenceTime' can be used in the expression inside of the 'latest' function.".to_string());
            return;
        }

        let inner_op = expr.COMPARISON_OPERATOR().map(|o| o.get_text()).unwrap_or_else(|| "in".to_string());

        if let Some(pit) = expr.pointInTimeArithmetic() {
            self.process_point_in_time_arithmetic(pit.as_ref(), inner_field, inner_op, true);
        } else if let Some(ti) = expr.timeIntervalArithmetic() {
            self.process_time_interval_arithmetic(ti.as_ref(), inner_field, true);
        } else if let Some(ti_to_pit) = expr.timeIntervalToPointInTime() {
            self.process_time_interval_to_point_in_time(ti_to_pit.as_ref(), inner_field, inner_op, true);
        } else {
            self.errors.push("Unsupported expression inside latest function.".to_string());
        }
    }

    fn visit_TextComparison(&mut self, ctx: &TextComparisonContext<'input>) {
        let field = ctx.textColumn().unwrap().get_text();
        let operator = ctx.COMPARISON_OPERATOR().unwrap().get_text();
        let val_str = ctx.genericValue().unwrap().get_text();

        self.results.push(FilterNode::Comparison(ComparisonFilter {
            raw: ctx.get_text(),
            field,
            operator,
            value: FilterValue {
                kind: FilterValueKind::Text,
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

    fn visit_RankOver(&mut self, ctx: &RankOverContext<'input>) {
        let rank_over = ctx.rankOverFunction().unwrap();
        let mut filter = RankOverFilter {
            raw: rank_over.get_text(),
            partition_by: Vec::new(),
            order_by: Vec::new(),
            bounds: Vec::new(),
        };

        let ids: Vec<String> = rank_over.ID_all().iter().map(|id| id.get_text()).collect();
        let sort_orders: Vec<String> = rank_over.SORT_ORDER_all().iter().map(|s| s.get_text()).collect();
        
        let partition_count = ids.len().saturating_sub(sort_orders.len());
        filter.partition_by.extend(ids.iter().take(partition_count).cloned());
        
        for (i, order_id) in ids.iter().skip(partition_count).enumerate() {
            let direction = sort_orders.get(i).cloned().unwrap_or_default();
            filter.order_by.push(SortExpression {
                field: order_id.clone(),
                direction,
            });
        }

        for bound_ctx in rank_over.rankOverFilter_all() {
            filter.bounds.push(self.visit_rank_over_bound_internal(bound_ctx.as_ref()));
        }

        self.results.push(FilterNode::RankOver(filter));
    }
}

impl FilterVisitor {
    fn process_point_in_time_arithmetic(
        &mut self,
        ctx: &PointInTimeArithmeticContext,
        field: String,
        operator: String,
        is_latest: bool,
    ) {
        let raw = ctx.get_text();
        let effective_tz = self.effective_time_zone(None);
        
        match crate::domain::timeexpr::parse_point_in_time_arithmetic(&raw, &effective_tz) {
            Ok(dt) => {
                let dt_str = crate::domain::timeexpr::format_utc(dt);
                let kind = if is_latest {
                    FilterValueKind::Latest
                } else {
                    FilterValueKind::PointInTime
                };
                
                self.results.push(FilterNode::Comparison(ComparisonFilter {
                    raw: format!("{} {} {}", field, operator, dt_str),
                    field,
                    operator,
                    value: FilterValue {
                        kind,
                        raw: dt_str,
                        function: None,
                        arithmetic: None,
                        time_zone: Some(effective_tz),
                        start: None,
                        end: None,
                        arguments: vec![],
                    },
                }));
            }
            Err(e) => {
                self.errors.push(format!("Failed to parse point in time arithmetic {}: {}", raw, e));
            }
        }
    }

    fn process_time_interval_arithmetic(
        &mut self,
        ctx: &TimeIntervalArithmeticContext,
        field: String,
        is_latest: bool,
    ) {
        let raw_ti = ctx.get_text();
        let effective_tz = self.effective_time_zone(None);
        
        match crate::domain::timeexpr::interval::resolve_time_interval_arithmetic(&raw_ti, &effective_tz) {
            Ok((start, end)) => {
                let start_str = crate::domain::timeexpr::format_utc(start);
                let end_str = crate::domain::timeexpr::format_utc(end);
                
                if is_latest {
                    self.results.push(FilterNode::Comparison(ComparisonFilter {
                        raw: format!("{} < {}", field, end_str),
                        field: field.clone(),
                        operator: "<".to_string(),
                        value: FilterValue {
                            kind: FilterValueKind::Latest,
                            raw: end_str,
                            function: None,
                            arithmetic: None,
                            time_zone: Some(effective_tz),
                            start: None,
                            end: None,
                            arguments: vec![],
                        },
                    }));
                } else {
                    self.results.push(FilterNode::Comparison(ComparisonFilter {
                        raw: format!("{} >= {}", field, start_str),
                        field: field.clone(),
                        operator: ">=".to_string(),
                        value: FilterValue {
                            kind: FilterValueKind::TimeInterval,
                            raw: start_str.clone(),
                            function: None,
                            arithmetic: None,
                            time_zone: Some(effective_tz.clone()),
                            start: Some(start_str.clone()),
                            end: Some(end_str.clone()),
                            arguments: vec![],
                        },
                    }));
                    
                    self.results.push(FilterNode::Comparison(ComparisonFilter {
                        raw: format!("{} < {}", field, end_str),
                        field: field.clone(),
                        operator: "<".to_string(),
                        value: FilterValue {
                            kind: FilterValueKind::TimeInterval,
                            raw: end_str.clone(),
                            function: None,
                            arithmetic: None,
                            time_zone: Some(effective_tz),
                            start: Some(start_str.clone()),
                            end: Some(end_str.clone()),
                            arguments: vec![],
                        },
                    }));
                }
            }
            Err(e) => {
                self.errors.push(format!("Failed to parse time interval arithmetic {}: {}", raw_ti, e));
            }
        }
    }

    fn process_time_interval_to_point_in_time(
        &mut self,
        ctx: &TimeIntervalToPointInTimeContext,
        field: String,
        operator: String,
        is_latest: bool,
    ) {
        let is_begin = ctx.get_text().to_lowercase().contains("begin");
        let arithmetic_ctx = ctx.timeIntervalArithmetic().unwrap();
        let raw_ti = arithmetic_ctx.get_text();
        let effective_tz = self.effective_time_zone(None);
        
        match crate::domain::timeexpr::interval::resolve_time_interval_arithmetic(&raw_ti, &effective_tz) {
            Ok((start, end)) => {
                let target_dt = if is_begin { start } else { end };
                let val_str = crate::domain::timeexpr::format_utc(target_dt);
                let kind = if is_latest {
                    FilterValueKind::Latest
                } else {
                    FilterValueKind::TimeIntervalPointTime
                };
                
                self.results.push(FilterNode::Comparison(ComparisonFilter {
                    raw: format!("{} {} {}", field, operator, val_str),
                    field,
                    operator,
                    value: FilterValue {
                        kind,
                        raw: val_str,
                        function: None,
                        arithmetic: None,
                        time_zone: Some(effective_tz),
                        start: None,
                        end: None,
                        arguments: vec![],
                    },
                }));
            }
            Err(e) => {
                self.errors.push(format!("Failed to resolve time interval to point in time {}: {}", raw_ti, e));
            }
        }
    }

    fn visit_rank_over_bound_internal(&mut self, ctx: &RankOverFilterContext) -> RankOverBound {
        let mut bound = RankOverBound {
            raw: ctx.get_text(),
            start: None,
            end: None,
        };
        let integers = ctx.SIGNED_INTEGER_all();
        if !integers.is_empty() {
            bound.start = Some(integers[0].get_text());
        }
        if integers.len() > 1 {
            bound.end = Some(integers[1].get_text());
        }
        if ctx.OPEN_FILTER_INTERVAL_MARKER().is_some() {
            bound.end = Some(ctx.OPEN_FILTER_INTERVAL_MARKER().unwrap().get_text());
        }
        bound
    }
}
