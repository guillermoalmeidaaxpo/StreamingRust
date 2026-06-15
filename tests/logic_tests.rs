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
        assert_eq!(c.value.raw, "2024-04-25T22:00:00.000000000Z");
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
        assert_eq!(c.operator, "<=");
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

struct MockMappingResolver {
    watermark: chrono::DateTime<chrono::Utc>,
    mapping: streaming_rust::domain::Mapping,
}

#[async_trait::async_trait]
impl streaming_rust::application::ports::MappingResolver for MockMappingResolver {
    async fn resolve_mappings(&self, _ids: &[streaming_rust::domain::Identifier], _category: streaming_rust::domain::DataCategory, _stage: &str) -> anyhow::Result<Vec<streaming_rust::domain::Mapping>> {
        Ok(vec![self.mapping.clone()])
    }
    async fn get_watermark(&self, _mappings: &[streaming_rust::domain::Mapping]) -> anyhow::Result<chrono::DateTime<chrono::Utc>> {
        Ok(self.watermark)
    }
    async fn get_filter_limits(&self, _ids: &[streaming_rust::domain::Identifier], _category: streaming_rust::domain::DataCategory) -> anyhow::Result<streaming_rust::application::quote_index::FilterLimits> {
        Ok(streaming_rust::application::quote_index::FilterLimits::default())
    }
    async fn get_max_reference_time_before(&self, _id: streaming_rust::domain::Identifier, _reference_time: chrono::DateTime<chrono::Utc>, _comparison_operator: &str, _category: streaming_rust::domain::DataCategory) -> anyhow::Result<chrono::DateTime<chrono::Utc>> {
        Ok(chrono::Utc::now())
    }
}

struct MockRepository;

#[async_trait::async_trait]
impl streaming_rust::application::ports::Repository for MockRepository {
    async fn execute(&self, _query: streaming_rust::domain::ExecutableQuery) -> anyhow::Result<Vec<streaming_rust::domain::DataItem>> {
        Ok(vec![])
    }
    async fn stream(&self, _query: streaming_rust::domain::ExecutableQuery) -> anyhow::Result<std::pin::Pin<Box<dyn futures::Stream<Item = anyhow::Result<streaming_rust::domain::DataItem>> + Send>>> {
        Err(anyhow::anyhow!("Not implemented"))
    }
}

#[tokio::test]
async fn test_planner_hybrid_routing() {
    use std::sync::Arc;
    use chrono::TimeZone;
    use streaming_rust::application::planner::DefaultPlanner;
    use streaming_rust::application::ports::{Planner, RequestContext};
    use streaming_rust::domain::{SourceKind, DataCategory, Mapping, MappingViews, ColumnMapping};
    use streaming_rust::domain::request::{Filters, Request};
    use streaming_rust::infrastructure::cassandra::query_builder::CassandraQueryBuilder;
    use streaming_rust::infrastructure::antlr_parser::AntlrFilterParser;
    use streaming_rust::application::filter_engine::FilterProvider;

    let watermark = chrono::Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    
    let mapping = Mapping {
        id: 536013751.into(),
        data_category: DataCategory::Curves,
        cassandra_id: Some("power:1".to_string()),
        hyperscale_id: None,
        mesap_id: None,
        source: SourceKind::Cassandra,
        view_name: "TestView".to_string(),
        views: MappingViews {
            latest_version: String::new(),
            latest_reference_time: String::new(),
            latest_version_with_created_on: String::new(),
            latest_reference_time_with_created_on: String::new(),
            get_by_created_on: String::new(),
            get_by_created_on_latest_reference_time: String::new(),
        },
        index_field: "QuoteDateIndex".to_string(),
        resolution: "P1D".to_string(),
        switch_over: String::new(),
        split_query: true,
        timezone: "Europe/Zurich".to_string(),
        columns: vec![
            ColumnMapping {
                mds_name: "ReferenceTime".to_string(),
                source_name: "ReferenceTime".to_string(),
                data_type: "DateTime".to_string(),
                is_key: true,
                is_projectable: true,
                order_priority: None,
                key_column_ordering: None,
                value_column_ordering: None,
            }
        ],
    };

    let resolver = Arc::new(MockMappingResolver { watermark, mapping });
    let parser = Arc::new(AntlrFilterParser::new());
    let filter_provider = Arc::new(FilterProvider::new(Arc::new(MockRepository)));
    
    let mut table_mappings = std::collections::HashMap::new();
    table_mappings.insert("power".to_string(), "hpfc".to_string());
    let cassandra_builder = CassandraQueryBuilder::new(table_mappings, None);
    
    let planner = DefaultPlanner::new(resolver, parser, filter_provider, cassandra_builder);

    // Context
    let ctx = RequestContext {
        stage: "development".to_string(),
        is_mesap_endpoint: false,
        data_category: DataCategory::Curves,
    };

    // 1. Query < watermark (2023-12-30T00:00:00Z)
    let req1 = Request {
        ids: vec![536013751.into()],
        version_as_of: None,
        filters: Some(Filters {
            expressions: vec!["ReferenceTime = 2023-12-30T00:00:00".to_string()],
            filter_time_zone: Some("Europe/Zurich".to_string()),
            shape: None,
        }),
        transformations: None,
        columns: None,
        include_deleted: None,
    };

    let plan1 = planner.build_plan(ctx.clone(), vec![req1]).await.unwrap();
    assert_eq!(plan1.steps.len(), 1);
    assert_eq!(plan1.steps[0].command.source, SourceKind::Cassandra);

    // 2. Query >= watermark (2024-01-02T00:00:00Z)
    let req2 = Request {
        ids: vec![536013751.into()],
        version_as_of: None,
        filters: Some(Filters {
            expressions: vec!["ReferenceTime = 2024-01-02T00:00:00".to_string()],
            filter_time_zone: Some("Europe/Zurich".to_string()),
            shape: None,
        }),
        transformations: None,
        columns: None,
        include_deleted: None,
    };

    let plan2 = planner.build_plan(ctx, vec![req2]).await.unwrap();
    assert_eq!(plan2.steps.len(), 1);
    assert_eq!(plan2.steps[0].command.source, SourceKind::Cmdp);
}

