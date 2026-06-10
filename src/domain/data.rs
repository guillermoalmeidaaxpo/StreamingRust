use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::Identifier;

pub type DataValue = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataItem {
    pub id: Identifier,
    pub fields: HashMap<String, DataValue>,
}
