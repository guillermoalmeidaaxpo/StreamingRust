use thiserror::Error;
use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde::Serialize;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid request: {0}")]
    Invalid(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Dependency error: {0}")]
    Unavailable(String),
}

#[derive(Serialize)]
pub struct ProblemDetails {
    pub title: String,
    pub status: u16,
    pub detail: String,
    #[serde(rename = "type")]
    pub error_type: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, title, error_type) = match self {
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error", "https://httpstatuses.com/500"),
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, "Not Found", "https://httpstatuses.com/404"),
            AppError::Invalid(_) => (StatusCode::BAD_REQUEST, "Bad Request", "https://httpstatuses.com/400"),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized", "https://httpstatuses.com/401"),
            AppError::Unavailable(_) => (StatusCode::SERVICE_UNAVAILABLE, "Service Unavailable", "https://httpstatuses.com/503"),
        };

        let body = Json(ProblemDetails {
            title: title.to_string(),
            status: status.as_u16(),
            detail: self.to_string(),
            error_type: error_type.to_string(),
        });

        (status, body).into_response()
    }
}
