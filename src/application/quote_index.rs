use chrono::{DateTime, Utc, Datelike, Timelike, Duration, TimeZone};
use chrono_tz::Tz;
use crate::domain::filters::{FilterSet, FilterNode};

pub struct QuoteIndexGenerator;

#[derive(Debug, Clone, Default)]
pub struct FilterLimits {
    pub min_reference_time: Option<DateTime<Utc>>,
    pub max_reference_time: Option<DateTime<Utc>>,
}

impl QuoteIndexGenerator {
    pub fn generate_cassandra_indices(filters: &FilterSet, limits: &FilterLimits, time_zone_mdo: &str) -> Vec<i32> {
        let tz = if time_zone_mdo.is_empty() || time_zone_mdo.eq_ignore_ascii_case("CET") { "Europe/Zurich" } else { time_zone_mdo };
        let range = Self::calculate_cassandra_date_range(filters, limits, tz);
        Self::to_quote_indices(range.0, range.1, true)
    }

    pub fn generate_cmdp_indices(filters: &FilterSet, limits: &FilterLimits, tz_id: &str) -> Vec<i32> {
        let tz = if tz_id.is_empty() || tz_id.eq_ignore_ascii_case("CET") { "Europe/Zurich" } else { tz_id };
        let range = Self::calculate_cmdp_date_range(filters, limits, tz);
        if let Some((start, end)) = range {
            Self::to_quote_indices(Some(start), Some(end), false)
        } else {
            vec![]
        }
    }

