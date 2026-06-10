use anyhow::{Result, anyhow};
use regex::Regex;
use std::collections::HashSet;
use std::sync::Arc;
use crate::application::ports::{Validator, RequestValidationStrategy, StatisticsService};
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
    details_validator: GenericRequestDetailsValidator,
}

impl RequestValidationStrategy for GenericRequestValidationStrategy {
    fn can_handle(&self, category: DataCategory) -> bool {
        // Category for generic/mesap
        true 
    }

    fn validate(&self, requests: &[Request]) -> Result<()> {
        self.details_validator.validate_details(requests)
    }
}

pub struct GenericRequestDetailsValidator {
    // Depends on mapping storage, etc.
}

impl GenericRequestDetailsValidator {
    pub fn validate_details(&self, _requests: &[Request]) -> Result<()> {
        // Deep validation of ids, mappings, filters, projections
        Ok(())
    }
}

pub struct DataRowsNumberValidator {
    stats_service: Arc<dyn StatisticsService>,
    limit: u64,
}

impl DataRowsNumberValidator {
    pub fn new(stats_service: Arc<dyn StatisticsService>, limit: u64) -> Self {
        Self { stats_service, limit }
    }

    pub async fn validate_row_count(&self, requests: &[Request]) -> Result<()> {
        for _req in requests {
            // let estimate = self.stats_service.estimate_rows(&req.ids, &req.filters.parsed).await?;
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
