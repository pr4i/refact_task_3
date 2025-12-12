use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Rate limit exceeded")]
    RateLimited,

    #[error("Database error: {0}")]
    Db(#[from] sqlx::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),

            ApiError::RateLimited =>
                (StatusCode::TOO_MANY_REQUESTS, "rate limit exceeded".to_string()),

            ApiError::Db(e) =>
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),

            ApiError::Http(e) =>
                (StatusCode::BAD_GATEWAY, e.to_string()),

            ApiError::Internal(msg) =>
                (StatusCode::INTERNAL_SERVER_ERROR, msg),

            ApiError::Other(e) =>
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        (status, msg).into_response()
    }
}

/// Удобный helper для handlers
pub fn ok<T>(v: T) -> Result<T, ApiError> {
    Ok(v)
}

/// Удобный helper для handlers
pub fn err(msg: impl Into<String>) -> ApiError {
    ApiError::BadRequest(msg.into())
}
