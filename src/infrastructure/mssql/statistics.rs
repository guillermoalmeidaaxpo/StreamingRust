use async_trait::async_trait;
use crate::application::ports::{StatisticsService, MappingResolver};
use crate::domain::{Identifier, FilterSet, DataCategory, Mapping, filters::FilterNode};
use anyhow::{Result, anyhow};
use bb8_tiberius::ConnectionManager;
use bb8::Pool;
use std::sync::Arc;
use tiberius::Query;
use chrono::{DateTime, Utc, Datelike, TimeZone};

pub struct MssqlStatisticsService {
    pool: Pool<ConnectionManager>,
    mapping_resolver: Arc<dyn MappingResolver>,
    stage: String,
}

impl MssqlStatisticsService {
    pub async fn new(
        connection_string: &str,
        mapping_resolver: Arc<dyn MappingResolver>,
        stage: String,
    ) -> Result<Self> {
        let config = super::get_mssql_config(connection_string).await?;
        let manager = ConnectionManager::new(config);
        let pool = Pool::builder().build(manager).await?;
        Ok(Self {
            pool,
            mapping_resolver,
            stage,
        })
    }

    async fn fetch_statistics(&self, mdo_id: i64, category: DataCategory) -> Result<Option<DbStatistics>> {
        let (table_name, projection) = match category {
            DataCategory::TimeSeries => (
                "TimeseriesStatistics",
                "MdoId, FirstReferenceTime, LastReferenceTime"
            ),
            DataCategory::Curves => (
                "CurvesStatistics",
                "MdoId, FirstReferenceTime, LastReferenceTime, FirstDeliveryStart, LastDeliveryStart, FirstDeliveryEnd, LastDeliveryEnd, MinRelativeDeliveryPeriod, MaxRelativeDeliveryPeriod, DataRowCount"
            ),
            DataCategory::Surfaces => (
                "SurfacesStatistics",
                "MdoId, FirstReferenceTime, LastReferenceTime, DataRowCount"
            ),
        };

        let query_text = format!(
            "SELECT TOP (1) {} FROM {} WHERE MdoId = @p1",
            projection, table_name
        );

        let mut client = self.pool.get().await?;
        let mut query = Query::new(query_text);
        query.bind(mdo_id);

        let stream = query.query(&mut client).await?;
        let row_opt = stream.into_row().await?;

        if let Some(row) = row_opt {
            let mdo_id = row.get::<i64, _>("MdoId").unwrap_or(mdo_id);
            
            let first_ref_naive = row.get::<chrono::NaiveDateTime, _>("FirstReferenceTime")
                .ok_or_else(|| anyhow!("Missing FirstReferenceTime"))?;
            let first_reference_time = DateTime::<Utc>::from_naive_utc_and_offset(first_ref_naive, Utc);

            let last_ref_naive = row.get::<chrono::NaiveDateTime, _>("LastReferenceTime")
                .ok_or_else(|| anyhow!("Missing LastReferenceTime"))?;
            let last_reference_time = DateTime::<Utc>::from_naive_utc_and_offset(last_ref_naive, Utc);

            let first_delivery_start = row.get::<Option<chrono::NaiveDateTime>, _>("FirstDeliveryStart")
                .ok()
                .flatten()
                .map(|d| DateTime::<Utc>::from_naive_utc_and_offset(d, Utc));

            let last_delivery_start = row.get::<Option<chrono::NaiveDateTime>, _>("LastDeliveryStart")
                .ok()
                .flatten()
                .map(|d| DateTime::<Utc>::from_naive_utc_and_offset(d, Utc));

            let first_delivery_end = row.get::<Option<chrono::NaiveDateTime>, _>("FirstDeliveryEnd")
                .ok()
                .flatten()
                .map(|d| DateTime::<Utc>::from_naive_utc_and_offset(d, Utc));

            let last_delivery_end = row.get::<Option<chrono::NaiveDateTime>, _>("LastDeliveryEnd")
                .ok()
                .flatten()
                .map(|d| DateTime::<Utc>::from_naive_utc_and_offset(d, Utc));

            let min_relative_delivery_period = row.get::<Option<i32>, _>("MinRelativeDeliveryPeriod")
                .ok()
                .flatten();

            let max_relative_delivery_period = row.get::<Option<i32>, _>("MaxRelativeDeliveryPeriod")
                .ok()
                .flatten();

            let data_row_count = row.get::<Option<i32>, _>("DataRowCount")
                .ok()
                .flatten()
                .unwrap_or(0);

            Ok(Some(DbStatistics {
                mdo_id,
                first_reference_time,
                last_reference_time,
                first_delivery_start,
                last_delivery_start,
                first_delivery_end,
                last_delivery_end,
                min_relative_delivery_period,
                max_relative_delivery_period,
                data_row_count,
            }))
        } else {
            Ok(None)
        }
    }
}

