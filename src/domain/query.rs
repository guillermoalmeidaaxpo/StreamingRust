use super::identifier::Identifier;
use super::source::{SourceKind, DataCategory};
use super::filters::FilterSet;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ExecutableQuery {
    pub id: Identifier,
    pub data_category: DataCategory,
    pub source: SourceKind,
    pub filters: FilterSet,
    pub index_range: Option<IndexRange>,
    pub statement: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub arguments: Vec<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct IndexRange {
    pub start: i32,
    pub end: i32,
}
