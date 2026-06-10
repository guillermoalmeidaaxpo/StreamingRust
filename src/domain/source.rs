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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mapping {
    pub id: super::Identifier,
    pub cassandra_id: Option<String>,
    pub hyperscale_id: Option<i64>,
    pub mesap_id: Option<i64>,
    pub source: SourceKind,
}
