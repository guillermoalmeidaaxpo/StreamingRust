use async_trait::async_trait;
use chrono::{DateTime, Utc};
use crate::application::ports::MappingResolver;
use crate::domain::{Identifier, DataCategory, Mapping, SourceKind, MappingViews, ColumnMapping};
use anyhow::{Result, anyhow};
use tiberius::{Query, Row};
use bb8_tiberius::ConnectionManager;
use bb8::Pool;
use std::sync::Mutex;
use std::collections::HashMap;
use std::time::{Instant, Duration};

pub struct MssqlMappingResolver {
    mapping_pool: Pool<ConnectionManager>,
    mds_pool: Pool<ConnectionManager>,
    cmdp_pool: Pool<ConnectionManager>,
    limits_cache: Mutex<HashMap<(i64, DataCategory), (Instant, crate::application::quote_index::FilterLimits)>>,
    before_cache: Mutex<HashMap<(i64, i64, String), (Instant, DateTime<Utc>)>>,
}

impl MssqlMappingResolver {
    pub async fn new(
        mapping_connection_string: &str,
        mds_connection_string: &str,
        cmdp_connection_string: &str,
        max_connections: u32,
    ) -> Result<Self> {
        let mapping_config = super::get_mssql_config(mapping_connection_string).await?;
        let mapping_manager = ConnectionManager::new(mapping_config);
        let mapping_pool = Pool::builder()
            .max_size(max_connections)
            .build(mapping_manager)
            .await?;

        let mds_config = super::get_mssql_config(mds_connection_string).await?;
        let mds_manager = ConnectionManager::new(mds_config);
        let mds_pool = Pool::builder()
            .max_size(max_connections)
            .build(mds_manager)
            .await?;

        let cmdp_config = super::get_mssql_config(cmdp_connection_string).await?;
        let cmdp_manager = ConnectionManager::new(cmdp_config);
        let cmdp_pool = Pool::builder()
            .max_size(max_connections)
            .build(cmdp_manager)
            .await?;

        Ok(Self { 
            mapping_pool, 
            mds_pool,
            cmdp_pool,
            limits_cache: Mutex::new(HashMap::new()),
            before_cache: Mutex::new(HashMap::new()),
        })
    }

    fn group_by_switchover(&self, mappings: Vec<Mapping>) -> SwitchoverGroups {
        let mut mds_switchover = Vec::new();
        let mut cmdp_switchover = Vec::new();
        let mut no_switchover = Vec::new();

        for mapping in mappings {
            let lower_so = mapping.switch_over.to_lowercase();
            if lower_so.starts_with("mds") {
                mds_switchover.push(mapping);
            } else if lower_so.starts_with("cmdp") {
                cmdp_switchover.push(mapping);
            } else {
                no_switchover.push(mapping);
            }
        }

        SwitchoverGroups {
            mds_switchover,
            cmdp_switchover,
            no_switchover,
        }
    }

    async fn read_cmdp_mappings(&self, ids: &[Identifier]) -> Result<Vec<Mapping>> {
        let mut client = self.mapping_pool.get().await?;
        let placeholders: Vec<String> = (1..=ids.len()).map(|i| format!("@p{}", i)).collect();
        let query_text = format!(
            "SELECT * FROM CMDP_TO_MDS_MAPPING WHERE TIMESERIES_ID IN ({})",
            placeholders.join(",")
        );

        let mut query = Query::new(query_text);
        for id in ids {
            query.bind(*id);
        }

        let stream = query.query(&mut client).await?;
        let rows = stream.into_first_result().await?;
        
        Ok(self.build_domain_mappings_from_cmdp(rows))
    }

    async fn read_mds_domain_mappings(&self, ids: &[Identifier], category: DataCategory) -> Result<Vec<Mapping>> {
        let mut client = self.mds_pool.get().await?;
        let placeholders: Vec<String> = (1..=ids.len()).map(|i| format!("@p{}", i)).collect();
        let query_text = format!(
            "SELECT MdoId, CategoryName, ResolutionISO, ColumnName, DataType, OrderPriority, \
             KeyColumnOrdering, ValueColumnOrdering, LatestVersionView, LatestReferenceTimeView, \
             GetByCreatedOnView, GetByCreatedOnLatestReferenceTimeView, TimeZone \
             FROM [Api].[VI_MdsMappingDetails] WHERE MdoId IN ({})",
            placeholders.join(",")
        );

        let mut query = Query::new(query_text);
        for id in ids {
            query.bind(*id);
        }

        let stream = query.query(&mut client).await?;
        let rows = stream.into_first_result().await?;
        
        Ok(self.build_domain_mappings_from_mds(rows, category))
    }

