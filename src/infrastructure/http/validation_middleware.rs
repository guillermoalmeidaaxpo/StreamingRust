use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use std::sync::Arc;
use crate::infrastructure::http::router::AppState;
use crate::domain::{Request as DomainRequest, DataCategory};
use crate::application::ports::RequestValidationStrategy;

pub async fn validation_middleware(
    State(state): State<Arc<AppState>>,
    request: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    // 1. Determine DataCategory based on path
    let category = match request.uri().path() {
        p if p.contains("/curves") => DataCategory::Curves,
        p if p.contains("/surfaces") => DataCategory::Surfaces,
        _ => DataCategory::TimeSeries,
    };

    // 2. Resolve Strategy
    let strategy = state.validation_resolver.resolve(category)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 3. Extract and parse body
    let (parts, body) = request.into_parts();
    let bytes = axum::body::to_bytes(body, usize::MAX).await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let payload: Vec<DomainRequest> = serde_json::from_slice(&bytes)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // 4. Validate Contract
    if let Err(e) = strategy.validate(&payload) {
        tracing::warn!("Validation failed: {}", e);
        return Err(StatusCode::BAD_REQUEST);
    }

    // 5. Block oversized requests (NON-STREAMING ONLY)
    // 15.1 Requirement: DataRowsNumberValidator at validation time
    let is_streaming = parts.uri.path().contains("/streaming");
    if !is_streaming {
        state.pipeline.row_validator().validate_row_count(&payload).await
            .map_err(|e| {
                tracing::warn!("Row count validation failed: {}", e);
                StatusCode::BAD_REQUEST
            })?;
    }

    // 6. Reconstruct request
    let mut request = Request::from_parts(parts, Body::from(bytes));
    request.extensions_mut().insert(payload);
    
    Ok(next.run(request).await)
}
