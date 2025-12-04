use axum::extract::State;
use serde::Serialize;

use crate::AppState;
use crate::domain::iss::{IssLast, IssTrend};
use crate::errors::{ApiError, ApiResult, ok};
use crate::services::iss_service::IssService;

#[derive(Serialize)]
pub struct IssLastResponse {
    pub last: Option<IssLast>,
}

#[derive(Serialize)]
pub struct IssTrendResponse {
    pub trend: IssTrend,
}

// GET /last
pub async fn last_iss(State(state): State<AppState>) -> ApiResult<IssLastResponse> {
    let service = IssService::new(&state)
        .map_err(|e| ApiError::internal(format!("Failed to init IssService: {e}")))?;

    let last = service
        .last(&state)
        .await
        .map_err(|e| ApiError::db(format!("Failed to load last ISS row: {e}")))?;

    Ok(ok(IssLastResponse { last }))
}

// GET /iss/trend
pub async fn iss_trend(State(state): State<AppState>) -> ApiResult<IssTrendResponse> {
    let service = IssService::new(&state)
        .map_err(|e| ApiError::internal(format!("Failed to init IssService: {e}")))?;

    let trend = service
        .trend(&state)
        .await
        .map_err(|e| ApiError::db(format!("Failed to compute ISS trend: {e}")))?;

    Ok(ok(IssTrendResponse { trend }))
}

// GET /fetch — ручной триггер обновления + возврат последнего значения
pub async fn trigger_iss(State(state): State<AppState>) -> ApiResult<IssLastResponse> {
    let service = IssService::new(&state)
        .map_err(|e| ApiError::internal(format!("Failed to init IssService: {e}")))?;

    service
        .fetch_and_store(&state)
        .await
        .map_err(|e| ApiError::upstream(None, format!("Failed to fetch ISS data: {e}")))?;

    let last = service
        .last(&state)
        .await
        .map_err(|e| ApiError::db(format!("Failed to load last ISS row after fetch: {e}")))?;

    Ok(ok(IssLastResponse { last }))
}