struct DbStatistics {
    mdo_id: i64,
    first_reference_time: DateTime<Utc>,
    last_reference_time: DateTime<Utc>,
    first_delivery_start: Option<DateTime<Utc>>,
    last_delivery_start: Option<DateTime<Utc>>,
    first_delivery_end: Option<DateTime<Utc>>,
    last_delivery_end: Option<DateTime<Utc>>,
    min_relative_delivery_period: Option<i32>,
    max_relative_delivery_period: Option<i32>,
    data_row_count: i32,
}

#[async_trait]
impl StatisticsService for MssqlStatisticsService {
    async fn estimate_rows(&self, ids: &[Identifier], filters: &FilterSet) -> Result<u64> {
        let mut max_estimate = 0u64;

        for id in ids {
            let mdo_id = i64::from(*id);
            
            // 1. Resolve mappings to determine data category and resolutions
            let resolved_mappings = self.mapping_resolver.resolve_mappings(&[*id], DataCategory::TimeSeries, &self.stage).await?;
            if resolved_mappings.is_empty() {
                continue;
            }
            let mapping = &resolved_mappings[0];
            let category = mapping.data_category;

            // 2. Skip validation if not needed (same logic as C# IsRequestValidationNotNeeded)
            let has_rank_over = filters.nodes.iter().any(|node| matches!(node, FilterNode::RankOver(_)));
            let has_latest = filters.nodes.iter().any(|node| {
                if let FilterNode::Comparison(comp) = node {
                    comp.value.raw.to_lowercase().contains("latest")
                } else {
                    false
                }
            });
            if has_rank_over || has_latest || category == DataCategory::Surfaces {
                continue;
            }

            // 3. Fetch statistics row from the database
            let statistics_opt = self.fetch_statistics(mdo_id, category).await?;
            let stats = match statistics_opt {
                Some(s) => s,
                None => {
                    tracing::warn!("No statistics found for MdoId {}, failing open.", mdo_id);
                    continue;
                }
            };

            // 4. Determine timezone context
            let tz_id = if mapping.timezone.is_empty() {
                "Europe/Zurich"
            } else {
                &mapping.timezone
            };

            // 5. Calculate ReferenceTime Datapoints
            let has_ref_time_filters = filters.nodes.iter().any(|node| {
                if let FilterNode::Comparison(comp) = node {
                    comp.field.eq_ignore_ascii_case("ReferenceTime")
                } else {
                    false
                }
            });

            let min_ref = if !has_ref_time_filters {
                stats.last_reference_time
            } else {
                stats.first_reference_time
            };
            let max_ref = stats.last_reference_time;

            let (ref_start, ref_end) = apply_filters_to_interval(min_ref, max_ref, &filters.nodes, "ReferenceTime", tz_id);
            let ref_seconds = (ref_end - ref_start).num_seconds();
            let ref_resolution = get_resolution(&resolved_mappings, "ReferenceTime");
            let ref_datapoints = parse_to_datapoints(ref_seconds, &ref_resolution);

            // 6. Calculate Curves Datapoints (ReferenceTime * DeliveryPoints)
            let estimate = if category == DataCategory::Curves {
                let min_del = get_min_datetime();
                let max_del = get_max_datetime();
                let del_resolution = get_resolution(&resolved_mappings, "DeliveryStart");
                
                let (del_start, del_end) = apply_filters_to_interval_curves(min_del, max_del, &filters.nodes, &del_resolution, tz_id);
                let del_seconds = (del_end - del_start).num_seconds();
                let del_start_datapoints = parse_to_datapoints(del_seconds, &del_resolution);

                let (rdp_start, rdp_end) = get_rdp_datapoints(&filters.nodes, stats.min_relative_delivery_period, stats.max_relative_delivery_period);
                let mut delivery_datapoints = if rdp_start <= rdp_end {
                    (rdp_end - rdp_start) as i64 + 1
                } else {
                    1
                };

                delivery_datapoints = delivery_datapoints.min(del_start_datapoints);

                if stats.data_row_count > 0 && (stats.data_row_count as i64) < delivery_datapoints {
                    delivery_datapoints = stats.data_row_count as i64;
                }

                (ref_datapoints * delivery_datapoints) as u64
            } else {
                ref_datapoints as u64
            };

            if estimate > max_estimate {
                max_estimate = estimate;
            }
        }

        Ok(max_estimate)
    }
}

