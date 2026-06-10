use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Principal {
    pub sub: String,
    pub roles: Vec<String>,
}

pub async fn auth_middleware(
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    if let Some(auth_header) = auth_header {
        if auth_header.starts_with("Bearer ") {
            let _token = &auth_header[7..];
            
            // In a real implementation:
            // 1. Verify token using jsonwebtoken or openidconnect
            // 2. Extract claims and create Principal
            // 3. Insert Principal into request extensions
            
            let principal = Principal {
                sub: "user_id".to_string(),
                roles: vec!["outbound_reader".to_string()],
            };
            request.extensions_mut().insert(principal);
            
            return Ok(next.run(request).await);
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
