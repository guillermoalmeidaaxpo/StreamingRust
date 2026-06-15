use axum::{
    routing::{get, post},
    Router,
    middleware,
};
use std::sync::Arc;
use crate::application::pipeline::Pipeline;
use super::handlers;
use crate::infrastructure::auth::middleware::auth_middleware;
use crate::infrastructure::http::validation_middleware::validation_middleware;
use crate::infrastructure::config::{AuthConfig, MetaConfig, ExecutionConfig};
use crate::application::validator::RequestValidationStrategyResolver;
use crate::infrastructure::auth::middleware::JwkStore;
use crate::application::ports::LicenseValidator;
use crate::infrastructure::http::license_middleware::license_middleware;

pub struct AppState {
    pub pipeline: Arc<Pipeline>,
    pub auth_config: AuthConfig,
    pub meta_config: MetaConfig,
    pub execution_config: ExecutionConfig,
    pub validation_resolver: Arc<RequestValidationStrategyResolver>,
    pub jwk_store: Arc<JwkStore>,
    pub license_validator: Arc<dyn LicenseValidator>,
}

pub fn create_router(state: Arc<AppState>) -> Router {
    let api_routes = Router::new()
        .nest("/productive", transactional_routes())
        .nest("/design", transactional_routes())
        .nest("/validation", transactional_routes())
        .nest("/migration", transactional_routes())
        .route("/curves", post(handlers::transactional))
        .route("/timeseries", post(handlers::transactional))
        .route("/surfaces", post(handlers::transactional))
        .route("/curves/streaming", post(handlers::transactional_stream))
        .route("/timeseries/streaming", post(handlers::transactional_stream))
        .route("/surfaces/streaming", post(handlers::transactional_stream))
        .route("/generic", post(handlers::generic_csv))
        .route("/generic/streaming", post(handlers::generic_csv_stream))
        .route("/lite", get(handlers::lite_csv))
        .route("/productive/lite", get(handlers::lite_csv))
        .route("/design/lite", get(handlers::lite_csv))
        .route("/validation/lite", get(handlers::lite_csv))
        .layer(middleware::from_fn_with_state(state.clone(), license_middleware))
        .layer(middleware::from_fn_with_state(state.clone(), validation_middleware))
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state.clone());

    Router::new()
        .route("/health/startup", get(handlers::health))
        .route("/health/liveness", get(handlers::health))
        .route("/health/readiness", get(handlers::health))
        .nest("/api/v1", api_routes)
}

fn transactional_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/curves", post(handlers::transactional))
        .route("/timeseries", post(handlers::transactional))
        .route("/surfaces", post(handlers::transactional))
        .route("/curves/streaming", post(handlers::transactional_stream))
        .route("/timeseries/streaming", post(handlers::transactional_stream))
        .route("/surfaces/streaming", post(handlers::transactional_stream))
        .route("/generic", post(handlers::generic_csv))
        .route("/generic/streaming", post(handlers::generic_csv_stream))
}
