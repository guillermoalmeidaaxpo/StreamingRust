use serde::{Deserialize, Deserializer, Serialize, Serializer};
use chrono::{DateTime, Utc, NaiveTime, Timelike};
use super::Identifier;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Request {
    #[serde(alias = "ids", alias = "Ids")]
    pub ids: Vec<Identifier>,
    #[serde(alias = "versionAsOf", alias = "VersionAsOf")]
    pub version_as_of: Option<DateTime<Utc>>,
    #[serde(alias = "filters", alias = "Filters")]
    pub filters: Option<Filters>,
    #[serde(alias = "transformations", alias = "Transformations")]
    pub transformations: Option<Transformations>,
    #[serde(alias = "columns", alias = "Columns")]
    pub columns: Option<Vec<String>>,
    #[serde(alias = "includeDeleted", alias = "IncludeDeleted")]
    pub include_deleted: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericRequest {
    #[serde(alias = "id", alias = "Id")]
    pub id: Option<Identifier>,
    #[serde(alias = "ids", alias = "Ids")]
    pub ids: Option<Vec<Identifier>>,
    #[serde(alias = "versionAsOf", alias = "VersionAsOf")]
    pub version_as_of: Option<DateTime<Utc>>,
    #[serde(alias = "filters", alias = "Filters")]
    pub filters: Option<Filters>,
    #[serde(alias = "transformations", alias = "Transformations")]
    pub transformations: Option<Transformations>,
    #[serde(alias = "columns", alias = "Columns")]
    pub columns: Option<Vec<String>>,
    #[serde(alias = "includeDeleted", alias = "IncludeDeleted")]
    pub include_deleted: Option<bool>,
}

fn normalize_generic_transformations(transformations: Option<Transformations>) -> Transformations {
    match transformations {
        None => Transformations {
            timezone: Some("UTC".to_string()),
            target_time_zone: Some("UTC".to_string()),
            offset: Some(false),
            nested: None,
            keys: None,
            values: None,
        },
        Some(t) => {
            let offset = t.offset.unwrap_or(false);
            let has_aggregations = t.keys.as_ref().map(|k| !k.is_empty()).unwrap_or(false)
                || t.values.as_ref().map(|v| !v.is_empty()).unwrap_or(false);
            
            let mut target_time_zone = t.target_time_zone.clone();
            let mut timezone = t.timezone.clone();

            if target_time_zone.as_deref().unwrap_or("").is_empty() && timezone.as_ref().map(|tz| !tz.is_empty()).unwrap_or(false) {
                target_time_zone = timezone.clone();
            }

            if target_time_zone.as_deref().unwrap_or("").is_empty() && !offset && !has_aggregations {
                target_time_zone = Some("UTC".to_string());
                timezone = Some("UTC".to_string());
            }

            Transformations {
                timezone,
                target_time_zone,
                offset: Some(offset),
                nested: t.nested,
                keys: t.keys,
                values: t.values,
            }
        }
    }
}

impl GenericRequest {
    pub fn into_request(self) -> Request {
        let mut ids = self.ids.unwrap_or_default();
        if ids.is_empty() {
            if let Some(id) = self.id {
                ids.push(id);
            }
        }
        
        let transformations = normalize_generic_transformations(self.transformations);

        Request {
            ids,
            version_as_of: self.version_as_of,
            filters: self.filters,
            transformations: Some(transformations),
            columns: self.columns,
            include_deleted: self.include_deleted,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Filters {
    #[serde(alias = "expressions", alias = "Expressions")]
    pub expressions: Vec<String>,
    #[serde(alias = "filterTimeZone", alias = "filterTimezone", alias = "FilterTimeZone", alias = "FilterTimezone")]
    pub filter_time_zone: Option<String>,
    #[serde(alias = "shape", alias = "Shape")]
    pub shape: Option<Shape>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Transformations {
    #[serde(alias = "timezone", alias = "Timezone")]
    pub timezone: Option<String>,
    #[serde(alias = "targetTimeZone", alias = "targetTimezone", alias = "TargetTimeZone", alias = "TargetTimezone")]
    pub target_time_zone: Option<String>,
    #[serde(alias = "offset", alias = "Offset")]
    pub offset: Option<bool>,
    #[serde(alias = "nested", alias = "Nested")]
    pub nested: Option<String>,
    #[serde(alias = "keys", alias = "Keys")]
    pub keys: Option<Vec<String>>,
    #[serde(alias = "values", alias = "Values")]
    pub values: Option<Vec<Vec<String>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aggregations {
    pub group_by: Vec<AggregationColumn>,
    pub expressions: Vec<AggregationColumn>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationColumn {
    pub expression: String,
    pub alias: String,
}

fn normalize_aggregation_expression(expression: &str) -> String {
    let mut result = String::new();
    let lower = expression.to_lowercase();
    let mut last_idx = 0;
    
    while let Some(idx) = lower[last_idx..].find("delivery") {
        let abs_idx = last_idx + idx;
        result.push_str(&expression[last_idx..abs_idx]);
        
        let after_delivery = &lower[abs_idx + 8..];
        if after_delivery.starts_with("start") {
            result.push_str(&expression[abs_idx..abs_idx + 13]);
            last_idx = abs_idx + 13;
        } else {
            result.push_str("DeliveryStart");
            last_idx = abs_idx + 8;
        }
    }
    result.push_str(&expression[last_idx..]);
    result
}

impl Transformations {
    pub fn create_aggregations(&self) -> Option<Aggregations> {
        if self.keys.is_none() && self.values.is_none() {
            return None;
        }

        let group_by: Vec<AggregationColumn> = self.keys.as_ref().map(|keys| {
            keys.iter().map(|key| {
                let parts: Vec<&str> = key.splitn(2, '=').collect();
                let expr = normalize_aggregation_expression(parts[0].trim());
                let alias = if parts.len() == 2 {
                    parts[1].trim().to_string()
                } else {
                    expr.clone()
                };
                AggregationColumn {
                    expression: expr,
                    alias,
                }
            }).collect()
        }).unwrap_or_default();

        let expressions = self.values.as_ref().map(|values| {
            values.iter().filter_map(|pair| {
                if pair.len() == 2 {
                    Some(AggregationColumn {
                        expression: pair[0].trim().to_string(),
                        alias: pair[1].trim().to_string(),
                    })
                } else {
                    None
                }
            }).collect()
        }).unwrap_or_default();

        if group_by.is_empty() && expressions.is_empty() {
            return None;
        }

        Some(Aggregations {
            group_by,
            expressions,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Shape {
    #[serde(alias = "holidayCalendar", alias = "holidaycalendar", alias = "HolidayCalendar")]
    pub holiday_calendar: Option<i32>,
    #[serde(alias = "months", alias = "Months")]
    pub months: Option<Vec<String>>,
    #[serde(alias = "days", alias = "Days")]
    pub days: Option<Vec<String>>,
    #[serde(alias = "time", alias = "Time")]
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