    fn build_domain_mappings_from_cmdp(&self, rows: Vec<Row>) -> Vec<Mapping> {
        let mut by_id: std::collections::HashMap<Identifier, Vec<&Row>> = std::collections::HashMap::new();
        let mut order = Vec::new();

        for row in &rows {
            let id: i64 = row.get("TIMESERIES_ID").unwrap();
            let identifier = Identifier::from(id);
            if !by_id.contains_key(&identifier) {
                order.push(identifier);
            }
            by_id.entry(identifier).or_default().push(row);
        }

        let mut mappings = Vec::new();
        for id in order {
            let group = &by_id[&id];
            let first = group[0];
            
            let mut columns = Vec::new();
            for row in group {
                columns.push(ColumnMapping {
                    mds_name: row.get::<&str, _>("MDS_COLUMN_NAME").unwrap_or_default().to_string(),
                    source_name: row.get::<&str, _>("CMDP_COLUMN_NAME").unwrap_or_default().to_string(),
                    data_type: row.get::<&str, _>("DATA_TYPE").unwrap_or_default().to_string(),
                    is_key: row.get::<bool, _>("IS_KEY").unwrap_or_default(),
                    is_projectable: row.get::<bool, _>("IS_PROJECTABLE").unwrap_or_default(),
                    order_priority: row.get::<i32, _>("ORDER_PRIORITY"),
                    key_column_ordering: row.get::<i32, _>("KEY_COLUMN_ORDERING"),
                    value_column_ordering: row.get::<i32, _>("VALUE_COLUMN_ORDERING"),
                });
            }

            mappings.push(Mapping {
                id,
                data_category: self.parse_data_category(first.get("MDS_DATA_CATEGORY").unwrap_or_default()),
                cassandra_id: first.get::<&str, _>("CASSANDRA_ID").map(|s| s.to_string()),
                hyperscale_id: first.get::<i64, _>("HYPERSCALE_ID"),
                mesap_id: None,
                source: self.source_kind_from_cmdp(first),
                view_name: first.get::<&str, _>("CMDP_VIEW_NAME").unwrap_or_default().to_string(),
                views: MappingViews::default(),
                index_field: first.get::<&str, _>("CMDP_COLUMN_INDEXED").unwrap_or_default().to_string(),
                resolution: first.get::<&str, _>("RESOLUTION").unwrap_or_default().to_string(),
                switch_over: first.get::<&str, _>("SWITCHOVER").unwrap_or_default().to_string(),
                split_query: first.get::<bool, _>("SPLIT_QUERY").unwrap_or(true),
                timezone: first.try_get::<&str, _>("TIMEZONE").ok().flatten().unwrap_or_default().to_string(),
                columns,
            });
        }
        mappings
    }

