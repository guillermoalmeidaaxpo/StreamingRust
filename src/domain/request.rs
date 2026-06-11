use serde::{Deserialize, Deserializer, Serialize, Serializer};
use chrono::{DateTime, Utc, NaiveTime, Timelike};
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
    pub shape: Option<Shape>,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Shape {
    pub holiday_calendar: Option<i32>,
    pub months: Option<Vec<String>>,
    pub days: Option<Vec<String>>,
    pub time: Option<Vec<TimeRange>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeRange {
    pub start: NaiveTime,
    pub end: NaiveTime,
}

impl<'de> Deserialize<'de> for TimeRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if !s.starts_with("TimeSpan(") || !s.ends_with(')') {
            return Err(serde::de::Error::custom("Invalid TimeRange format. Expected TimeSpan(T##:##:##, T##:##:##)"));
        }
        let content = &s[9..s.len() - 1];
        let parts: Vec<&str> = content.split(',').map(|p| p.trim()).collect();
        if parts.len() != 2 {
            return Err(serde::de::Error::custom("TimeRange must have exactly 2 time values"));
        }
        let start_str = parts[0];
        let end_str = parts[1];
        if !start_str.starts_with('T') || !end_str.starts_with('T') {
            return Err(serde::de::Error::custom("Invalid time format. Expected T prefix"));
        }
        let start = NaiveTime::parse_from_str(&start_str[1..], "%H:%M:%S")
            .map_err(|e| serde::de::Error::custom(format!("Invalid start time: {}", e)))?;
        let end = NaiveTime::parse_from_str(&end_str[1..], "%H:%M:%S")
            .map_err(|e| serde::de::Error::custom(format!("Invalid end time: {}", e)))?;
        Ok(TimeRange { start, end })
    }
}

impl Serialize for TimeRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("TimeSpan(T{}, T{})", self.start.format("%H:%M:%S"), self.end.format("%H:%M:%S"));
        serializer.serialize_str(&s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedShape {
    pub months: Vec<i32>,
    pub days: Vec<i32>,
    pub time_spans: Vec<NormalizedTimeSpan>,
    pub holiday_calendar: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedTimeSpan {
    pub start: chrono::Duration,
    pub end: chrono::Duration,
}

pub const VALID_MONTHS: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
pub const VALID_DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

impl Shape {
    pub fn normalize(&self) -> NormalizedShape {
        let months = self.months.as_ref().map(|m_list| {
            let mut resolved = Vec::new();
            for m in m_list {
                if let Some(pos) = VALID_MONTHS.iter().position(|&x| x.eq_ignore_ascii_case(m)) {
                    resolved.push((pos + 1) as i32);
                }
            }
            resolved.sort_unstable();
            resolved
        }).unwrap_or_default();

        let days = self.days.as_ref().map(|d_list| {
            let mut resolved = Vec::new();
            for d in d_list {
                if let Some(pos) = VALID_DAYS.iter().position(|&x| x.eq_ignore_ascii_case(d)) {
                    resolved.push((pos + 1) as i32);
                }
            }
            resolved.sort_unstable();
            resolved
        }).unwrap_or_default();

        let time_spans = self.time.as_ref().map(|t_list| {
            t_list.iter().map(|t| {
                NormalizedTimeSpan {
                    start: chrono::Duration::seconds(t.start.num_seconds_from_midnight() as i64),
                    end: chrono::Duration::seconds(t.end.num_seconds_from_midnight() as i64),
                }
            }).collect()
        }).unwrap_or_default();

        NormalizedShape {
            months,
            days,
            time_spans,
            holiday_calendar: self.holiday_calendar,
        }
    }
}
