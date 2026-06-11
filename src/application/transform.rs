use crate::domain::DataItem;
use crate::application::ports::Command;
use crate::domain::SourceKind;
use chrono_tz::Tz;

#[derive(Clone)]
pub struct TransformationProcessor;

impl TransformationProcessor {
    pub fn new() -> Self {
        Self
    }

    pub fn process(&self, items: &mut [DataItem], command: &Command) {
        let target_tz: Option<Tz> = command.target_time_zone.parse().ok();

        for item in items.iter_mut() {
            self.process_item(item, command, target_tz);
        }
    }

    fn process_item(&self, item: &mut DataItem, command: &Command, target_tz: Option<Tz>) {
        // 1. Timezone conversion
        if let Some(_tz) = target_tz {
            for _value in item.fields.values_mut() {
                // In a real implementation, we'd check if DataValue is a timestamp
                // and convert it using the target_tz
            }
        }

        // 2. RDP Calculation
        if command.source == SourceKind::Cassandra && self.wants_column(command, "RelativeDeliveryPeriod") {
            self.calculate_rdp(item, command);
        }
    }

    fn calculate_rdp(&self, item: &mut DataItem, _command: &Command) {
        // Simplified RDP trigger
        if let (Some(_ref_val), Some(_del_val)) = (item.fields.get("ReferenceTime"), item.fields.get("DeliveryStart")) {
            // Extract DateTime and call RDPCalculator
        }
    }

    fn wants_column(&self, command: &Command, name: &str) -> bool {
        command.columns.is_empty() || command.columns.iter().any(|c| c.to_lowercase() == name.to_lowercase())
    }
}
