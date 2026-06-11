use anyhow::{Result, anyhow};
use regex::Regex;
use std::sync::Arc;
use crate::application::ports::{Validator, RequestValidationStrategy, StatisticsService, FilterParser};
use crate::domain::{Request, DataCategory, request::{VALID_MONTHS, VALID_DAYS, Shape}};
use chrono::{NaiveTime, Timelike};

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
            if let Some(f) = &request.filters {
                if let Some(shape) = &f.shape {
                    validate_shape(shape)?;
                }
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
            if let Some(f) = &req.filters {
                if let Some(shape) = &f.shape {
                    validate_shape(shape)?;
                }
                // 15.1 Requirement: ParseFilters during validation time
                let _ = tokio::task::block_in_place(|| {
                    futures::executor::block_on(self.parser.parse(&f.expressions, &f.filter_time_zone))
                })?;
            }
        }
        Ok(())
    }
}

fn validate_shape(shape: &Shape) -> Result<()> {
    // 1. Validate Months
    if let Some(months) = &shape.months {
        let mut invalid_months = Vec::new();
        for m in months {
            if !VALID_MONTHS.iter().any(|&x| x.eq_ignore_ascii_case(m)) {
                invalid_months.push(m.clone());
            }
        }
        if !invalid_months.is_empty() {
            return Err(anyhow!(
                "Invalid month abbreviation(s): {}. Valid values are: {}",
                invalid_months.join(", "),
                VALID_MONTHS.join(", ")
            ));
        }

        // Check duplicates
        let mut seen = std::collections::HashSet::new();
        let mut duplicates = Vec::new();
        for m in months {
            let norm = m.to_lowercase();
            if !seen.insert(norm) {
                duplicates.push(m.clone());
            }
        }
        if !duplicates.is_empty() {
            duplicates.dedup();
            return Err(anyhow!(
                "Duplicate month abbreviation(s) found: {}. Each month should appear only once.",
                duplicates.join(", ")
            ));
        }
    }

    // 2. Validate Days
    if let Some(days) = &shape.days {
        let mut invalid_days = Vec::new();
        for d in days {
            if !VALID_DAYS.iter().any(|&x| x.eq_ignore_ascii_case(d)) {
                invalid_days.push(d.clone());
            }
        }
        if !invalid_days.is_empty() {
            return Err(anyhow!(
                "Invalid day abbreviation(s): {}. Valid values are: {}",
                invalid_days.join(", "),
                VALID_DAYS.join(", ")
            ));
        }

        // Check duplicates
        let mut seen = std::collections::HashSet::new();
        let mut duplicates = Vec::new();
        for d in days {
            let norm = d.to_lowercase();
            if !seen.insert(norm) {
                duplicates.push(d.clone());
            }
        }
        if !duplicates.is_empty() {
            duplicates.dedup();
            return Err(anyhow!(
                "Duplicate day abbreviation(s) found: {}. Each day should appear only once.",
                duplicates.join(", ")
            ));
        }
    }

    // 3. Validate TimeRanges
    if let Some(time_ranges) = &shape.time {
        if !time_ranges.is_empty() {
            // 3a. Validate individual ranges (start < end, treating 00:00:00 as 24:00 i.e. 86400 seconds)
            for range in time_ranges {
                let start_secs = to_seconds(range.start, false);
                let end_secs = to_seconds(range.end, true);
                if start_secs >= end_secs {
                    return Err(anyhow!(
                        "Time range TimeSpan(T{}, T{}) has an invalid start/end time. Start time must be earlier than end time.",
                        range.start.format("%H:%M:%S"),
                        range.end.format("%H:%M:%S")
                    ));
                }
            }

            // 3b. Check duplicates
            let mut seen_ranges = std::collections::HashSet::new();
            let mut duplicate_ranges = Vec::new();
            for r in time_ranges {
                let key = (r.start, r.end);
                if !seen_ranges.insert(key) {
                    duplicate_ranges.push(r.clone());
                }
            }
            if !duplicate_ranges.is_empty() {
                duplicate_ranges.dedup();
                let formatted: Vec<String> = duplicate_ranges.iter()
                    .map(|r| format!("TimeSpan(T{}, T{})", r.start.format("%H:%M:%S"), r.end.format("%H:%M:%S")))
                    .collect();
                return Err(anyhow!(
                    "Duplicate time range(s) found: {}. Each time range should appear only once.",
                    formatted.join(", ")
                ));
            }

            // 3c. Check overlaps
            let mut overlaps = Vec::new();
            for i in 0..time_ranges.len() {
                for j in (i + 1)..time_ranges.len() {
                    let r1 = &time_ranges[i];
                    let r2 = &time_ranges[j];
                    if r1.start == r2.start && r1.end == r2.end {
                        continue;
                    }
                    let s1 = to_seconds(r1.start, false);
                    let e1 = to_seconds(r1.end, true);
                    let s2 = to_seconds(r2.start, false);
                    let e2 = to_seconds(r2.end, true);

                    if s1 < e2 && s2 < e1 {
                        overlaps.push((r1.clone(), r2.clone()));
                    }
                }
            }
            if !overlaps.is_empty() {
                let formatted: Vec<String> = overlaps.iter()
                    .map(|(r1, r2)| format!(
                        "TimeSpan(T{}, T{}) and TimeSpan(T{}, T{})",
                        r1.start.format("%H:%M:%S"),
                        r1.end.format("%H:%M:%S"),
                        r2.start.format("%H:%M:%S"),
                        r2.end.format("%H:%M:%S")
                    ))
                    .collect();
                return Err(anyhow!(
                    "Overlapping time range(s) found: {}. Time ranges must not overlap.",
                    formatted.join("; ")
                ));
            }
        }
    }

    Ok(())
}

fn to_seconds(t: NaiveTime, is_end: bool) -> u32 {
    let secs = t.hour() * 3600 + t.minute() * 60 + t.second();
    if is_end && secs == 0 {
        86400
    } else {
        secs
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

            let estimate = self.stats_service.estimate_rows(&req.ids, &parsed_filters).await?;
            if estimate > self.limit {
                return Err(anyhow!("The estimated number of records requested exceeds the quota ({} rows). Provide a smaller range of values for the selected filters.", self.limit));
            }
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
