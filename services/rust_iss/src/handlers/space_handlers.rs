use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};

use std::collections::HashMap;

use crate::{
    app_state::AppState,
    errors::ApiError,
    services::space_service::SpaceService,
};

/// Последние данные по конкретному источнику: apod / neo / flr / cme / spacex
pub async fn space_latest(
    Path(src): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let svc = SpaceService::new(&state)?;
    let latest = svc.latest(&state, &src).await?;

    Ok(Json(serde_json::json!({
        "source": src,
        "payload": latest
    })))
}

/// Форсированное обновление нескольких источников сразу
pub async fn space_refresh(
    Query(q): Query<HashMap<String, String>>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let list = q
        .get("src")
        .cloned()
        .unwrap_or_else(|| "apod,neo,flr,cme,spacex".to_string());

    let svc = SpaceService::new(&state)?;
    let mut done = Vec::new();

    for item in list.split(',') {
        let item = item.trim().to_lowercase();
        // не роняем всё из-за одной ошибки
        let _ = svc.refresh(&state, &item).await;
        done.push(item);
    }

    Ok(Json(serde_json::json!({
        "refreshed": done
    })))
}

/// Сводка по космическим событиям
pub async fn space_summary(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let svc = SpaceService::new(&state)?;
    let summary = svc.summary(&state).await?;

    Ok(Json(summary))
}
