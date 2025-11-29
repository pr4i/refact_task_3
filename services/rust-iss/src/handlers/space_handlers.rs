use axum::extract::{Query, Path, State};
use axum::Json;
use serde::Deserialize;
use serde_json::json;

use crate::{AppState, errors::{ApiResult, ApiError, ApiResponse}};
use crate::services::space_service::SpaceService;

pub async fn space_latest(
    Path(src): Path<String>,
    State(st): State<AppState>,
) -> ApiResult<serde_json::Value> {
    let service = SpaceService::new()
        .map_err(|e| ApiError::new("INIT_SERVICE", e.to_string()))?;

    let v = service.latest(&st, &src)
        .await
        .map_err(|e| ApiError::new("SPACE_LATEST_ERROR", e.to_string()))?;

    Ok(Json(ApiResponse {
        ok: true,
        data: Some(json!({
            "source": src,
            "data": v
        })),
        error: None,
    }))
}

#[derive(Deserialize)]
pub struct RefreshQuery {
    pub src: Option<String>,
}

pub async fn space_refresh(
    Query(q): Query<RefreshQuery>,
    State(st): State<AppState>,
) -> ApiResult<serde_json::Value>
{
    let service = SpaceService::new()
        .map_err(|e| ApiError::new("INIT_SERVICE", e.to_string()))?;

    let src_list = q.src.unwrap_or("apod,neo,flr,cme,spacex".into());
    let mut refreshed = Vec::new();

    for s in src_list.split(',').map(|x| x.trim().to_lowercase()) {
        service.refresh(&st, &s)
            .await
            .ok();
        refreshed.push(s);
    }

    Ok(Json(ApiResponse {
        ok: true,
        data: Some(json!({ "refreshed": refreshed })),
        error: None,
    }))
}

pub async fn space_summary(
    State(st): State<AppState>,
) -> ApiResult<serde_json::Value>
{
    let service = SpaceService::new()
        .map_err(|e| ApiError::new("INIT_SERVICE", e.to_string()))?;

    let data = service.summary(&st)
        .await
        .map_err(|e| ApiError::new("SPACE_SUMMARY_ERROR", e.to_string()))?;

    Ok(Json(ApiResponse {
        ok: true,
        data: Some(data),
        error: None,
    }))
}
