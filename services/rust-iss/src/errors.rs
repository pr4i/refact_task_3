use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct ApiErrorBody {
    pub code: String,
    pub message: String,
    pub trace_id: String,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub ok: bool,
    pub data: Option<T>,
    pub error: Option<ApiErrorBody>,
}

pub type ApiResult<T> = Result<Json<ApiResponse<T>>, ApiError>;

#[derive(Debug)]
pub struct ApiError {
    pub code: &'static str,
    pub message: String,
}

impl ApiError {
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        ApiError { code, message: message.into() }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let trace_id = Uuid::new_v4().to_string();

        let body = ApiResponse::<()> {
            ok: false,
            data: None,
            error: Some(ApiErrorBody {
                code: self.code.to_string(),
                message: self.message,
                trace_id,
            }),
        };

        (StatusCode::OK, Json(body)).into_response()
    }
}

impl<T> From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError::new("INTERNAL", err.to_string())
    }
}
