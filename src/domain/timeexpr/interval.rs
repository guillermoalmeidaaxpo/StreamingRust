use chrono::{DateTime, Utc, TimeZone, Datelike, Timelike, Duration};
use chrono_tz::Europe::Zurich;
use anyhow::{Result, anyhow};

pub fn resolve_interval(name: &str, start_point: DateTime<Utc>) -> Result<(DateTime<Utc>, DateTime<Utc>)> {
    let name_lower = name.to_lowercase();
    match name_lower.as_str() {
        "tiday" => Ok(tiday(start_point)),
        "tiweek" => Ok(tiweek(start_point)),
        "timonth" => Ok(timonth(start_point)),
        "tiquarter" => Ok(tiquarter(start_point)),
        "tiyear" => Ok(tiyear(start_point)),
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

// -----------------------------------------------------------------------------
// Standard Time Intervals (TIDay, TIMonth, etc.)
// These start at 00:00:00 UTC of the respective boundary
// -----------------------------------------------------------------------------

fn tiday(dt: DateTime<Utc>) -> (DateTime<Utc>, DateTime<Utc>) {
    let start = Utc.with_ymd_and_hms(dt.year(), dt.month(), dt.day(), 0, 0, 0).single().unwrap();
    let end = start + Duration::days(1);
    (start, end)
}

fn tiweek(dt: DateTime<Utc>) -> (DateTime<Utc>, DateTime<Utc>) {
    let days_from_monday = dt.weekday().num_days_from_monday();
    let start = (dt - Duration::days(days_from_monday as i64)).with_time(chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap()).unwrap();
    let end = start + Duration::days(7);
    (start, end)
}

fn timonth(dt: DateTime<Utc>) -> (DateTime<Utc>, DateTime<Utc>) {
    let start = Utc.with_ymd_and_hms(dt.year(), dt.month(), 1, 0, 0, 0).single().unwrap();
    let next_month = if dt.month() == 12 { 1 } else { dt.month() + 1 };
    let next_year = if dt.month() == 12 { dt.year() + 1 } else { dt.year() };
    let end = Utc.with_ymd_and_hms(next_year, next_month, 1, 0, 0, 0).single().unwrap();
    (start, end)
}

fn tiquarter(dt: DateTime<Utc>) -> (DateTime<Utc>, DateTime<Utc>) {
    let quarter = (dt.month() - 1) / 3 + 1;
    let start_month = (quarter - 1) * 3 + 1;
    let start = Utc.with_ymd_and_hms(dt.year(), start_month, 1, 0, 0, 0).single().unwrap();
    
    let next_quarter = if quarter == 4 { 1 } else { quarter + 1 };
    let next_year = if quarter == 4 { dt.year() + 1 } else { dt.year() };
    let next_month = (next_quarter - 1) * 3 + 1;
    let end = Utc.with_ymd_and_hms(next_year, next_month, 1, 0, 0, 0).single().unwrap();
    (start, end)
}

fn tiyear(dt: DateTime<Utc>) -> (DateTime<Utc>, DateTime<Utc>) {
    let start = Utc.with_ymd_and_hms(dt.year(), 1, 1, 0, 0, 0).single().unwrap();
    let end = Utc.with_ymd_and_hms(dt.year() + 1, 1, 1, 0, 0, 0).single().unwrap();
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
