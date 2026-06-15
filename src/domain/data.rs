use std::collections::HashMap;
use serde::Deserialize;
use super::Identifier;

pub type DataValue = serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct DataItem {
    pub id: Identifier,
    pub fields: HashMap<String, DataValue>,
}

impl serde::Serialize for DataItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.fields.len() + 1))?;
        
        map.serialize_entry("Identifier", &self.id)?;

        let mut keys: Vec<&String> = self.fields.keys().collect();
        keys.sort();

        for key in keys {
            if key.eq_ignore_ascii_case("Identifier") {
                continue;
            }
            
            let val = &self.fields[key];
            if val.is_array() {
                map.serialize_entry(key, val)?;
            } else {
                map.serialize_entry(key, &vec![val])?;
            }
        }
        
        map.end()
    }
}
