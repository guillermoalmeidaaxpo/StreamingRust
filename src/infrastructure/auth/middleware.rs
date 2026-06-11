use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, decode_header, DecodingKey, Validation, Algorithm};
use std::sync::Arc;
use crate::infrastructure::http::router::AppState;
use tokio::sync::RwLock;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Principal {
    pub sub: Option<String>,
    #[serde(default)]
    pub roles: Vec<String>,
    pub raw_token: String,
}

#[derive(Debug, Deserialize)]
struct Claims {
    sub: Option<String>,
    roles: Option<Vec<String>>,
    aud: Option<Vec<String>>, // aud can be string or array
    iss: Option<String>,
}

// Custom deserializer for aud since it can be string or array in JWT
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Aud {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Deserialize)]
struct DiscoveryDocument {
    jwks_uri: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Jwk {
    kid: String,
    n: String,
    e: String,
}

#[derive(Debug, Deserialize)]
struct JwkSet {
    keys: Vec<Jwk>,
}

pub struct JwkStore {
    keys: RwLock<HashMap<String, DecodingKey>>,
    issuer: String,
}

impl JwkStore {
    pub fn new(issuer: String) -> Self {
        Self {
            keys: RwLock::new(HashMap::new()),
            issuer,
        }
    }

    pub async fn refresh(&self) -> anyhow::Result<()> {
        let discovery_url = format!("{}/.well-known/openid-configuration", self.issuer.trim_end_matches('/'));
        let discovery: DiscoveryDocument = reqwest::get(discovery_url).await?.json().await?;
        
        let jwks: JwkSet = reqwest::get(discovery.jwks_uri).await?.json().await?;
        
        let mut keys = self.keys.write().await;
        keys.clear();
        for jwk in jwks.keys {
            if let Ok(key) = DecodingKey::from_rsa_components(&jwk.n, &jwk.e) {
                keys.insert(jwk.kid, key);
            }
        }
        Ok(())
    }

    pub async fn get_key(&self, kid: &str) -> Option<DecodingKey> {
        let keys = self.keys.read().await;
        keys.get(kid).cloned()
    }
}

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    if let Some(auth_header) = auth_header {
        if auth_header.starts_with("Bearer ") {
            let token = &auth_header[7..];
            
            let header = decode_header(token).map_err(|_| StatusCode::UNAUTHORIZED)?;
            let kid = header.kid.ok_or(StatusCode::UNAUTHORIZED)?;

            // Try to get key, if not found, refresh once (simplified caching logic)
            let mut decoding_key = state.jwk_store.get_key(&kid).await;
            if decoding_key.is_none() {
                if let Err(e) = state.jwk_store.refresh().await {
                    tracing::error!("Failed to refresh JWKS: {}", e);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
                decoding_key = state.jwk_store.get_key(&kid).await;
            }

            let key = decoding_key.ok_or(StatusCode::UNAUTHORIZED)?;

            let mut validation = Validation::new(Algorithm::RS256);
            validation.set_audience(&state.auth_config.audiences);
            validation.set_issuer(&[state.auth_config.issuer.clone()]);
            validation.validate_exp = true;

            match decode::<Claims>(token, &key, &validation) {
                Ok(token_data) => {
                    let user_roles = token_data.claims.roles.unwrap_or_default();
                    
                    // Role authorization check based on UserRolesRequirementHandler.cs
                    let has_required_role = state.auth_config.allowed_roles.iter()
                        .any(|required_role| user_roles.contains(required_role));

                    if !has_required_role && !state.auth_config.allowed_roles.is_empty() {
                        tracing::warn!("User missing required role. Present: {:?}", user_roles);
                        return Err(StatusCode::FORBIDDEN);
                    }

                    let principal = Principal {
                        sub: token_data.claims.sub,
                        roles: user_roles,
                        raw_token: token.to_string(),
                    };
                    request.extensions_mut().insert(principal);
                    
                    return Ok(next.run(request).await);
                }
                Err(e) => {
                    tracing::warn!("JWT Validation failed: {}", e);
                    return Err(StatusCode::UNAUTHORIZED);
                }
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
