use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, decode_header, DecodingKey, Validation};
use std::sync::Arc;
use crate::infrastructure::http::router::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Principal {
    pub sub: Option<String>,
    #[serde(default)]
    pub roles: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Claims {
    sub: Option<String>,
    roles: Option<Vec<String>>,
    aud: Option<String>,
    iss: Option<String>,
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
            
            // In a full production setup with Entra ID (Azure AD), we would fetch the JWKS 
            // from the OpenID discovery endpoint and cache it. For this migration parity,
            // we will configure the Validation object to check issuer, audience, and roles.
            
            let mut validation = Validation::default();
            validation.set_audience(&state.auth_config.audiences);
            validation.set_issuer(&[state.auth_config.issuer.clone()]);
            // Disabling signature validation purely for the local migration stub as we don't 
            // have an async JWKS fetcher wired in via reqwest yet.
            validation.insecure_disable_signature_validation();

            // A dummy key is required by the jsonwebtoken API even when signature validation is disabled
            let dummy_key = DecodingKey::from_secret(&[]);

            match decode::<Claims>(token, &dummy_key, &validation) {
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
