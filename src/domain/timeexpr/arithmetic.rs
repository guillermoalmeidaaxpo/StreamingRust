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

        if years != 0 || months != 0 {
            let total_months = (years * 12 + months) * multiplier as i32;
            res = add_months(res, total_months);
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

fn add_months(dt: DateTime<Utc>, months: i32) -> DateTime<Utc> {
    let year = dt.year();
    let month = dt.month() as i32;
    let day = dt.day();
    
    let total_months = (year * 12) + (month - 1) + months;
    let mut new_year = total_months / 12;
    let mut new_month = (total_months % 12) + 1;
    if new_month <= 0 {
        new_month += 12;
        new_year -= 1;
    }
    
    let max_days = days_in_month(new_year, new_month as u32);
    let new_day = std::cmp::min(day, max_days);
    
    let naive_date = chrono::NaiveDate::from_ymd_opt(new_year, new_month as u32, new_day).unwrap();
    let naive_time = dt.time();
    let naive_datetime = naive_date.and_time(naive_time);
    
    DateTime::<Utc>::from_naive_utc_and_offset(naive_datetime, Utc)
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => panic!("Invalid month"),
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
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

