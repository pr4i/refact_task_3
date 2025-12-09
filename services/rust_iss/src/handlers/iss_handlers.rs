use axum::{extract::State};
use serde_json::Value;
use axum::response::IntoResponse;

use crate::errors::{ok, err, ApiError};
use crate::services::iss_service::IssService;
use crate::AppState;

pub async fn get_iss_last(State(state): State<AppState>)
    -> Result<impl IntoResponse, ApiError>
{
    let service = IssService::new(&state)?;
    let last = service.last(&state).await?;
    Ok(ok(last))
}

pub async fn get_iss_trend(State(state): State<AppState>)
    -> Result<impl IntoResponse, ApiError>
{
    let service = IssService::new(&state)?;
    let trend = service.trend(&state).await?;
    Ok(ok(trend))
}

pub async fn trigger_iss_fetch(State(state): State<AppState>)
    -> Result<impl IntoResponse, ApiError>
{
    let allowed = state.limiter.check("iss_fetch", 10, 60).await?;
    if !allowed {
        return Ok(err(ApiError::RateLimited));
    }

    let service = IssService::new(&state)?;
    service.fetch_and_store(&state).await?;

    Ok(ok(service.last(&state).await?))
}
