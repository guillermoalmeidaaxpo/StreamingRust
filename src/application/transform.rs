use crate::domain::DataItem;
use crate::application::ports::Command;
use crate::domain::SourceKind;
use chrono_tz::Tz;
use chrono::{DateTime, FixedOffset, Utc};

#[derive(Clone)]
pub struct TransformationProcessor;

impl TransformationProcessor {
    pub fn new() -> Self {
        Self
    }

    pub fn process(&self, items: &mut [DataItem], command: &Command) {
        let target_tz = crate::domain::source::parse_timezone(&command.target_time_zone);

        for item in items.iter_mut() {
            self.process_item(item, command, target_tz);
        }
    }

    fn process_item(&self, item: &mut DataItem, command: &Command, target_tz: Option<Tz>) {
        // 1. Timezone conversion & offset inclusion formatting
        for val in item.fields.values_mut() {
            if let Some(s) = val.as_str() {
                if let Some(dt) = parse_datetime(s) {
                    let local_dt = if let Some(tz) = target_tz {
                        (dt + chrono::Duration::milliseconds(1)).with_timezone(&tz).fixed_offset()
                    } else {
                        dt
                    };
                    let formatted = if command.include_offset {
                        local_dt.format("%Y-%m-%dT%H:%M:%S.000%:z").to_string()
                    } else {
                        local_dt.format("%Y-%m-%dT%H:%M:%S.000").to_string()
                    };
                    *val = serde_json::Value::String(formatted);
                }
            }
        }

        // 2. RDP Calculation
        if command.source == SourceKind::Cassandra && self.wants_column(command, "RelativeDeliveryPeriod") {
            self.calculate_rdp(item, command);
        }
    }

    fn calculate_rdp(&self, item: &mut DataItem, command: &Command) {
        if let (Some(ref_val), Some(del_val)) = (item.fields.get("ReferenceTime"), item.fields.get("DeliveryStart")) {
            if let (Some(ref_str), Some(del_str)) = (ref_val.as_str(), del_val.as_str()) {
                if let (Some(ref_dt), Some(del_dt)) = (parse_datetime(ref_str), parse_datetime(del_str)) {
                    if !command.mappings.is_empty() {
                        let mapping = &command.mappings[0];
                        let resolution = &mapping.resolution;
                        let period = crate::application::rdp_calculator::RDPCalculator::calculate(
                            ref_dt.with_timezone(&Utc),
                            del_dt.with_timezone(&Utc),
                            resolution,
                            ""
                        );
                        if let Some(p) = period {
                            item.fields.insert("RelativeDeliveryPeriod".to_string(), serde_json::Value::Number(p.into()));
                        }
                    }
                }
            }
        }
    }

    fn wants_column(&self, command: &Command, name: &str) -> bool {
        command.columns.is_empty() || command.columns.iter().any(|c| c.to_lowercase() == name.to_lowercase())
    }
}

fn parse_datetime(s: &str) -> Option<DateTime<FixedOffset>> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Some(dt);
    }
    for fmt in &[
        "%Y-%m-%dT%H:%M:%S.000%:z",
        "%Y-%m-%dT%H:%M:%S%:z",
    ] {
        if let Ok(dt) = DateTime::parse_from_str(s, fmt) {
            return Some(dt);
        }
    }
    for fmt in &[
        "%Y-%m-%dT%H:%M:%S.000",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d %H:%M:%S",
    ] {
        if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(s, fmt) {
            let offset = FixedOffset::east_opt(0).unwrap();
            let dt_with_offset = DateTime::<FixedOffset>::from_naive_utc_and_offset(dt, offset);
            return Some(dt_with_offset);
        }
    }
    None
}
