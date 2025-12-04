use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
use tracing::error;
use uuid::Uuid;

/// Формат успешного/ошибочного ответа:
/// {
///   "ok": true/false,
///   "data": {...} | null,
///   "error": { "code": "...", "message": "...", "trace_id": "..." } | null
/// }
#[derive(Debug, Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiErrorBody>,
}

#[derive(Debug, Serialize)]
pub struct ApiErrorBody {
    pub code: String,
    pub message: String,
    pub trace_id: String,
}

/// Внутренний тип ошибок
#[derive(Debug, Error)]
pub enum ApiErrorKind {
    #[error("Validation error")]
    Validation,

    #[error("Database error")]
    Db,

    #[error("Upstream error")]
    Upstream { status: Option<StatusCode> },

    #[error("Not found")]
    NotFound,

    #[error("Rate limited")]
    RateLimited,

    #[error("Internal error")]
    Internal,
}

/// Ошибка верхнего уровня, которую возвращают хендлеры
#[derive(Debug)]
pub struct ApiError {
    pub kind: ApiErrorKind,
    pub message: String,
    pub trace_id: Uuid,
}

impl ApiError {
    pub fn new(kind: ApiErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            trace_id: Uuid::new_v4(),
        }
    }

    pub fn validation(msg: impl Into<String>) -> Self {
        Self::new(ApiErrorKind::Validation, msg)
    }

    pub fn db(msg: impl Into<String>) -> Self {
        Self::new(ApiErrorKind::Db, msg)
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::new(ApiErrorKind::NotFound, msg)
    }

    pub fn upstream(status: Option<StatusCode>, msg: impl Into<String>) -> Self {
        Self::new(ApiErrorKind::Upstream { status }, msg)
    }

    pub fn rate_limited(msg: impl Into<String>) -> Self {
        Self::new(ApiErrorKind::RateLimited, msg)
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self::new(ApiErrorKind::Internal, msg)
    }

    fn code(&self) -> String {
        match &self.kind {
            ApiErrorKind::Validation => "VALIDATION_ERROR".to_string(),
            ApiErrorKind::Db => "DB_ERROR".to_string(),
            ApiErrorKind::NotFound => "NOT_FOUND".to_string(),
            ApiErrorKind::RateLimited => "RATE_LIMITED".to_string(),
            ApiErrorKind::Internal => "INTERNAL_ERROR".to_string(),
            ApiErrorKind::Upstream { status } => {
                if let Some(s) = status {
                    format!("UPSTREAM_{}", s.as_u16())
                } else {
                    "UPSTREAM_ERROR".to_string()
                }
            }
        }
    }
}

/// То, что возвращают хендлеры
pub type ApiResult<T> = Result<Json<ApiResponse<T>>, ApiError>;

/// Обёртка для успешного ответа
pub fn ok<T: Serialize>(data: T) -> Json<ApiResponse<T>> {
    Json(ApiResponse {
        ok: true,
        data: Some(data),
        error: None,
    })
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        // логируем ошибку с trace_id
        error!(
            kind = ?self.kind,
            trace_id = %self.trace_id,
            message = %self.message,
            "API error"
        );

        let body = ApiResponse::<serde_json::Value> {
            ok: false,
            data: None,
            error: Some(ApiErrorBody {
                code: self.code(),
                message: self.message.clone(),
                trace_id: self.trace_id.to_string(),
            }),
        };

        // ВАЖНО: ВСЕГДА HTTP 200, как требует ТЗ
        (StatusCode::OK, Json(body)).into_response()
    }
}
