use chrono::{DateTime, Utc, TimeZone};
use chrono_tz::Tz;
use anyhow::{Result, anyhow};

pub fn parse_point_in_time(raw: &str, tz_name: &str) -> Result<DateTime<Utc>> {
    let tz: Tz = match tz_name.to_uppercase().as_str() {
        "" | "UTC" => chrono_tz::UTC,
        "CET" => "Europe/Zurich".parse().map_err(|_| anyhow!("Invalid timezone CET"))?,
        _ => tz_name.parse().map_err(|_| anyhow!("Invalid timezone {}", tz_name))?,
    };

    // Try parsing as ISO 8601 / RFC 3339 first
    if let Ok(dt) = DateTime::parse_from_rfc3339(raw) {
        return Ok(dt.with_timezone(&Utc));
    }

    // Try parsing with the provided timezone
    // Expected format: YYYY-MM-DDTHH:MM:SS
    let format = "%Y-%m-%dT%H:%M:%S";
    if let Ok(naive_dt) = chrono::NaiveDateTime::parse_from_str(raw, format) {
        return match tz.from_local_datetime(&naive_dt) {
            chrono::LocalResult::Single(dt) => Ok(dt.with_timezone(&Utc)),
            chrono::LocalResult::Ambiguous(min, _) => Ok(min.with_timezone(&Utc)), // Take earlier for now
            chrono::LocalResult::None => Err(anyhow!("Invalid datetime for timezone")),
        };
    }

    Err(anyhow!("Failed to parse point in time: {}", raw))
}

pub fn format_utc(dt: DateTime<Utc>) -> String {
    dt.to_rfc3339_opts(chrono::SecondsFormat::Nanos, true)
}
