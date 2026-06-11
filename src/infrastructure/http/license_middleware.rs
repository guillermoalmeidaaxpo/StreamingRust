use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use std::sync::Arc;
use crate::infrastructure::http::router::AppState;
use crate::infrastructure::auth::middleware::Principal;
use crate::domain::Request as DomainRequest;

pub async fn license_middleware(
    State(state): State<Arc<AppState>>,
    request: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    // 1. Get Principal from extensions (injected by auth_middleware)
    let principal = request.extensions().get::<Principal>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 2. Get parsed payload from extensions (injected by validation_middleware)
    let payload = request.extensions().get::<Vec<DomainRequest>>()
        .ok_or(StatusCode::BAD_REQUEST)?;

    // 3. Extract all unique IDs from the request batch
    let mut ids: Vec<_> = payload.iter()
        .flat_map(|r| r.ids.iter())
        .copied()
        .collect();
    ids.sort_unstable();
    ids.dedup();

    if ids.is_empty() {
        return Ok(next.run(request).await);
    }

    // 4. Call License API using the raw token we just validated
    state.license_validator.validate_read_access(
        &principal.raw_token,
        &ids,
        &state.meta_config.stage
    ).await.map_err(|e| {
        tracing::warn!("License validation failed: {}", e);
        StatusCode::FORBIDDEN
    })?;

    Ok(next.run(request).await)
}
