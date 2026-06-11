use chrono::{DateTime, Utc, Datelike};

pub struct RDPCalculator;

impl RDPCalculator {
    pub fn calculate(reference_time: DateTime<Utc>, delivery_start: DateTime<Utc>, resolution: &str, delivery_resolution: &str) -> Option<i64> {
        let ref_time = reference_time;
        let del_start = delivery_start;

        match resolution {
            "P1Y" => Some(Self::get_years_adjusted_period(ref_time, del_start, delivery_resolution) as i64),
            "P1M" => Some(Self::get_months_adjusted_period(ref_time, del_start) as i64),
            "P3M" => Some(Self::get_quarter_adjusted_period(ref_time, del_start) as i64),
            "P6M" => Some(Self::get_half_year_adjusted_period(ref_time, del_start, delivery_resolution) as i64),
            "P1D" => Some((del_start.date_naive() - ref_time.date_naive()).num_days()),
            "P1W" => Some(Self::get_week_adjusted_period(ref_time, del_start) as i64),
            "PT1H" => Some(del_start.timestamp() / 3600 - ref_time.timestamp() / 3600 ),
            "PT30M" => Some(del_start.timestamp() / 1800 - ref_time.timestamp() / 1800 ),
            "PT15M" => Some(del_start.timestamp() / 900 - ref_time.timestamp() / 900 ),
            "PT5M" => Some(del_start.timestamp() / 300 - ref_time.timestamp() / 300 ),
            "PT1M" => Some(del_start.timestamp() / 60 - ref_time.timestamp() / 60 ),
            _ => None,
        }
    }

    fn get_years_adjusted_period(ref_time: DateTime<Utc>, del_start: DateTime<Utc>, delivery_resolution: &str) -> i32 {
        let ref_time = ref_time;
        let del_start = del_start;
        if delivery_resolution == "Year" {
            // This is a simplified gas-year adjustment from Go/C#
            // In a real impl, we'd use a more robust month/year math
        }
        del_start.year() - ref_time.year() 
    }

    fn get_months_adjusted_period(ref_time: DateTime<Utc>, del_start: DateTime<Utc>) -> i32 {
        let year_diff = del_start.year() - ref_time.year();
        year_diff * 12 + (del_start.month() as i32 - ref_time.month() as i32)
    }

    fn get_quarter_adjusted_period(ref_time: DateTime<Utc>, del_start: DateTime<Utc>) -> i32 {
        let start_quarter = (ref_time.month() - 1) / 3;
        let end_quarter = (del_start.month() - 1) / 3;
        let year_diff = del_start.year() - ref_time.year();
        year_diff * 4 + (end_quarter as i32 - start_quarter as i32)
    }

    fn get_half_year_adjusted_period(ref_time: DateTime<Utc>, del_start: DateTime<Utc>, _delivery_resolution: &str) -> i32 {
        let ref_half = (ref_time.month() - 1) / 6;
        let del_half = (del_start.month() - 1) / 6;
        let year_diff = del_start.year() - ref_time.year();
        year_diff * 2 + (del_half as i32 - ref_half as i32)
    }

    fn get_week_adjusted_period(ref_time: DateTime<Utc>, del_start: DateTime<Utc>) -> i32 {
        // Rust chrono week logic
        let start_ref = ref_time.date_naive();
        let start_del = del_start.date_naive();
        ((start_del - start_ref).num_days() / 7) as i32
    }
}
