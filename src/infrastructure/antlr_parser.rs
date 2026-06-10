use async_trait::async_trait;
use anyhow::{Result, anyhow};
use crate::application::ports::FilterParser;
use crate::domain::filters::*;
use crate::domain::DataCategory;

use crate::infrastructure::generated::outboundapilexer::OutboundAPILexer;
use crate::infrastructure::generated::outboundapiparser::*;
use crate::infrastructure::generated::outboundapiparservisitor::OutboundAPIParserVisitor;
use antlr4rust::common_token_stream::CommonTokenStream;
use antlr4rust::InputStream;
use antlr4rust::tree::{ParseTree, ParseTreeVisitor, Visitable};

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

        Ok(FilterSet {
            expressions: expressions.to_vec(),
            nodes: all_nodes,
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

    fn parse_point_in_time(&mut self, raw: &str, tz: Option<String>) -> FilterValue {
        let effective_tz = self.effective_time_zone(tz.clone());
        let result = crate::domain::timeexpr::parse_point_in_time(raw, &effective_tz);
        
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

impl<'input> OutboundAPIParserVisitor<'input> for FilterVisitor {}
