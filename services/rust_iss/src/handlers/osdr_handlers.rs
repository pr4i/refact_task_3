use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;
use serde_json::Value;

use crate::{
    app_state::AppState,
    errors::ApiError,
    services::osdr_service::OsdrService,
};

#[derive(Deserialize)]
pub struct ListParams {
    pub limit: Option<i64>,
}

pub async fn osdr_sync(State(state): State<AppState>) -> Result<Json<Value>, ApiError> {
    let svc = OsdrService::new(&state)?;
    let written = svc.sync(&state).await?;
    Ok(Json(serde_json::json!({ "written": written })))
}

pub async fn osdr_list(
    State(state): State<AppState>,
    Query(p): Query<ListParams>,
) -> Result<Json<Value>, ApiError> {
    let svc = OsdrService::new(&state)?;

    let mut limit = p.limit.unwrap_or(20);
    if limit < 1 {
        limit = 1;
    }
    if limit > 200 {
        limit = 200;
    }

    let items = svc.list(&state, limit).await?;
    Ok(Json(serde_json::json!({ "items": items })))
}
