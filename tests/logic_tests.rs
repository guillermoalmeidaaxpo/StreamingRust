use streaming_rust::application::rdp_calculator::RDPCalculator;
use chrono::{TimeZone, Utc, NaiveTime};
use streaming_rust::domain::request::{Shape, TimeRange, Filters, Request};
use streaming_rust::application::ports::RequestValidationStrategy;
use streaming_rust::application::validator::TransactionalDataValidationStrategy;

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

#[test]
fn test_shape_normalization() {
    let shape = Shape {
        holiday_calendar: Some(123),
        months: Some(vec!["Jan".to_string(), "Dec".to_string(), "Jun".to_string()]),
        days: Some(vec!["Mon".to_string(), "Sun".to_string()]),
        time: Some(vec![
            TimeRange {
                start: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
                end: NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
            }
        ]),
    };

    let norm = shape.normalize();
    assert_eq!(norm.holiday_calendar, Some(123));
    // Months normalized and sorted: Jan -> 1, Jun -> 6, Dec -> 12
    assert_eq!(norm.months, vec![1, 6, 12]);
    // Days normalized and sorted: Mon -> 1, Sun -> 7
    assert_eq!(norm.days, vec![1, 7]);
    assert_eq!(norm.time_spans.len(), 1);
    assert_eq!(norm.time_spans[0].start, chrono::Duration::seconds(8 * 3600));
    assert_eq!(norm.time_spans[0].end, chrono::Duration::seconds(14 * 3600));
}

#[test]
fn test_shape_validation_success() {
    let strategy = TransactionalDataValidationStrategy;
    let request = Request {
        ids: vec![1],
        version_as_of: None,
        filters: Some(Filters {
            expressions: vec![],
            filter_time_zone: None,
            shape: Some(Shape {
                holiday_calendar: None,
                months: Some(vec!["Jan".to_string()]),
                days: Some(vec!["Mon".to_string()]),
                time: Some(vec![
                    TimeRange {
                        start: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
                        end: NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
                    }
                ]),
            }),
        }),
        transformations: None,
        columns: None,
        include_deleted: None,
    };

    let result = strategy.validate(&[request]);
    assert!(result.is_ok());
}

#[test]
fn test_shape_validation_failures() {
    let strategy = TransactionalDataValidationStrategy;

    // Invalid Month
    let req_invalid_month = Request {
        ids: vec![1],
        version_as_of: None,
        filters: Some(Filters {
            expressions: vec![],
            filter_time_zone: None,
            shape: Some(Shape {
                holiday_calendar: None,
                months: Some(vec!["XYZ".to_string()]),
                days: None,
                time: None,
            }),
        }),
        transformations: None,
        columns: None,
        include_deleted: None,
    };
    assert!(strategy.validate(&[req_invalid_month]).is_err());

    // Duplicate Day
    let req_dup_day = Request {
        ids: vec![1],
        version_as_of: None,
        filters: Some(Filters {
            expressions: vec![],
            filter_time_zone: None,
            shape: Some(Shape {
                holiday_calendar: None,
                months: None,
                days: Some(vec!["Mon".to_string(), "mon".to_string()]),
                time: None,
            }),
        }),
        transformations: None,
        columns: None,
        include_deleted: None,
    };
    assert!(strategy.validate(&[req_dup_day]).is_err());

    // Invalid Time range (start >= end)
    let req_invalid_time = Request {
        ids: vec![1],
        version_as_of: None,
        filters: Some(Filters {
            expressions: vec![],
            filter_time_zone: None,
            shape: Some(Shape {
                holiday_calendar: None,
                months: None,
                days: None,
                time: Some(vec![
                    TimeRange {
                        start: NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
                        end: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
                    }
                ]),
            }),
        }),
        transformations: None,
        columns: None,
        include_deleted: None,
    };
    assert!(strategy.validate(&[req_invalid_time]).is_err());

    // Overlapping Time ranges
    let req_overlapping = Request {
        ids: vec![1],
        version_as_of: None,
        filters: Some(Filters {
            expressions: vec![],
            filter_time_zone: None,
            shape: Some(Shape {
                holiday_calendar: None,
                months: None,
                days: None,
                time: Some(vec![
                    TimeRange {
                        start: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
                        end: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
                    },
                    TimeRange {
                        start: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
                        end: NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
                    }
                ]),
            }),
        }),
        transformations: None,
        columns: None,
        include_deleted: None,
    };
    assert!(strategy.validate(&[req_overlapping]).is_err());
}

#[tokio::test]
async fn test_antlr_filter_parsing() {
    use streaming_rust::infrastructure::antlr_parser::AntlrFilterParser;
    use streaming_rust::application::ports::FilterParser;
    use streaming_rust::domain::filters::{FilterNode, FilterValueKind};

    let parser = AntlrFilterParser::new();
    
    // 1. Point in time
    let exprs_pit = vec!["ReferenceTime = 2024-04-26T00:00:00".to_string()];
    let res = parser.parse(&exprs_pit, &Some("Europe/Zurich".to_string())).await;
    assert!(res.is_ok());
    let filter_set = res.unwrap();
    assert_eq!(filter_set.nodes.len(), 1);
    if let FilterNode::Comparison(c) = &filter_set.nodes[0] {
        assert_eq!(c.field, "ReferenceTime");
        assert_eq!(c.operator, "=");
        assert_eq!(c.value.kind, FilterValueKind::PointInTime);
        assert_eq!(c.value.raw, "2024-04-26T00:00:00");
    } else {
        panic!("Expected comparison node");
    }

    // 2. Latest
    let exprs_latest = vec!["ReferenceTime = latest(ReferenceTime <= 2024-04-26T00:00:00)".to_string()];
    let res = parser.parse(&exprs_latest, &Some("Europe/Zurich".to_string())).await;
    assert!(res.is_ok());
    let filter_set = res.unwrap();
    assert_eq!(filter_set.nodes.len(), 1);
    if let FilterNode::Comparison(c) = &filter_set.nodes[0] {
        assert_eq!(c.field, "ReferenceTime");
        assert_eq!(c.operator, "=");
        assert_eq!(c.value.kind, FilterValueKind::Latest);
    } else {
        panic!("Expected comparison node");
    }

    // 3. LatestGlobal
    let exprs_lg = vec!["ReferenceTime = latestGlobal()".to_string()];
    let res = parser.parse(&exprs_lg, &Some("Europe/Zurich".to_string())).await;
    assert!(res.is_ok());
    let filter_set = res.unwrap();
    assert_eq!(filter_set.nodes.len(), 1);
    if let FilterNode::Comparison(c) = &filter_set.nodes[0] {
        assert_eq!(c.field, "ReferenceTime");
        assert_eq!(c.operator, "=");
        assert_eq!(c.value.kind, FilterValueKind::LatestGlobal);
    } else {
        panic!("Expected comparison node");
    }
}

