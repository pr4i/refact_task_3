use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Db(String),

    #[error("Upstream error: {0}")]
    Upstream(String),

    #[error("Rate limit exceeded")]
    RateLimited,

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Unknown error")]
    Unknown,
}

#[derive(Serialize)]
pub struct ApiErrorBody {
    pub code: String,
    pub message: String,
    pub trace_id: String,
}

#[derive(Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub ok: bool,
    pub data: Option<T>,
    pub error: Option<ApiErrorBody>,
}

pub type ApiResult<T> = Result<JsonResponse<T>, ApiError>;

use axum::Json;
pub type JsonResponse<T> = Json<ApiResponse<T>>;

impl ApiError {
    pub fn to_body(&self) -> ApiErrorBody {
        let (code, msg) = match self {
            ApiError::Db(e) => ("DB_ERROR", e.to_string()),
            ApiError::Upstream(e) => ("UPSTREAM_ERROR", e.to_string()),
            ApiError::RateLimited => ("RATE_LIMIT", "Too many requests".to_string()),
            ApiError::Validation(e) => ("VALIDATION_ERROR", e.to_string()),
            ApiError::Unknown => ("UNKNOWN", "Unknown error".to_string()),
        };

        ApiErrorBody {
            code: code.to_string(),
            message: msg,
            trace_id: Uuid::new_v4().to_string(),
        }
    }
}

pub fn ok<T: Serialize>(data: T) -> JsonResponse<T> {
    Json(ApiResponse {
        ok: true,
        data: Some(data),
        error: None,
    })
}

pub fn err<T: Serialize>(e: ApiError) -> JsonResponse<T> {
    Json(ApiResponse {
        ok: false,
        data: None,
        error: Some(e.to_body()),
    })
}

impl From<anyhow::Error> for ApiError {
    fn from(e: anyhow::Error) -> Self {
        ApiError::Upstream(e.to_string())
    }
}
