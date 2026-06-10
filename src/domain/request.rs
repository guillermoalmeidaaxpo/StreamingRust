use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use super::Identifier;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Request {
    pub ids: Vec<Identifier>,
    pub version_as_of: Option<DateTime<Utc>>,
    pub filters: Option<Filters>,
    pub transformations: Option<Transformations>,
    pub columns: Option<Vec<String>>,
    pub include_deleted: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Filters {
    pub expressions: Vec<String>,
    pub filter_time_zone: Option<String>,
    pub shape: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Transformations {
    pub timezone: Option<String>,
    pub target_time_zone: Option<String>,
    pub offset: Option<bool>,
    pub nested: Option<String>,
    pub keys: Option<Vec<String>>,
    pub values: Option<Vec<Vec<String>>>,
}
