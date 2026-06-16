use serde::{Deserialize, Serialize};
use chrono_tz::Tz;

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MappingViews {
    pub latest_version: String,
    pub latest_reference_time: String,
    pub latest_version_with_created_on: String,
    pub latest_reference_time_with_created_on: String,
    pub get_by_created_on: String,
    pub get_by_created_on_latest_reference_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnMapping {
    pub mds_name: String,
    pub source_name: String,
    pub data_type: String,
    pub is_key: bool,
    pub is_projectable: bool,
    pub order_priority: Option<i32>,
    pub key_column_ordering: Option<i32>,
    pub value_column_ordering: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mapping {
    pub id: super::Identifier,
    pub data_category: DataCategory,
    pub cassandra_id: Option<String>,
    pub hyperscale_id: Option<i64>,
    pub mesap_id: Option<i64>,
    pub source: SourceKind,
    pub view_name: String,
    pub views: MappingViews,
    pub index_field: String,
    pub resolution: String,
    pub switch_over: String,
    pub split_query: bool,
    pub timezone: String,
    pub columns: Vec<ColumnMapping>,
}

pub fn parse_timezone(tz_name: &str) -> Option<Tz> {
    if tz_name.is_empty() {
        return None;
    }
    if let Ok(tz) = tz_name.parse::<Tz>() {
        return Some(tz);
    }
    let mapped = match tz_name.to_lowercase().as_str() {
        "w. europe standard time" | "central european standard time" | "romance standard time" | "cet" | "cest" | "met" => {
            Some("Europe/Zurich")
        }
        "gmt standard time" | "gmt" | "wet" | "west" | "bst" => Some("Europe/London"),
        "gtb standard time" | "e. europe standard time" | "eet" | "eest" => Some("Europe/Athens"),
        "belarus standard time" | "fet" => Some("Europe/Minsk"),
        "russian standard time" | "msk" => Some("Europe/Moscow"),
        "turkey standard time" | "trt" => Some("Europe/Istanbul"),
        "eastern standard time" | "est" | "edt" => Some("America/New_York"),
        "central standard time" | "cst" | "cdt" => Some("America/Chicago"),
        "mountain standard time" | "mst" | "mdt" => Some("America/Denver"),
        "pacific standard time" | "pst" | "pdt" => Some("America/Los_Angeles"),
        "atlantic standard time" | "ast" | "adt" => Some("America/Halifax"),
        "newfoundland standard time" | "nst" | "ndt" => Some("America/St_Johns"),
        "alaskan standard time" | "akt" => Some("America/Anchorage"),
        "hawaiian standard time" | "hst" => Some("Pacific/Honolulu"),
        "e. south america standard time" | "brt" => Some("America/Sao_Paulo"),
        "argentina standard time" | "art" => Some("America/Argentina/Buenos_Aires"),
        "india standard time" | "ist" => Some("Asia/Kolkata"),
        "pakistan standard time" | "pkt" => Some("Asia/Karachi"),
        "se asia standard time" | "ict" | "wib" => Some("Asia/Bangkok"),
        "singapore standard time" | "sgt" => Some("Asia/Singapore"),
        "china standard time" | "hkt" | "cst8" => Some("Asia/Hong_Kong"),
        "tokyo standard time" | "jst" => Some("Asia/Tokyo"),
        "korea standard time" | "kst" => Some("Asia/Seoul"),
        "aus eastern standard time" | "aest" | "aedt" => Some("Australia/Sydney"),
        "cen. australia standard time" | "acst" => Some("Australia/Adelaide"),
        "w. australia standard time" | "awst" => Some("Australia/Perth"),
        "new zealand standard time" | "nzst" | "nzdt" => Some("Pacific/Auckland"),
        "e. africa standard time" | "eat" => Some("Africa/Nairobi"),
        "south africa standard time" | "cat" | "sast" => Some("Africa/Johannesburg"),
        "w. central africa standard time" | "wat" => Some("Africa/Lagos"),
        "arab standard time" | "ast3" => Some("Asia/Riyadh"),
        "arabian standard time" | "gst" => Some("Asia/Dubai"),
        "israel standard time" | "idt" => Some("Asia/Jerusalem"),
        "utc" | "gmt+0" | "gmt-0" | "z" => Some("UTC"),
        _ => None,
    };
    if let Some(m) = mapped {
        m.parse::<Tz>().ok()
    } else {
        None
    }
}
