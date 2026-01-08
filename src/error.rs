use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error: {0}")]
    InternalError(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("RSS fetch error: {0}")]
    RssFetchError(String),

    #[error("RSS parse error: {0}")]
    RssParseError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            ApiError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            ApiError::Timeout(msg) => (StatusCode::REQUEST_TIMEOUT, msg.clone()),
            ApiError::RssFetchError(msg) => (StatusCode::BAD_GATEWAY, msg.clone()),
            ApiError::RssParseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        };

        (status, Json(json!({"status": "error", "message": message}))).into_response()
    }
}
