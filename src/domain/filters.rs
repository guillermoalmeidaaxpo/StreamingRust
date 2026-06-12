use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterSet {
    pub expressions: Vec<String>,
    pub nodes: Vec<FilterNode>,
    #[serde(default)]
    pub has_latest_global_filter: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FilterNode {
    Comparison(ComparisonFilter),
    RankOver(RankOverFilter),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonFilter {
    pub raw: String,
    pub field: String,
    pub operator: String,
    pub value: FilterValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterValue {
    pub kind: FilterValueKind,
    pub raw: String,
    pub function: Option<String>,
    pub arithmetic: Option<TimeArithmetic>,
    pub time_zone: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub arguments: Vec<LatestExpression>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterValueKind {
    PointInTime,
    TimeInterval,
    TimeIntervalPointTime,
    Number,
    Text,
    Latest,
    LatestGlobal,
    Generic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeArithmetic {
    pub operator: String,
    pub period: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatestExpression {
    pub raw: String,
    pub field: String,
    pub operator: String,
    pub value: Box<FilterValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankOverFilter {
    pub raw: String,
    pub partition_by: Vec<String>,
    pub order_by: Vec<SortExpression>,
    pub bounds: Vec<RankOverBound>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortExpression {
    pub field: String,
    pub direction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankOverBound {
    pub raw: String,
    pub start: Option<String>,
    pub end: Option<String>,
}