    fn build_domain_mappings_from_mds(&self, rows: Vec<Row>, _fallback_category: DataCategory) -> Vec<Mapping> {
        let mut by_id: std::collections::HashMap<Identifier, Vec<&Row>> = std::collections::HashMap::new();
        let mut order = Vec::new();

        for row in &rows {
            let id: i64 = row.get("MdoId").unwrap();
            let identifier = Identifier::from(id);
            if !by_id.contains_key(&identifier) {
                order.push(identifier);
            }
            by_id.entry(identifier).or_default().push(row);
        }

        let mut mappings = Vec::new();
        for id in order {
            let group = &by_id[&id];
            let first = group[0];
            
            let mut columns = Vec::new();
            for row in group {
                columns.push(ColumnMapping {
                    mds_name: row.get::<&str, _>("ColumnName").unwrap_or_default().to_string(),
                    source_name: row.get::<&str, _>("ColumnName").unwrap_or_default().to_string(),
                    data_type: row.get::<&str, _>("DataType").unwrap_or_default().to_string(),
                    is_key: row.get::<u8, _>("KeyColumnOrdering").is_some(),
                    is_projectable: row.get::<u8, _>("KeyColumnOrdering").is_none(),
                    order_priority: row.get::<u8, _>("OrderPriority").map(|b| b as i32),
                    key_column_ordering: row.get::<u8, _>("KeyColumnOrdering").map(|b| b as i32),
                    value_column_ordering: row.get::<u8, _>("ValueColumnOrdering").map(|b| b as i32),
                });
            }

            mappings.push(Mapping {
                id,
                data_category: self.parse_data_category(first.get("CategoryName").unwrap_or_default()),
                cassandra_id: None,
                hyperscale_id: Some(id),
                mesap_id: None,
                source: SourceKind::Hyperscale,
                view_name: String::new(),
                views: MappingViews {
                    latest_version: first.get::<&str, _>("LatestVersionView").unwrap_or_default().to_string(),
                    latest_reference_time: first.get::<&str, _>("LatestReferenceTimeView").unwrap_or_default().to_string(),
                    latest_version_with_created_on: first.get::<&str, _>("LatestVersionWithCreatedOnView").unwrap_or_default().to_string(),
                    latest_reference_time_with_created_on: first.get::<&str, _>("LatestReferenceTimeWithCreatedOnView").unwrap_or_default().to_string(),
                    get_by_created_on: first.get::<&str, _>("GetByCreatedOnView").unwrap_or_default().to_string(),
                    get_by_created_on_latest_reference_time: first.get::<&str, _>("GetByCreatedOnLatestReferenceTimeView").unwrap_or_default().to_string(),
                },
                index_field: String::new(),
                resolution: first.get::<&str, _>("ResolutionISO").unwrap_or_default().to_string(),
                switch_over: String::new(),
                split_query: true,
                timezone: first.get::<&str, _>("TimeZone").unwrap_or_default().to_string(),
                columns,
            });
        }
        mappings
    }

    fn parse_data_category(&self, value: &str) -> DataCategory {
        match value.to_lowercase().as_str() {
            "curve" | "curves" => DataCategory::Curves,
            "surface" | "surfaces" => DataCategory::Surfaces,
            _ => DataCategory::TimeSeries,
        }
    }

    fn source_kind_from_cmdp(&self, row: &Row) -> SourceKind {
        if row.get::<i64, _>("HYPERSCALE_ID").is_some() {
            return SourceKind::Hyperscale;
        }
        let so: &str = row.get("SWITCHOVER").unwrap_or_default();
        if so.to_lowercase().starts_with("mds") {
            return SourceKind::Hyperscale;
        }
        if let Some(cass_id) = row.get::<&str, _>("CASSANDRA_ID") {
            if !cass_id.trim().is_empty() {
                return SourceKind::Cassandra;
            }
        }
        SourceKind::Cmdp
    }

    fn enrich_mds_mappings(&self, mds_mappings: Vec<Mapping>, originals: &[Mapping]) -> Vec<Mapping> {
        mds_mappings.into_iter().map(|mut mds| {
            if let Some(orig) = originals.iter().find(|o| o.hyperscale_id == Some(mds.id) || (o.hyperscale_id.is_none() && o.id == mds.id)) {
                mds.switch_over = orig.switch_over.clone();
                if mds.timezone.is_empty() {
                    mds.timezone = orig.timezone.clone();
                }
            }
            mds
        }).collect()
    }
}

struct SwitchoverGroups {
    mds_switchover: Vec<Mapping>,
    cmdp_switchover: Vec<Mapping>,
    no_switchover: Vec<Mapping>,
}

#[async_trait]
impl MappingResolver for MssqlMappingResolver {
    async fn resolve_mappings(&self, ids: &[Identifier], category: DataCategory, stage: &str) -> Result<Vec<Mapping>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let stage_lower = stage.to_lowercase();
        let uses_mds = stage_lower.contains("design") || stage_lower.contains("validation");
        let uses_migration = stage_lower.contains("migration");

        if uses_mds {
            return self.read_mds_domain_mappings(ids, category).await;
        }

        let cmdp_mappings = self.read_cmdp_mappings(ids).await?;
        if cmdp_mappings.is_empty() {
            return Err(anyhow!("requested identifiers do not have mappings"));
        }

        let groups = self.group_by_switchover(cmdp_mappings.clone());

