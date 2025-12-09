use axum::{extract::State, response::IntoResponse, Json};
use crate::{AppState, errors::ApiError, services::osdr_service::OsdrService};

pub async fn osdr_sync(
    State(state): State<AppState>
) -> Result<impl IntoResponse, ApiError>
{
    let svc = OsdrService::new(&state)?;
    let written = svc.sync(&state).await?;
    Ok(Json(serde_json::json!({ "written": written })))
}

pub async fn osdr_list(
    State(state): State<AppState>
) -> Result<impl IntoResponse, ApiError>
{
    let limit = std::env::var("OSDR_LIST_LIMIT")
        .ok()
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(20);

    let svc = OsdrService::new(&state)?;
    let items = svc.list(&state, limit).await?;

    Ok(Json(serde_json::json!({ "items": items })))
}