    fn calculate_cassandra_date_range(filters: &FilterSet, limits: &FilterLimits, time_zone_mdo: &str) -> (Option<DateTime<Utc>>, Option<DateTime<Utc>>) {
        let mut start_date: Option<DateTime<Utc>> = None;
        let mut end_date: Option<DateTime<Utc>> = None;

        for node in &filters.nodes {
            if let FilterNode::Comparison(f) = node {
                if f.field.to_lowercase() == "referencetime" {
                    if let Ok(utc_dt) = crate::domain::timeexpr::point_in_time::parse_point_in_time(&f.value.raw, time_zone_mdo) {
                        let (local_dt, is_midnight) = Self::get_local_dt_and_midnight(utc_dt, time_zone_mdo);
                        let local_date = Self::truncate_to_date(local_dt, time_zone_mdo);

                        match f.operator.as_str() {
                            ">=" => {
                                start_date = if is_midnight { Some(local_date) } else { Some(local_date + Duration::days(1)) };
                            }
                            ">" => {
                                start_date = Some(local_date + Duration::days(1));
                            }
                            "<" => {
                                end_date = if is_midnight { Some(local_date - Duration::days(1)) } else { Some(local_date) };
                            }
                            "<=" => {
                                end_date = Some(local_date);
                            }
                            "=" => {
                                if is_midnight {
                                    return (Some(local_date), Some(local_date));
                                } else {
                                    return (None, None);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        let max_ref = limits.max_reference_time;
        end_date = Self::adjust_cassandra_end_date(max_ref, end_date, time_zone_mdo);
        start_date = Self::adjust_cassandra_start_date(limits, max_ref, start_date);

        (start_date, end_date)
    }

    fn calculate_cmdp_date_range(filters: &FilterSet, limits: &FilterLimits, tz_id: &str) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
        let mut start_date: Option<DateTime<Utc>> = None;
        let mut end_date: Option<DateTime<Utc>> = None;
        let mut original_start_date: Option<DateTime<Utc>> = None;
        let mut original_end_date: Option<DateTime<Utc>> = None;
        let mut is_end_date_additional_day_needed = false;

        for node in &filters.nodes {
            if let FilterNode::Comparison(f) = node {
                if f.field.to_lowercase() == "referencetime" {
                    if let Ok(reference_time) = crate::domain::timeexpr::point_in_time::parse_point_in_time(&f.value.raw, tz_id) {
                        let reference_time = reference_time.with_timezone(&Utc);

                        match f.operator.as_str() {
                            ">" | ">=" => {
                                original_start_date = Some(reference_time);
                                start_date = if f.operator == ">" {
                                    Some(reference_time - Duration::days(3))
                                } else {
                                    Some(reference_time - Duration::days(2))
                                };
                                is_end_date_additional_day_needed = true;
                            }
                            "<" | "<=" => {
                                original_end_date = Some(reference_time);
                                end_date = if f.operator == "<" {
                                    Some(reference_time + Duration::days(3))
                                } else {
                                    Some(reference_time + Duration::days(2))
                                };
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        if let (Some(os), Some(oe)) = (original_start_date, original_end_date) {
            if os >= oe {
                return None;
            }
        }

        if start_date.is_none() && end_date.is_some() {
            start_date = limits.min_reference_time.or_else(|| Some(end_date.unwrap() - Duration::days(3 * 365))); // Simplified years
        } else if start_date.is_some() && end_date.is_none() && is_end_date_additional_day_needed {
            end_date = limits.max_reference_time.map(|m| m + Duration::days(2)).or_else(|| {
                let now = Utc::now();
                let default_end = if start_date.unwrap() >= now { start_date.unwrap() } else { now };
                Some(default_end + Duration::days(2))
            });
        }

        if let (Some(s), Some(e)) = (start_date, end_date) {
            if s > e {
                return None;
            }
            return Some((s, e));
        }

        None
    }

    fn get_local_dt_and_midnight(utc_dt: DateTime<Utc>, tz_id: &str) -> (DateTime<Tz>, bool) {
        let tz: Tz = tz_id.parse().unwrap_or(chrono_tz::UTC);
        let local = utc_dt.with_timezone(&tz);
        let is_midnight = local.hour() == 0 && local.minute() == 0 && local.second() == 0;
        (local, is_midnight)
    }

    fn truncate_to_date(local_dt: DateTime<Tz>, tz_id: &str) -> DateTime<Utc> {
        let tz: Tz = tz_id.parse().unwrap_or(chrono_tz::UTC);
        tz.with_ymd_and_hms(local_dt.year(), local_dt.month(), local_dt.day(), 0, 0, 0)
            .single()
            .expect("Valid local date")
            .with_timezone(&Utc)
    }

    fn convert_to_local_time(utc_dt: DateTime<Utc>, tz_id: &str) -> (DateTime<Utc>, bool) {
        let (local_dt, is_midnight) = Self::get_local_dt_and_midnight(utc_dt, tz_id);
        (Self::truncate_to_date(local_dt, tz_id), is_midnight)
    }

    fn adjust_cassandra_end_date(max_ref: Option<DateTime<Utc>>, end_date: Option<DateTime<Utc>>, tz_id: &str) -> Option<DateTime<Utc>> {
        if max_ref.is_none() || (end_date.is_some() && end_date.unwrap() < max_ref.unwrap()) {
            return end_date;
        }

        let (local_date, is_midnight) = Self::convert_to_local_time(max_ref.unwrap(), tz_id);
        if is_midnight {
            Some(local_date - Duration::days(1))
        } else {
            Some(local_date)
        }
    }

    fn adjust_cassandra_start_date(limits: &FilterLimits, max_ref: Option<DateTime<Utc>>, start_date: Option<DateTime<Utc>>) -> Option<DateTime<Utc>> {
        if start_date.is_some() {
            return start_date;
        }

        let utc_candidate = limits.min_reference_time.or_else(|| {
            Some((max_ref.unwrap_or_else(Utc::now)) - Duration::days(3 * 365))
        }).unwrap();

        Some(Utc.with_ymd_and_hms(utc_candidate.year(), utc_candidate.month(), utc_candidate.day(), 0, 0, 0).single().unwrap())
    }

    fn to_quote_indices(start: Option<DateTime<Utc>>, end: Option<DateTime<Utc>>, inclusive: bool) -> Vec<i32> {
        if start.is_none() && end.is_none() {
            return vec![];
        }

        let start = start.expect("Start date required if end date present");
        let now = Utc::now();
        let adjusted_end = end.unwrap_or_else(|| if start >= now { start } else { now });

        if start == adjusted_end {
            return vec![Self::format_date_as_index(start)];
        }

        if start > adjusted_end {
            return vec![];
        }

        let mut indices = Vec::new();
        let mut current = start;
        while if inclusive { current <= adjusted_end } else { current < adjusted_end } {
            indices.push(Self::format_date_as_index(current));
            current = current + Duration::days(1);
        }
        indices
    }

    fn format_date_as_index(dt: DateTime<Utc>) -> i32 {
        dt.year() * 10000 + (dt.month() as i32) * 100 + dt.day() as i32
    }
}
