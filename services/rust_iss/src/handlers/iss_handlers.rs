use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
    http::StatusCode,
};

use crate::{
    app_state::AppState,
    services::iss_service::IssService,
    errors::ApiError,
};

pub async fn last(State(state): State<AppState>) -> Result<Response, ApiError> {
    let service = IssService::new(&state)?;
    let last = service.last(&state).await?;

    Ok(match last {
        Some(v) => Json(v).into_response(),
        None => StatusCode::NO_CONTENT.into_response(),
    })
}

pub async fn trend(State(state): State<AppState>) -> Result<Response, ApiError> {
    let service = IssService::new(&state)?;
    let trend = service.trend(&state).await?;
    Ok(Json(trend).into_response())
}
