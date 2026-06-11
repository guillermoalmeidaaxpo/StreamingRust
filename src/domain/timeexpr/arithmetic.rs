use chrono::{DateTime, Utc, Duration};
use anyhow::{Result, anyhow};

pub fn apply_arithmetic(dt: DateTime<Utc>, operator: &str, period: &str) -> Result<DateTime<Utc>> {
    let amount = parse_period(period)?;
    
    match operator {
        "+" => Ok(dt + amount),
        "-" => Ok(dt - amount),
        _ => Err(anyhow!("Invalid arithmetic operator: {}", operator)),
    }
}

fn parse_period(period: &str) -> Result<Duration> {
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
