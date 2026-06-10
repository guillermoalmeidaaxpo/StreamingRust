use crate::domain::{SourceKind, Identifier, Mapping};

pub struct StrategySelector;

const HPFC_IDS_TO_CMDP: &[Identifier] = &[536000751, 536214287, 536346251];

impl StrategySelector {
    pub fn select_source(mapping: &Mapping, has_aggregations: bool, has_shape: bool, is_mesap_endpoint: bool) -> SourceKind {
        // 1. MESAP Check
        if is_mesap_endpoint && mapping.mesap_id.is_some() {
            return SourceKind::Mesap;
        }

        // 2. Hyperscale Check
        if mapping.hyperscale_id.is_some() {
            return SourceKind::Hyperscale;
        }

        // 3. CMDP Forced Conditions
        if has_aggregations 
            || has_shape 
            || mapping.cassandra_id.is_none() 
            || HPFC_IDS_TO_CMDP.contains(&mapping.id) 
            || !Self::is_europe_zurich_timezone(&Self::get_cassandra_timezone(mapping.id))
        {
            return SourceKind::Cmdp;
        }
        
        // 4. Default to mapping's source or Cassandra
        if mapping.source == SourceKind::Cassandra {
            SourceKind::Cassandra
        } else {
            mapping.source
        }
    }

    pub fn get_cassandra_timezone(id: Identifier) -> String {
        match id {
            536958751 | 536959001 | 536959251 | 536959501 => "Australia/Sydney".to_string(),
            536960251 => "Asia/Singapore".to_string(),
            536959751 | 536960001 => "Asia/Tokyo".to_string(),
            537085751 | 537119501 => "Pacific/Auckland".to_string(),
            _ => "Europe/Zurich".to_string(),
        }
    }

    fn is_europe_zurich_timezone(tz: &str) -> bool {
        matches!(tz.trim().to_lowercase().as_str(), "" | "cet" | "europe/zurich")
    }
}