        if uses_migration {
            let mut mds_ids: Vec<Identifier> = groups.cmdp_switchover.iter().map(|m| m.hyperscale_id.unwrap_or(m.id)).collect();
            mds_ids.extend(groups.no_switchover.iter().map(|m| m.id));
            
            let mut result = Vec::new();
            if !mds_ids.is_empty() {
                let mds_mappings = self.read_mds_domain_mappings(&mds_ids, category).await?;
                result.extend(self.enrich_mds_mappings(mds_mappings, &groups.cmdp_switchover));
            }
            
            for mut m in groups.mds_switchover {
                m.hyperscale_id = None;
                m.source = SourceKind::Cmdp;
                result.push(m);
            }
            Ok(result)
        } else {
            // Regular Productive Flow
            let mut mds_ids: Vec<Identifier> = groups.mds_switchover.iter().map(|m| m.hyperscale_id.unwrap_or(m.id)).collect();
            for m in &groups.no_switchover {
                if m.hyperscale_id.is_some() {
                    mds_ids.push(m.id);
                }
            }

            let mut result = Vec::new();
            if !mds_ids.is_empty() {
                let mds_mappings = self.read_mds_domain_mappings(&mds_ids, category).await?;
                result.extend(self.enrich_mds_mappings(mds_mappings, &groups.mds_switchover));
            }

            for mut m in groups.cmdp_switchover {
                m.hyperscale_id = None;
                m.source = SourceKind::Cmdp;
                result.push(m);
            }

            for m in groups.no_switchover {
                if m.hyperscale_id.is_none() {
                    result.push(m);
                }
            }
            Ok(result)
        }
    }

    async fn get_watermark(&self, _mappings: &[Mapping]) -> Result<DateTime<Utc>> {
        Ok(Utc::now())
    }

    async fn get_filter_limits(&self, ids: &[Identifier], category: DataCategory) -> Result<crate::application::quote_index::FilterLimits> {
        if ids.is_empty() {
            return Ok(crate::application::quote_index::FilterLimits::default());
        }
        let now = Instant::now();
        let key = (i64::from(ids[0]), category);

        if let Ok(cache) = self.limits_cache.lock() {
            if let Some((timestamp, cached_val)) = cache.get(&key) {
                if now.duration_since(*timestamp) < Duration::from_secs(600) {
                    return Ok(cached_val.clone());
                }
            }
        }

        let result = self.get_filter_limits_db(ids, category).await?;

        if let Ok(mut cache) = self.limits_cache.lock() {
            cache.insert(key, (now, result.clone()));
        }

        Ok(result)
    }

    async fn get_max_reference_time_before(&self, id: Identifier, reference_time: DateTime<Utc>, comparison_operator: &str, category: DataCategory) -> Result<DateTime<Utc>> {
        let now = Instant::now();
        let key = (i64::from(id), reference_time.timestamp(), comparison_operator.to_string());
        
        if let Ok(cache) = self.before_cache.lock() {
            if let Some((timestamp, cached_val)) = cache.get(&key) {
                if now.duration_since(*timestamp) < Duration::from_secs(600) {
                    return Ok(*cached_val);
                }
            }
        }

        let result = self.get_max_reference_time_before_db(id, reference_time, comparison_operator, category).await?;

        if let Ok(mut cache) = self.before_cache.lock() {
            cache.insert(key, (now, result));
        }

        Ok(result)
    }
}

