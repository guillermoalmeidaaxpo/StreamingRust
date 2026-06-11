use axum::{
    body::Body,
    extract::State,
    http::Request,
    middleware::Next,
    response::IntoResponse,
};
use std::sync::Arc;
use crate::infrastructure::http::router::AppState;
use crate::domain::{Request as DomainRequest, DataCategory};
use crate::apperr::AppError;

pub async fn validation_middleware(
    State(state): State<Arc<AppState>>,
    request: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    // 1. Determine DataCategory based on path
    let category = match request.uri().path() {
        p if p.contains("/curves") => DataCategory::Curves,
        p if p.contains("/surfaces") => DataCategory::Surfaces,
        _ => DataCategory::TimeSeries,
    };

    // 2. Resolve Strategy
    let strategy = state.validation_resolver.resolve(category)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    // 3. Extract and parse body
    let (parts, body) = request.into_parts();
    let bytes = axum::body::to_bytes(body, usize::MAX).await
        .map_err(|e| AppError::Invalid(format!("Failed to read request body: {}", e)))?;

    let payload: Vec<DomainRequest> = serde_json::from_slice(&bytes)
        .map_err(|e| AppError::Invalid(format!("Invalid request payload: {}", e)))?;

    // 4. Validate Shape Category Restriction
    for req in &payload {
        if req.filters.as_ref().and_then(|f| f.shape.as_ref()).is_some() && category != DataCategory::Curves {
            return Err(AppError::Invalid("Shape filter is only supported for data category 'Curve'.".to_string()));
        }
    }

    // 5. Validate Contract
    if let Err(e) = strategy.validate(&payload) {
        tracing::warn!("Validation failed: {}", e);
        return Err(AppError::Invalid(e.to_string()));
    }

    // 6. Block oversized requests (NON-STREAMING ONLY)
    let is_streaming = parts.uri.path().contains("/streaming");
    if !is_streaming {
        state.pipeline.row_validator().validate_row_count(&payload).await
            .map_err(|e| {
                tracing::warn!("Row count validation failed: {}", e);
                AppError::Invalid(e.to_string())
            })?;
    }

    // 7. Reconstruct request
    let mut request = Request::from_parts(parts, Body::from(bytes));
    request.extensions_mut().insert(payload);
    
    Ok(next.run(request).await)
}
