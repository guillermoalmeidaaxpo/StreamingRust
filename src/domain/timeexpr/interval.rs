use chrono::{DateTime, Utc, TimeZone, Datelike, Timelike, Duration};
use chrono_tz::Europe::Zurich;
use chrono_tz::Tz;
use anyhow::{Result, anyhow};

pub fn resolve_interval(name: &str, start_point: DateTime<Utc>, tz_name: &str) -> Result<(DateTime<Utc>, DateTime<Utc>)> {
    let tz: Tz = match tz_name.to_uppercase().as_str() {
        "" | "UTC" => chrono_tz::UTC,
        "CET" => "Europe/Zurich".parse().map_err(|_| anyhow!("Invalid timezone CET"))?,
        _ => tz_name.parse().map_err(|_| anyhow!("Invalid timezone {}", tz_name))?,
    };

    let name_lower = name.to_lowercase();
    match name_lower.as_str() {
        "tiday" => Ok(tiday(start_point, tz)),
        "tiweek" => Ok(tiweek(start_point, tz)),
        "timonth" => Ok(timonth(start_point, tz)),
        "tiquarter" => Ok(tiquarter(start_point, tz)),
        "tiyear" => Ok(tiyear(start_point, tz)),
        "gasdayeurope" => Ok(gasdayeurope(start_point)),
        "gasweekeurope" => Ok(gasweekeurope(start_point)),
        "gasmontheurope" => Ok(gasmontheurope(start_point)),
        "gasquartereurope" => Ok(gasquartereurope(start_point)),
        "gassummereurope" => Ok(gassummereurope(start_point)?),
        "gaswintereurope" => Ok(gaswintereurope(start_point)?),
        "gasyeareurope" => Ok(gasyeareurope(start_point)),
        _ => Err(anyhow!("unsupported interval function: {}", name)),
    }
}

fn resolve_local(tz: Tz, date: chrono::NaiveDate, hour: u32) -> DateTime<Tz> {
    match tz.from_local_datetime(&date.and_hms_opt(hour, 0, 0).unwrap()) {
        chrono::LocalResult::Single(dt) => dt,
        chrono::LocalResult::Ambiguous(min, _) => min,
        chrono::LocalResult::None => {
            match tz.from_local_datetime(&date.and_hms_opt(hour + 1, 0, 0).unwrap()) {
                chrono::LocalResult::Single(dt) => dt,
                chrono::LocalResult::Ambiguous(min, _) => min,
                chrono::LocalResult::None => panic!("Invalid local datetime"),
            }
        }
    }
}

// -----------------------------------------------------------------------------
// Standard Time Intervals (TIDay, TIMonth, etc.)
// -----------------------------------------------------------------------------

fn tiday(dt: DateTime<Utc>, tz: Tz) -> (DateTime<Utc>, DateTime<Utc>) {
    let local = dt.with_timezone(&tz);
    let start_local = resolve_local(tz, local.date_naive(), 0);
    let start = start_local.with_timezone(&Utc);
    let end = (start_local + Duration::days(1)).with_timezone(&Utc);
    (start, end)
}

fn tiweek(dt: DateTime<Utc>, tz: Tz) -> (DateTime<Utc>, DateTime<Utc>) {
    let local = dt.with_timezone(&tz);
    let days_from_monday = local.weekday().num_days_from_monday();
    let start_date = local.date_naive() - Duration::days(days_from_monday as i64);
    let start_local = resolve_local(tz, start_date, 0);
    let start = start_local.with_timezone(&Utc);
    let end = (start_local + Duration::days(7)).with_timezone(&Utc);
    (start, end)
}

fn timonth(dt: DateTime<Utc>, tz: Tz) -> (DateTime<Utc>, DateTime<Utc>) {
    let local = dt.with_timezone(&tz);
    let start_date = local.date_naive().with_day(1).unwrap();
    let start_local = resolve_local(tz, start_date, 0);
    let start = start_local.with_timezone(&Utc);
    
    let next_month = if local.month() == 12 { 1 } else { local.month() + 1 };
    let next_year = if local.month() == 12 { local.year() + 1 } else { local.year() };
    let end_date = chrono::NaiveDate::from_ymd_opt(next_year, next_month, 1).unwrap();
    let end_local = resolve_local(tz, end_date, 0);
    let end = end_local.with_timezone(&Utc);
    (start, end)
}

