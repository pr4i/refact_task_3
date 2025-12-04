use axum::{
    extract::{Query, State},
};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

use crate::AppState;
use crate::errors::{ApiError, ApiResult, ok};
use crate::services::osdr_service::OsdrService;

#[derive(Serialize)]
pub struct OsdrListResponse {
    pub items: Vec<Value>,
}

#[derive(Serialize)]
pub struct OsdrSyncResponse {
    pub written: usize,
}

// GET /osdr/list?limit=20
pub async fn osdr_list(
    State(state): State<AppState>,
    Query(q): Query<HashMap<String, String>>,
) -> ApiResult<OsdrListResponse> {
    let service = OsdrService::new(&state)
        .map_err(|e| ApiError::internal(format!("Failed to init OsdrService: {e}")))?;

    let limit: i64 = q
        .get("limit")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or_else(|| {
            std::env::var("OSDR_LIST_LIMIT")
                .ok()
                .and_then(|s| s.parse::<i64>().ok())
                .unwrap_or(20)
        });

    let items = service
        .list(&state, limit)
        .await
        .map_err(|e| ApiError::db(format!("Failed to list OSDR items: {e}")))?;

    Ok(ok(OsdrListResponse { items }))
}

// GET /osdr/sync
pub async fn osdr_sync(State(state): State<AppState>) -> ApiResult<OsdrSyncResponse> {
    let service = OsdrService::new(&state)
        .map_err(|e| ApiError::internal(format!("Failed to init OsdrService: {e}")))?;

    let written = service
        .sync(&state)
        .await
        .map_err(|e| ApiError::upstream(None, format!("Failed to sync OSDR: {e}")))?;

    Ok(ok(OsdrSyncResponse { written }))
}