impl MssqlMappingResolver {
    async fn get_filter_limits_db(&self, ids: &[Identifier], category: DataCategory) -> Result<crate::application::quote_index::FilterLimits> {
        let mut mapping_opt = self.read_cmdp_mappings(ids).await.ok().and_then(|m| if m.is_empty() { None } else { Some(m[0].clone()) });
        if mapping_opt.is_none() {
            mapping_opt = self.read_mds_domain_mappings(ids, category).await.ok().and_then(|m| if m.is_empty() { None } else { Some(m[0].clone()) });
        }

        let mapping = match mapping_opt {
            Some(m) => m,
            None => return Ok(crate::application::quote_index::FilterLimits::default()),
        };

        if mapping.source == SourceKind::Hyperscale {
            return Ok(crate::application::quote_index::FilterLimits::default());
        }

        let ref_col = mapping.columns.iter()
            .find(|c| c.mds_name.eq_ignore_ascii_case("ReferenceTime"))
            .map(|c| c.source_name.as_str())
            .unwrap_or("ReferenceTime");

        let del_col = mapping.columns.iter()
            .find(|c| c.mds_name.eq_ignore_ascii_case("DeliveryStart"))
            .map(|c| c.source_name.as_str())
            .unwrap_or("");

        let mut client = self.cmdp_pool.get().await?;
        let mut query = Query::new(
            "DECLARE @minRef DATETIMEOFFSET; \
             DECLARE @maxRef DATETIMEOFFSET; \
             EXEC [MDS].[CalculateMinMaxReferenceTimeDeliveryStart] \
                 @Id = @p1, \
                 @referenceTimeIndexedFieldName = @p2, \
                 @referenceTimeFieldName = @p3, \
                 @deliveryStartFieldName = @p4, \
                 @getMinReferenceTime = @p5, \
                 @getMaxReferenceTime = @p6, \
                 @getMinMaxDeliveryStart = @p7, \
                 @schemaQualifiedViewName = @p8, \
                 @minReferenceTime = @minRef OUTPUT, \
                 @maxReferenceTime = @maxRef OUTPUT; \
             SELECT @minRef AS minReferenceTime, @maxRef AS maxReferenceTime;"
        );
        let mdo_id: i64 = ids[0].into();
        query.bind(mdo_id);
        query.bind(mapping.index_field.as_str());
        query.bind(ref_col);
        query.bind(del_col);
        query.bind(true);
        query.bind(true);
        query.bind(false);
        query.bind(mapping.view_name.as_str());

        let stream = query.query(&mut client).await?;
        let row_opt = stream.into_row().await?;

        if let Some(row) = row_opt {
            let min_ref_opt: Option<chrono::DateTime<chrono::FixedOffset>> = row.try_get("minReferenceTime").ok().flatten();
            let max_ref_opt: Option<chrono::DateTime<chrono::FixedOffset>> = row.try_get("maxReferenceTime").ok().flatten();

            let min_reference_time = min_ref_opt.map(|dt| dt.with_timezone(&Utc));
            let max_reference_time = max_ref_opt.map(|dt| dt.with_timezone(&Utc));

            Ok(crate::application::quote_index::FilterLimits {
                min_reference_time,
                max_reference_time,
            })
        } else {
            Ok(crate::application::quote_index::FilterLimits::default())
        }
    }

    async fn get_max_reference_time_before_db(&self, id: Identifier, reference_time: DateTime<Utc>, comparison_operator: &str, category: DataCategory) -> Result<DateTime<Utc>> {
        let ids = &[id];
        let mut mapping_opt = self.read_cmdp_mappings(ids).await.ok().and_then(|m| if m.is_empty() { None } else { Some(m[0].clone()) });
        if mapping_opt.is_none() {
            mapping_opt = self.read_mds_domain_mappings(ids, category).await.ok().and_then(|m| if m.is_empty() { None } else { Some(m[0].clone()) });
        }

        let mapping = match mapping_opt {
            Some(m) => m,
            None => return Err(anyhow!("No mapping found for ID {:?}", id)),
        };

        if mapping.source == SourceKind::Hyperscale {
            return Err(anyhow!("Max reference time before is not supported for Hyperscale"));
        }

        let ref_col = mapping.columns.iter()
            .find(|c| c.mds_name.eq_ignore_ascii_case("ReferenceTime"))
            .map(|c| c.source_name.as_str())
            .unwrap_or("ReferenceTime");

        let mut client = self.cmdp_pool.get().await?;
        let mut query = Query::new(
            "DECLARE @localMaxRef DATETIMEOFFSET; \
             EXEC [MDS].[GetMaxReferenceTimeBefore] \
                 @Id = @p1, \
                 @referenceTimeIndexedFieldName = @p2, \
                 @referenceTimeFieldName = @p3, \
                 @schemaQualifiedViewName = @p4, \
                 @comparisonOperator = @p5, \
                 @inputReferenceTime = @p6, \
                 @localMaxReferenceTime = @localMaxRef OUTPUT; \
             SELECT @localMaxRef AS localMaxReferenceTime;"
        );

        let mdo_id: i64 = id.into();
        query.bind(mdo_id);
        query.bind(mapping.index_field.as_str());
        query.bind(ref_col);
        query.bind(mapping.view_name.as_str());
        query.bind(comparison_operator);
        
        let fixed_dt = reference_time.with_timezone(&chrono::FixedOffset::east_opt(0).unwrap());
        query.bind(fixed_dt);

        let stream = query.query(&mut client).await?;
        let row_opt = stream.into_row().await?;

        if let Some(row) = row_opt {
            let res_opt: Option<chrono::DateTime<chrono::FixedOffset>> = row.try_get("localMaxReferenceTime").ok().flatten();
            match res_opt {
                Some(dt) => Ok(dt.with_timezone(&Utc)),
                None => Err(anyhow!("No reference time found before {:?}", reference_time)),
            }
        } else {
            Err(anyhow!("No reference time found before {:?}", reference_time))
        }
    }
}