fn tiquarter(dt: DateTime<Utc>, tz: Tz) -> (DateTime<Utc>, DateTime<Utc>) {
    let local = dt.with_timezone(&tz);
    let quarter = (local.month() - 1) / 3 + 1;
    let start_month = (quarter - 1) * 3 + 1;
    let start_date = chrono::NaiveDate::from_ymd_opt(local.year(), start_month, 1).unwrap();
    let start_local = resolve_local(tz, start_date, 0);
    let start = start_local.with_timezone(&Utc);
    
    let next_quarter = if quarter == 4 { 1 } else { quarter + 1 };
    let next_year = if quarter == 4 { local.year() + 1 } else { local.year() };
    let next_month = (next_quarter - 1) * 3 + 1;
    let end_date = chrono::NaiveDate::from_ymd_opt(next_year, next_month, 1).unwrap();
    let end_local = resolve_local(tz, end_date, 0);
    let end = end_local.with_timezone(&Utc);
    (start, end)
}

fn tiyear(dt: DateTime<Utc>, tz: Tz) -> (DateTime<Utc>, DateTime<Utc>) {
    let local = dt.with_timezone(&tz);
    let start_date = chrono::NaiveDate::from_ymd_opt(local.year(), 1, 1).unwrap();
    let start_local = resolve_local(tz, start_date, 0);
    let start = start_local.with_timezone(&Utc);
    
    let end_date = chrono::NaiveDate::from_ymd_opt(local.year() + 1, 1, 1).unwrap();
    let end_local = resolve_local(tz, end_date, 0);
    let end = end_local.with_timezone(&Utc);
    (start, end)
}

// -----------------------------------------------------------------------------
// Gas Europe Time Intervals 
// These start at 06:00:00 local time in Europe/Zurich
// -----------------------------------------------------------------------------

fn get_gas_day_start_utc(dt: DateTime<Utc>) -> DateTime<Utc> {
    let local = dt.with_timezone(&Zurich);
    // If local time is before 06:00, the gas day started yesterday
    let mut gas_date = local.date_naive();
    if local.hour() < 6 {
        gas_date -= Duration::days(1);
    }
    // Gas day starts at 06:00:00 Europe/Zurich
    Zurich.from_local_datetime(&gas_date.and_hms_opt(6, 0, 0).unwrap())
        .single()
        .expect("Valid local gas day time")
        .with_timezone(&Utc)
}

fn gasdayeurope(dt: DateTime<Utc>) -> (DateTime<Utc>, DateTime<Utc>) {
    let start = get_gas_day_start_utc(dt);
    // The gas day ends exactly 24 hours (or 23/25 depending on DST boundary) later?
    // Actually, it ends at 06:00:00 the next day local time.
    let local_start = start.with_timezone(&Zurich);
    let end_local = (local_start.date_naive() + Duration::days(1)).and_hms_opt(6, 0, 0).unwrap();
    let end = Zurich.from_local_datetime(&end_local).single().unwrap().with_timezone(&Utc);
    (start, end)
}

fn gasweekeurope(dt: DateTime<Utc>) -> (DateTime<Utc>, DateTime<Utc>) {
    let local = get_gas_day_start_utc(dt).with_timezone(&Zurich);
    let days_from_monday = local.weekday().num_days_from_monday();
    let start_local = (local.date_naive() - Duration::days(days_from_monday as i64)).and_hms_opt(6, 0, 0).unwrap();
    let start = Zurich.from_local_datetime(&start_local).single().unwrap().with_timezone(&Utc);
    
    let end_local = start_local + Duration::days(7);
    let end = Zurich.from_local_datetime(&end_local).single().unwrap().with_timezone(&Utc);
    (start, end)
}

