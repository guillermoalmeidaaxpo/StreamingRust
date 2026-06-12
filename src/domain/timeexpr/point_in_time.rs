use chrono::{DateTime, Utc, TimeZone};
use chrono_tz::Tz;
use anyhow::{Result, anyhow};
use regex::Regex;

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

pub fn parse_point_in_time_arithmetic(raw: &str, default_tz_name: &str) -> Result<DateTime<Utc>> {
    let raw = raw.trim();

    // Regex to split into: Base, Operator, Period
    let re = Regex::new(r"(?i)^(now\(\)|UTCtime\([^)]+\)|\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[-+]\d{2}:\d{2})?)\s*(?:([+-])\s*(P[0-9A-Z_]+))?$").unwrap();

    if let Some(caps) = re.captures(raw) {
        let base_str = caps.get(1).unwrap().as_str().trim();
        let operator = caps.get(2).map(|m| m.as_str());
        let period = caps.get(3).map(|m| m.as_str());

        let mut dt = parse_base_point_in_time(base_str, default_tz_name)?;

        if let (Some(op), Some(per)) = (operator, period) {
            dt = super::arithmetic::apply_arithmetic(dt, op, per)?;
        }

        Ok(dt)
    } else {
        // Fallback to parsing directly
        parse_point_in_time(raw, default_tz_name)
    }
}

fn parse_base_point_in_time(base: &str, default_tz_name: &str) -> Result<DateTime<Utc>> {
    let base = base.trim();
    if base.eq_ignore_ascii_case("now()") {
        return Ok(Utc::now());
    }

    if base.to_lowercase().starts_with("utctime(") && base.ends_with(')') {
        let inner = &base[8..base.len() - 1];
        let parts: Vec<&str> = inner.split(',').map(|p| p.trim()).collect();
        if parts.len() != 2 {
            return Err(anyhow!("Invalid UTCtime format. Expected UTCtime(datetime, timezone)"));
        }
        let dt_str = parts[0];
        let tz_str = parts[1];
        return parse_point_in_time(dt_str, tz_str);
    }

    parse_point_in_time(base, default_tz_name)
}

pub fn format_utc(dt: DateTime<Utc>) -> String {
    dt.to_rfc3339_opts(chrono::SecondsFormat::Nanos, true)
}