fn get_min_datetime() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(1900, 1, 1, 0, 0, 0).unwrap()
}

fn get_max_datetime() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2100, 1, 1, 0, 0, 0).unwrap()
}

fn get_resolution(mappings: &[Mapping], field_name: &str) -> String {
    let mapping_opt = mappings.iter().find(|m| {
        m.columns.iter().any(|c| c.mds_name.eq_ignore_ascii_case(field_name))
    });
    if let Some(m) = mapping_opt {
        m.resolution.clone()
    } else {
        if field_name.eq_ignore_ascii_case("ReferenceTime") {
            "P1D".to_string()
        } else {
            "PT1H".to_string()
        }
    }
}

fn parse_to_datapoints(seconds: i64, resolution: &str) -> i64 {
    let minutes = seconds / 60;
    let result = match resolution {
        "P1Y" => minutes / 365 / 24 / 60,
        "P6M" => minutes / 6 / 30 / 24 / 60,
        "P3M" => minutes / 3 / 30 / 24 / 60,
        "P1M" => minutes / 30 / 24 / 60,
        "P1W" => minutes / 7 / 24 / 60,
        "P1D" => minutes / 24 / 60,
        "PT1H" => minutes / 60,
        "PT30M" => minutes / 30,
        "PT15M" => minutes / 15,
        "PT5M" => minutes / 5,
        "PT1M" => minutes,
        "PT4S" => seconds / 4,
        _ => minutes,
    };
    if result == 0 { 1 } else { result }
}

fn adjust_delivery_end_to_start(delivery_end: DateTime<Utc>, resolution: &str) -> DateTime<Utc> {
    match resolution {
        "P1Y" => {
            let year = delivery_end.year() - 1;
            delivery_end.with_year(year).unwrap_or(delivery_end)
        }
        "P6M" => {
            let mut month = delivery_end.month() as i32 - 6;
            let mut year = delivery_end.year();
            if month <= 0 {
                month += 12;
                year -= 1;
            }
            delivery_end.with_year(year).and_then(|d| d.with_month(month as u32)).unwrap_or(delivery_end)
        }
        "P3M" => {
            let mut month = delivery_end.month() as i32 - 3;
            let mut year = delivery_end.year();
            if month <= 0 {
                month += 12;
                year -= 1;
            }
            delivery_end.with_year(year).and_then(|d| d.with_month(month as u32)).unwrap_or(delivery_end)
        }
        "P1M" => {
            let mut month = delivery_end.month() as i32 - 1;
            let mut year = delivery_end.year();
            if month <= 0 {
                month += 12;
                year -= 1;
            }
            delivery_end.with_year(year).and_then(|d| d.with_month(month as u32)).unwrap_or(delivery_end)
        }
        "P1W" => delivery_end - chrono::Duration::days(7),
        "P1D" => delivery_end - chrono::Duration::days(1),
        "PT1H" => delivery_end - chrono::Duration::hours(1),
        "PT30M" => delivery_end - chrono::Duration::minutes(30),
        "PT15M" => delivery_end - chrono::Duration::minutes(15),
        "PT5M" => delivery_end - chrono::Duration::minutes(5),
        "PT1M" => delivery_end - chrono::Duration::minutes(1),
        "PT4S" => delivery_end - chrono::Duration::seconds(4),
        _ => delivery_end - chrono::Duration::minutes(1),
    }
}

