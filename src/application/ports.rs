use async_trait::async_trait;
use chrono::{DateTime, Utc};
use crate::domain::{Identifier, DataItem, FilterSet, SourceKind, DataCategory, ExecutableQuery, Mapping, Request};
use anyhow::Result;
use futures::Stream;
use std::pin::Pin;

#[async_trait]
pub trait Repository: Send + Sync {
    async fn execute(&self, query: ExecutableQuery) -> Result<Vec<DataItem>>;
    async fn stream(&self, query: ExecutableQuery) -> Result<Pin<Box<dyn Stream<Item = Result<DataItem>> + Send>>>;
}

use super::quote_index::FilterLimits;

#[async_trait]
pub trait MappingResolver: Send + Sync {
    async fn resolve_mappings(&self, ids: &[Identifier], category: DataCategory, stage: &str) -> Result<Vec<Mapping>>;
    async fn get_watermark(&self, mappings: &[Mapping]) -> Result<DateTime<Utc>>;
    async fn get_filter_limits(&self, ids: &[Identifier], category: DataCategory) -> Result<FilterLimits>;
}

#[async_trait]
pub trait FilterParser: Send + Sync {
    async fn parse(&self, expressions: &[String], time_zone: &Option<String>) -> Result<FilterSet>;
}

#[async_trait]
pub trait LicenseValidator: Send + Sync {
    async fn validate_read_access(&self, token: &str, ids: &[Identifier], stage: &str) -> Result<()>;
}

pub trait Validator: Send + Sync {
    fn validate(&self, requests: &[Request]) -> Result<()>;
}

#[async_trait]
pub trait StatisticsService: Send + Sync {
    async fn estimate_rows(&self, ids: &[Identifier], filters: &FilterSet) -> Result<u64>;
}

pub trait RequestValidationStrategy: Send + Sync {
    fn can_handle(&self, category: DataCategory) -> bool;
    fn validate(&self, requests: &[Request]) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct RequestContext {
    pub stage: String,
    pub is_mesap_endpoint: bool,
    pub data_category: DataCategory,
}

#[derive(Debug, Clone)]
pub struct Command {
    pub ids: Vec<Identifier>,
    pub data_category: DataCategory,
    pub columns: Vec<String>,
    pub version_as_of: Option<DateTime<Utc>>,
    pub include_deleted: bool,
    pub include_identifier: bool,
    pub include_offset: bool,
    pub filter_time_zone: String,
    pub target_time_zone: String,
    pub has_aggregations: bool,
    pub has_shape: bool,
    pub filters: FilterSet,
    pub mappings: Vec<Mapping>,
    pub source: SourceKind,
    pub quote_indices: Vec<i32>,
    pub index_range: Option<crate::domain::query::IndexRange>,
}

pub struct Plan {
    pub steps: Vec<PlanStep>,
}

pub struct PlanStep {
    pub command: Command,
    pub query: ExecutableQuery,
}

#[async_trait]
pub trait Planner: Send + Sync {
    async fn build_plan(&self, ctx: RequestContext, requests: Vec<Request>) -> Result<Plan>;
}
