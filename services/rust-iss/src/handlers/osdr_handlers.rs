use axum::extract::{Query, State};
use axum::Json;
use serde_json::json;

use crate::AppState;
use crate::errors::{ApiError, ApiResult, ApiResponse};
use crate::services::osdr_service::OsdrService;

pub async fn osdr_sync(State(st): State<AppState>) -> ApiResult<serde_json::Value> {
    let service = OsdrService::new(&st)
        .map_err(|e| ApiError::new("INIT_SERVICE", e.to_string()))?;

    let written = service
        .sync(&st)
        .await
        .map_err(|e| ApiError::new("OSDR_SYNC_ERROR", e.to_string()))?;

    Ok(Json(ApiResponse {
        ok: true,
        data: Some(json!({ "written": written })),
        error: None,
    }))
}

#[derive(serde::Deserialize)]
pub struct OsdrListParams {
    pub limit: Option<i64>,
}

pub async fn osdr_list(
    Query(params): Query<OsdrListParams>,
    State(st): State<AppState>,
) -> ApiResult<serde_json::Value> {
    let service =
        OsdrService::new(&st).map_err(|e| ApiError::new("INIT_SERVICE", e.to_string()))?;

    let limit = params.limit.unwrap_or(20);

    let items = service
        .list(&st, limit)
        .await
        .map_err(|e| ApiError::new("OSDR_LIST_ERROR", e.to_string()))?;

    Ok(Json(ApiResponse {
        ok: true,
        data: Some(json!({ "items": items })),
        error: None,
    }))
}