fn gasmontheurope(dt: DateTime<Utc>) -> (DateTime<Utc>, DateTime<Utc>) {
    let local = get_gas_day_start_utc(dt).with_timezone(&Zurich);
    let start_local = local.date_naive().with_day(1).unwrap().and_hms_opt(6, 0, 0).unwrap();
    let start = Zurich.from_local_datetime(&start_local).single().unwrap().with_timezone(&Utc);
    
    let next_month = if local.month() == 12 { 1 } else { local.month() + 1 };
    let next_year = if local.month() == 12 { local.year() + 1 } else { local.year() };
    let end_local = chrono::NaiveDate::from_ymd_opt(next_year, next_month, 1).unwrap().and_hms_opt(6, 0, 0).unwrap();
    let end = Zurich.from_local_datetime(&end_local).single().unwrap().with_timezone(&Utc);
    (start, end)
}

fn gasquartereurope(dt: DateTime<Utc>) -> (DateTime<Utc>, DateTime<Utc>) {
    let local = get_gas_day_start_utc(dt).with_timezone(&Zurich);
    let quarter = (local.month() - 1) / 3 + 1;
    let start_month = (quarter - 1) * 3 + 1;
    let start_local = chrono::NaiveDate::from_ymd_opt(local.year(), start_month, 1).unwrap().and_hms_opt(6, 0, 0).unwrap();
    let start = Zurich.from_local_datetime(&start_local).single().unwrap().with_timezone(&Utc);
    
    let next_quarter = if quarter == 4 { 1 } else { quarter + 1 };
    let next_year = if quarter == 4 { local.year() + 1 } else { local.year() };
    let next_month = (next_quarter - 1) * 3 + 1;
    let end_local = chrono::NaiveDate::from_ymd_opt(next_year, next_month, 1).unwrap().and_hms_opt(6, 0, 0).unwrap();
    let end = Zurich.from_local_datetime(&end_local).single().unwrap().with_timezone(&Utc);
    (start, end)
}

fn gassummereurope(dt: DateTime<Utc>) -> Result<(DateTime<Utc>, DateTime<Utc>)> {
    // Gas summer starts April 1st at 06:00
    let local = get_gas_day_start_utc(dt).with_timezone(&Zurich);
    let start_local = chrono::NaiveDate::from_ymd_opt(local.year(), 4, 1).unwrap().and_hms_opt(6, 0, 0).unwrap();
    let start = Zurich.from_local_datetime(&start_local).single().unwrap().with_timezone(&Utc);
    
    let end_local = chrono::NaiveDate::from_ymd_opt(local.year(), 10, 1).unwrap().and_hms_opt(6, 0, 0).unwrap();
    let end = Zurich.from_local_datetime(&end_local).single().unwrap().with_timezone(&Utc);
    
    if dt < start || dt >= end {
        return Err(anyhow!("The provided point in time is outside of the Gas Summer Europe interval"));
    }
    Ok((start, end))
}

fn gaswintereurope(dt: DateTime<Utc>) -> Result<(DateTime<Utc>, DateTime<Utc>)> {
    // Gas winter starts Oct 1st at 06:00 and ends April 1st at 06:00
    let local = get_gas_day_start_utc(dt).with_timezone(&Zurich);
    let year_start_mod = if local.month() <= 7 { -1 } else { 0 };
    let year_end_mod = if local.month() > 7 { 1 } else { 0 };
    
    let start_local = chrono::NaiveDate::from_ymd_opt(local.year() + year_start_mod, 10, 1).unwrap().and_hms_opt(6, 0, 0).unwrap();
    let start = Zurich.from_local_datetime(&start_local).single().unwrap().with_timezone(&Utc);
    
    let end_local = chrono::NaiveDate::from_ymd_opt(local.year() + year_end_mod, 4, 1).unwrap().and_hms_opt(6, 0, 0).unwrap();
    let end = Zurich.from_local_datetime(&end_local).single().unwrap().with_timezone(&Utc);
    
    if dt < start || dt >= end {
        return Err(anyhow!("The provided point in time is outside of the Gas Winter Europe interval"));
    }
    Ok((start, end))
}

