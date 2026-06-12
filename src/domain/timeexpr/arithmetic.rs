use chrono::{DateTime, Utc, Duration, Datelike};
use anyhow::{Result, anyhow};
use regex::Regex;

pub fn apply_arithmetic(dt: DateTime<Utc>, operator: &str, period: &str) -> Result<DateTime<Utc>> {
    let multiplier = match operator {
        "+" => 1,
        "-" => -1,
        _ => return Err(anyhow!("Invalid arithmetic operator: {}", operator)),
    };

    let period = period.trim();

    // 1. Check for week format: P(\d+)W
    let week_re = Regex::new(r"(?i)^P(\d+)W$").unwrap();
    if let Some(caps) = week_re.captures(period) {
        let weeks: i64 = caps[1].parse().map_err(|_| anyhow!("Invalid week value"))?;
        let duration = Duration::days(weeks * 7 * multiplier);
        return Ok(dt + duration);
    }

    // 2. General ISO 8601 duration format: P(YY)?(MM)?(DD)?(T(HH)?(MM)?(SS)?)?
    let duration_re = Regex::new(r"(?i)^P(?:(\d+)Y)?(?:(\d+)M)?(?:(\d+)D)?(?:T(?:(\d+)H)?(?:(\d+)M)?(?:(\d+)S)?)?$").unwrap();
    if let Some(caps) = duration_re.captures(period) {
        let years: i32 = caps.get(1).map(|m| m.as_str().parse()).transpose()?.unwrap_or(0);
        let months: i32 = caps.get(2).map(|m| m.as_str().parse()).transpose()?.unwrap_or(0);
        let days: i64 = caps.get(3).map(|m| m.as_str().parse()).transpose()?.unwrap_or(0);
        let hours: i64 = caps.get(4).map(|m| m.as_str().parse()).transpose()?.unwrap_or(0);
        let minutes: i64 = caps.get(5).map(|m| m.as_str().parse()).transpose()?.unwrap_or(0);
        let seconds: i64 = caps.get(6).map(|m| m.as_str().parse()).transpose()?.unwrap_or(0);

        let mut res = dt;

        if years != 0 {
            let y = years.abs() as u32;
            if multiplier == 1 {
                res = res + chrono::Years::new(y);
            } else {
                res = res - chrono::Years::new(y);
            }
        }

        if months != 0 {
            let m = months.abs() as u32;
            if multiplier == 1 {
                res = res + chrono::Months::new(m);
            } else {
                res = res - chrono::Months::new(m);
            }
        }

        let total_seconds = days * 24 * 3600 + hours * 3600 + minutes * 60 + seconds;
        if total_seconds != 0 {
            res = res + Duration::seconds(total_seconds * multiplier);
        }

        return Ok(res);
    }

    // Fallback: Try old format just in case it doesn't match the new ones
    let amount = parse_period(period)?;
    match operator {
        "+" => Ok(dt + amount),
        "-" => Ok(dt - amount),
        _ => Err(anyhow!("Invalid arithmetic operator: {}", operator)),
    }
}

fn parse_period(period: &str) -> Result<Duration> {
    if period.is_empty() {
        return Err(anyhow!("Empty period"));
    }
    // Basic parser for patterns like "1D", "2H", "30M"
    let (amount_str, unit) = period.split_at(period.len() - 1);
    let amount: i64 = amount_str.parse().map_err(|_| anyhow!("Invalid period amount"))?;

    match unit.to_uppercase().as_str() {
        "D" => Ok(Duration::days(amount)),
        "H" => Ok(Duration::hours(amount)),
        "M" => Ok(Duration::minutes(amount)),
        "S" => Ok(Duration::seconds(amount)),
        _ => Err(anyhow!("Unsupported period unit: {}", unit)),
    }
}

