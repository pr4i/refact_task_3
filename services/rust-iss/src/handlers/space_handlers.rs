use axum::{
    extract::{Path, Query, State},
};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

use crate::AppState;
use crate::errors::{ApiError, ApiResult, ok};
use crate::services::space_service::SpaceService;

#[derive(Serialize)]
pub struct SpaceLatestResponse {
    pub source: String,
    pub payload: Option<Value>,
}

#[derive(Serialize)]
pub struct SpaceRefreshResponse {
    pub refreshed: Vec<String>,
}

#[derive(Serialize)]
pub struct SpaceSummaryResponse {
    pub summary: Value,
}

// GET /space/:src/latest
pub async fn space_latest(
    Path(src): Path<String>,
    State(state): State<AppState>,
) -> ApiResult<SpaceLatestResponse> {
    let service = SpaceService::new()
        .map_err(|e| ApiError::internal(format!("Failed to init SpaceService: {e}")))?;

    let payload = service
        .latest(&state, &src)
        .await
        .map_err(|e| ApiError::db(format!("Failed to get latest for {src}: {e}")))?;

    Ok(ok(SpaceLatestResponse { source: src, payload }))
}

// GET /space/refresh?src=apod,neo,flr,cme,spacex
pub async fn space_refresh(
    State(state): State<AppState>,
    Query(q): Query<HashMap<String, String>>,
) -> ApiResult<SpaceRefreshResponse> {
    let service = SpaceService::new()
        .map_err(|e| ApiError::internal(format!("Failed to init SpaceService: {e}")))?;

    let list = q
        .get("src")
        .cloned()
        .unwrap_or_else(|| "apod,neo,flr,cme,spacex".to_string());

    let mut done = Vec::new();

    for s in list.split(',').map(|x| x.trim().to_lowercase()) {
        match s.as_str() {
            "apod" | "neo" | "flr" | "cme" | "spacex" => {
                // даже если refresh() вернёт Ok(()) из-за rate-limit — это норм
                let _ = service.refresh(&state, s.as_str()).await;
                done.push(s.to_string());
            }
            _ => {}
        }
    }

    Ok(ok(SpaceRefreshResponse { refreshed: done }))
}

// GET /space/summary
pub async fn space_summary(State(state): State<AppState>) -> ApiResult<SpaceSummaryResponse> {
    let service = SpaceService::new()
        .map_err(|e| ApiError::internal(format!("Failed to init SpaceService: {e}")))?;

    let summary = service
        .summary(&state)
        .await
        .map_err(|e| ApiError::db(format!("Failed to build space summary: {e}")))?;

    Ok(ok(SpaceSummaryResponse { summary }))
}