fn gasyeareurope(dt: DateTime<Utc>) -> (DateTime<Utc>, DateTime<Utc>) {
    // Gas year starts on Oct 1st at 06:00
    let local = get_gas_day_start_utc(dt).with_timezone(&Zurich);
    let start_year = if local.month() < 10 { local.year() - 1 } else { local.year() };
    
    let start_local = chrono::NaiveDate::from_ymd_opt(start_year, 10, 1).unwrap().and_hms_opt(6, 0, 0).unwrap();
    let start = Zurich.from_local_datetime(&start_local).single().unwrap().with_timezone(&Utc);
    
    let end_local = chrono::NaiveDate::from_ymd_opt(start_year + 1, 10, 1).unwrap().and_hms_opt(6, 0, 0).unwrap();
    let end = Zurich.from_local_datetime(&end_local).single().unwrap().with_timezone(&Utc);
    
    (start, end)
}

pub fn resolve_time_interval_arithmetic(raw: &str, default_tz: &str) -> Result<(DateTime<Utc>, DateTime<Utc>)> {
    let raw = raw.trim();
    
    // Find the last occurrence of ')'
    let last_paren = raw.rfind(')').ok_or_else(|| anyhow!("Invalid interval expression: {}", raw))?;
    let base_interval = raw[..=last_paren].trim();
    let trailing = raw[last_paren + 1..].trim();
    
    let (mut start, mut end) = resolve_base_time_interval(base_interval, default_tz)?;
    
    if !trailing.is_empty() {
        let re = regex::Regex::new(r"^\s*([+-])\s*(P[0-9A-Z_]+)\s*$").unwrap();
        if let Some(caps) = re.captures(trailing) {
            let op = caps.get(1).unwrap().as_str();
            let per = caps.get(2).unwrap().as_str();
            start = crate::domain::timeexpr::arithmetic::apply_arithmetic(start, op, per)?;
            end = crate::domain::timeexpr::arithmetic::apply_arithmetic(end, op, per)?;
        } else {
            return Err(anyhow!("Invalid trailing interval arithmetic: {}", trailing));
        }
    }
    
    Ok((start, end))
}

fn resolve_base_time_interval(base: &str, default_tz: &str) -> Result<(DateTime<Utc>, DateTime<Utc>)> {
    let base = base.trim();
    
    // Check if it's explicit: ti(start, end)
    if base.to_lowercase().starts_with("ti(") && base.ends_with(')') {
        let inner = &base[3..base.len() - 1];
        let parts: Vec<&str> = inner.split(',').map(|p| p.trim()).collect();
        if parts.len() != 2 {
            return Err(anyhow!("Explicit interval must have start and end datetimes"));
        }
        let start_dt = crate::domain::timeexpr::point_in_time::parse_point_in_time(parts[0], default_tz)?;
        let end_dt = crate::domain::timeexpr::point_in_time::parse_point_in_time(parts[1], default_tz)?;
        return Ok((start_dt, end_dt));
    }
    
    let first_paren = base.find('(').ok_or_else(|| anyhow!("Invalid interval function: {}", base))?;
    let name = &base[..first_paren].trim();
    let inner = &base[first_paren + 1..base.len() - 1];
    
    let last_comma = find_last_comma_outside_parens(inner);
    
    let (pit_expr, tz_str) = match last_comma {
        Some(idx) => {
            let pit = inner[..idx].trim();
            let tz = inner[idx + 1..].trim();
            (pit, tz)
        }
        None => (inner.trim(), default_tz),
    };
    
    let start_point = crate::domain::timeexpr::point_in_time::parse_point_in_time_arithmetic(pit_expr, default_tz)?;
    
    resolve_interval(name, start_point, tz_str)
}

fn find_last_comma_outside_parens(s: &str) -> Option<usize> {
    let mut depth = 0;
    let mut last_comma = None;
    for (i, c) in s.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' if depth == 0 => last_comma = Some(i),
            _ => {}
        }
    }
    last_comma
}

