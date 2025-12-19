use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::{
    app_state::AppState,
    errors::ApiError,
    services::iss_service::IssService,
    validation::TrendLimit,
};

#[derive(serde::Deserialize)]
pub struct TrendQuery {
    pub limit: Option<u32>,
}

const RL_MAX: u32 = 120;
const RL_WINDOW: u64 = 60;

pub async fn last(State(state): State<AppState>) -> Result<Response, ApiError> {
    let ok = state
        .limiter
        .check("iss:last", RL_MAX, RL_WINDOW)
        .await
        .unwrap_or(true);

    if !ok {
        return Ok((StatusCode::TOO_MANY_REQUESTS, "rate limit").into_response());
    }

    let service = IssService::new(&state)?;
    let last = service.last(&state).await?;

    Ok(match last {
        Some(v) => Json(v).into_response(),
        None => StatusCode::NO_CONTENT.into_response(),
    })
}

pub async fn trend(
    State(state): State<AppState>,
    Query(q): Query<TrendQuery>,
) -> Result<Response, ApiError> {
    let ok = state
        .limiter
        .check("iss:trend", RL_MAX, RL_WINDOW)
        .await
        .unwrap_or(true);

    if !ok {
        return Ok((StatusCode::TOO_MANY_REQUESTS, "rate limit").into_response());
    }

    let service = IssService::new(&state)?;
    let limit = TrendLimit::new(q.limit).value();

    let trend = service.trend(&state, limit).await?;
    Ok(Json(trend).into_response())
}