fn apply_filters_to_interval(
    mut start: DateTime<Utc>,
    mut end: DateTime<Utc>,
    filters: &[FilterNode],
    field_name: &str,
    time_zone: &str,
) -> (DateTime<Utc>, DateTime<Utc>) {
    let right_side = [">=", ">", "="];
    let left_side = ["<=", "<", "="];

    for node in filters {
        if let FilterNode::Comparison(comp) = node {
            if comp.field.eq_ignore_ascii_case(field_name) {
                if let Ok(parsed) = crate::domain::timeexpr::point_in_time::parse_point_in_time(&comp.value.raw, time_zone) {
                    if right_side.contains(&comp.operator.as_str()) && parsed > start {
                        start = parsed;
                    }
                    if left_side.contains(&comp.operator.as_str()) && parsed < end {
                        end = parsed;
                    }
                }
            }
        }
    }

    if start > end {
        (start, start)
    } else {
        (start, end)
    }
}

fn apply_filters_to_interval_curves(
    mut start: DateTime<Utc>,
    mut end: DateTime<Utc>,
    filters: &[FilterNode],
    resolution: &str,
    time_zone: &str,
) -> (DateTime<Utc>, DateTime<Utc>) {
    let right_side = [">=", ">", "="];
    let left_side = ["<=", "<", "="];

    for node in filters {
        if let FilterNode::Comparison(comp) = node {
            if comp.field.eq_ignore_ascii_case("DeliveryStart") {
                if let Ok(parsed) = crate::domain::timeexpr::point_in_time::parse_point_in_time(&comp.value.raw, time_zone) {
                    if right_side.contains(&comp.operator.as_str()) && parsed > start {
                        start = parsed;
                    }
                    if left_side.contains(&comp.operator.as_str()) && parsed < end {
                        end = parsed;
                    }
                }
            } else if comp.field.eq_ignore_ascii_case("DeliveryEnd") {
                if let Ok(parsed) = crate::domain::timeexpr::point_in_time::parse_point_in_time(&comp.value.raw, time_zone) {
                    let adjusted = adjust_delivery_end_to_start(parsed, resolution);
                    if right_side.contains(&comp.operator.as_str()) && adjusted > start {
                        start = adjusted;
                    }
                    if left_side.contains(&comp.operator.as_str()) && adjusted < end {
                        end = adjusted;
                    }
                }
            }
        }
    }

    if start > end {
        (start, start)
    } else {
        (start, end)
    }
}

fn get_rdp_datapoints(
    filters: &[FilterNode],
    min_rdp: Option<i32>,
    max_rdp: Option<i32>,
) -> (i32, i32) {
    let mut start = min_rdp.unwrap_or(i32::MIN);
    let mut end = max_rdp.unwrap_or(i32::MAX);

    let right_side = [">=", ">", "="];
    let left_side = ["<=", "<", "="];

    for node in filters {
        if let FilterNode::Comparison(comp) = node {
            if comp.field.eq_ignore_ascii_case("RelativeDeliveryPeriod") {
                if let Ok(rdp_value) = comp.value.raw.parse::<i32>() {
                    if right_side.contains(&comp.operator.as_str()) && rdp_value > start {
                        start = rdp_value;
                    }
                    if left_side.contains(&comp.operator.as_str()) && rdp_value < end {
                        end = rdp_value;
                    }
                }
            }
        }
    }

    if start > end {
        (start, start)
    } else {
        (start, end)
    }
}
