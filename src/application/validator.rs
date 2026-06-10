use anyhow::{Result, anyhow};
use regex::Regex;
use std::collections::HashSet;
use std::sync::Arc;
use crate::application::ports::{Validator, RequestValidationStrategy, StatisticsService, FilterParser};
use crate::domain::{Request, DataCategory};

pub struct RequestValidationStrategyResolver {
    strategies: Vec<Arc<dyn RequestValidationStrategy>>,
}

impl RequestValidationStrategyResolver {
    pub fn new(strategies: Vec<Arc<dyn RequestValidationStrategy>>) -> Self {
        Self { strategies }
    }

    pub fn resolve(&self, category: DataCategory) -> Result<Arc<dyn RequestValidationStrategy>> {
        self.strategies.iter()
            .find(|s| s.can_handle(category))
            .cloned()
            .ok_or_else(|| anyhow!("No validation strategy found for category {:?}", category))
    }
}

pub struct TransactionalDataValidationStrategy;

impl RequestValidationStrategy for TransactionalDataValidationStrategy {
    fn can_handle(&self, category: DataCategory) -> bool {
        matches!(category, DataCategory::Curves | DataCategory::Surfaces | DataCategory::TimeSeries)
    }

    fn validate(&self, requests: &[Request]) -> Result<()> {
        for request in requests {
            if request.ids.is_empty() {
                return Err(anyhow!("Request must contain at least one ID"));
            }
        }
        Ok(())
    }
}

pub struct GenericRequestValidationStrategy {
    pub details_validator: GenericRequestDetailsValidator,
}

impl RequestValidationStrategy for GenericRequestValidationStrategy {
    fn can_handle(&self, category: DataCategory) -> bool {
        // Handle categories that use generic logic
        matches!(category, DataCategory::Curves | DataCategory::TimeSeries)
    }

    fn validate(&self, requests: &[Request]) -> Result<()> {
        self.details_validator.validate_details(requests)
    }
}

pub struct GenericRequestDetailsValidator {
    parser: Arc<dyn FilterParser>,
}

impl GenericRequestDetailsValidator {
    pub fn new(parser: Arc<dyn FilterParser>) -> Self {
        Self { parser }
    }

    pub fn validate_details(&self, requests: &[Request]) -> Result<()> {
        for req in requests {
            // 15.1 Requirement: ParseFilters during validation time
            if let Some(f) = &req.filters {
                let _ = tokio::task::block_in_place(|| {
                    futures::executor::block_on(self.parser.parse(&f.expressions, &f.filter_time_zone))
                })?;
            }
        }
        Ok(())
    }
}

pub struct DataRowsNumberValidator {
    stats_service: Arc<dyn StatisticsService>,
    parser: Arc<dyn FilterParser>,
    limit: u64,
}

impl DataRowsNumberValidator {
    pub fn new(stats_service: Arc<dyn StatisticsService>, parser: Arc<dyn FilterParser>, limit: u64) -> Self {
        Self { stats_service, parser, limit }
    }

    pub async fn validate_row_count(&self, requests: &[Request]) -> Result<()> {
        for req in requests {
            // 15.1 Requirement: ParseFilters to get row estimation
            let parsed_filters = if let Some(f) = &req.filters {
                self.parser.parse(&f.expressions, &f.filter_time_zone).await?
            } else {
                crate::domain::FilterSet { expressions: vec![], nodes: vec![] }
            };

            let _estimate = self.stats_service.estimate_rows(&req.ids, &parsed_filters).await?;
            // if estimate > self.limit { return Err(anyhow!("Request size limit exceeded")); }
        }
        Ok(())
    }
}

// Keeping the original RequestValidator as the main entry point if needed
pub struct RequestValidator {
    column_name_regex: Regex,
}

impl RequestValidator {
    pub fn new() -> Self {
        Self {
            column_name_regex: Regex::new(r"^[a-zA-Z0-9]+$").unwrap(),
        }
    }
}

impl Validator for RequestValidator {
    fn validate(&self, requests: &[Request]) -> Result<()> {
        // Basic contract sanity check
        if requests.is_empty() {
            return Err(anyhow!("Invalid Request Body: Empty request"));
        }
        Ok(())
    }
}
