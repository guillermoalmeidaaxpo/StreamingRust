use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SourceKind {
    Cassandra,
    Cmdp,
    Mesap,
    Hyperscale,
}

impl Default for SourceKind {
    fn default() -> Self {
        Self::Cmdp
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataCategory {
    Curves,
    Surfaces,
    TimeSeries,
}

impl Default for DataCategory {
    fn default() -> Self {
        Self::TimeSeries
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MappingViews {
    pub latest_version: String,
    pub latest_reference_time: String,
    pub latest_version_with_created_on: String,
    pub latest_reference_time_with_created_on: String,
    pub get_by_created_on: String,
    pub get_by_created_on_latest_reference_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnMapping {
    pub mds_name: String,
    pub source_name: String,
    pub data_type: String,
    pub is_key: bool,
    pub is_projectable: bool,
    pub order_priority: Option<i32>,
    pub key_column_ordering: Option<i32>,
    pub value_column_ordering: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mapping {
    pub id: super::Identifier,
    pub data_category: DataCategory,
    pub cassandra_id: Option<String>,
    pub hyperscale_id: Option<i64>,
    pub mesap_id: Option<i64>,
    pub source: SourceKind,
    pub view_name: String,
    pub views: MappingViews,
    pub index_field: String,
    pub resolution: String,
    pub switch_over: String,
    pub split_query: bool,
    pub timezone: String,
    pub columns: Vec<ColumnMapping>,
}
