use streaming_rust::application::rdp_calculator::RDPCalculator;
use chrono::{TimeZone, Utc};

#[test]
fn test_rdp_daily_resolution() {
    let ref_time = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
    let del_start = Utc.with_ymd_and_hms(2023, 1, 5, 0, 0, 0).unwrap();
    
    let rdp = RDPCalculator::calculate(ref_time, del_start, "P1D", "");
    assert_eq!(rdp, Some(4));
}

#[test]
fn test_rdp_monthly_resolution() {
    let ref_time = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
    let del_start = Utc.with_ymd_and_hms(2023, 3, 1, 0, 0, 0).unwrap();
    
    let rdp = RDPCalculator::calculate(ref_time, del_start, "P1M", "");
    assert_eq!(rdp, Some(2));
}

#[test]
fn test_rdp_hourly_resolution() {
    let ref_time = Utc.with_ymd_and_hms(2023, 1, 1, 10, 0, 0).unwrap();
    let del_start = Utc.with_ymd_and_hms(2023, 1, 1, 15, 0, 0).unwrap();
    
    let rdp = RDPCalculator::calculate(ref_time, del_start, "PT1H", "");
    assert_eq!(rdp, Some(5));
}
