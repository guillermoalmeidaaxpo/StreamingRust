use async_trait::async_trait;
use crate::application::ports::LicenseValidator;
use crate::domain::Identifier;
use anyhow::{Result, anyhow};
use serde::Serialize;
use reqwest::{Client, StatusCode};
use std::collections::HashMap;

pub struct HttpLicenseValidator {
    client: Client,
    base_url: String,
    authorize_path: String,
    universe_path: String,
}

#[derive(Serialize)]
struct BulkDataUniverseRequest {
    #[serde(rename = "type")]
    request_type: String,
    action: String,
    #[serde(rename = "mdoIds")]
    mdo_ids: Vec<Identifier>,
    #[serde(rename = "stageId")]
    stage_id: u8,
}

#[derive(Serialize)]
struct TimeSeriesRequest {
    identifiers: Vec<Identifier>,
    #[serde(rename = "stageId")]
    stage_id: u8,
}

impl HttpLicenseValidator {
    pub fn new(base_url: String, authorize_path: String, universe_path: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            authorize_path,
            universe_path,
        }
    }

    fn stage_id(&self, stage: &str) -> u8 {
        match stage.to_lowercase().as_str() {
            "design" => 1,
            "validation" => 2,
            _ => 3,
        }
    }
}

#[async_trait]
impl LicenseValidator for HttpLicenseValidator {
    async fn validate_read_access(&self, token: &str, ids: &[Identifier], stage: &str) -> Result<()> {
        if self.base_url == "NOT SET" || self.base_url.is_empty() {
            return Ok(());
        }

        let stage_id = self.stage_id(stage);

        // 1. Data Universe Check
        let universe_url = format!("{}/{}", self.base_url.trim_end_matches('/'), self.universe_path.trim_start_matches('/'));
        let universe_req = BulkDataUniverseRequest {
            request_type: "TransactionalDataOutbound".to_string(),
            action: "Read".to_string(),
            mdo_ids: ids.to_vec(),
            stage_id,
        };

        let uni_resp = self.client.post(&universe_url)
            .bearer_auth(token)
            .json(&universe_req)
            .send().await?;

        if uni_resp.status() == StatusCode::INTERNAL_SERVER_ERROR {
            return Err(anyhow!("An error occurred while validating the data universe"));
        }
        if uni_resp.status() == StatusCode::UNAUTHORIZED {
            return Err(anyhow!("Unauthorized"));
        }
        if !uni_resp.status().is_success() {
            return Err(anyhow!("Authorization universe service returned status {}", uni_resp.status()));
        }

        let perms: HashMap<String, bool> = uni_resp.json().await?;
        for id in ids {
            if !perms.get(&id.to_string()).copied().unwrap_or(false) {
                return Err(anyhow!("You don't have access to MDO id: {}.", id));
            }
        }

        // 2. License Check
        let authorize_url = format!("{}/{}", self.base_url.trim_end_matches('/'), self.authorize_path.trim_start_matches('/'));
        let lic_req = TimeSeriesRequest {
            identifiers: ids.to_vec(),
            stage_id,
        };

        let lic_resp = self.client.post(&authorize_url)
            .bearer_auth(token)
            .json(&lic_req)
            .send().await?;

        if lic_resp.status().is_success() {
            Ok(())
        } else if lic_resp.status() == StatusCode::FORBIDDEN || lic_resp.status() == StatusCode::UNAUTHORIZED {
            Err(anyhow!("Access denied: Missing license for one or more identifiers"))
        } else {
            Err(anyhow!("Authorization license service returned status {}", lic_resp.status()))
        }
    }
}

pub struct NoopLicenseValidator;

#[async_trait]
impl LicenseValidator for NoopLicenseValidator {
    async fn validate_read_access(&self, _token: &str, _ids: &[Identifier], _stage: &str) -> Result<()> {
        Ok(())
    }
}
