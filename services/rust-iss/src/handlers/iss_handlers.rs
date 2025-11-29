use axum::extract::State;
use axum::Json;
use serde_json::json;

use crate::AppState;
use crate::errors::{ApiError, ApiResult, ApiResponse};
use crate::services::iss_service::IssService;

pub async fn last_iss(State(st): State<AppState>) -> ApiResult<serde_json::Value> {
    let service = IssService::new(&st).map_err(|e| ApiError::new("INIT_SERVICE", e.to_string()))?;
    let last = service.last(&st).await.map_err(|e| ApiError::new("DB_ERROR", e.to_string()))?;

    let data = match last {
        Some(rec) => json!({
            "id": rec.id,
            "fetched_at": rec.fetched_at,
            "source_url": rec.source_url,
            "payload": rec.payload
        }),
        None => json!({"message": "no data"}),
    };

    Ok(Json(ApiResponse {
        ok: true,
        data: Some(data),
        error: None,
    }))
}

pub async fn trigger_iss(State(st): State<AppState>) -> ApiResult<serde_json::Value> {
    let service = IssService::new(&st).map_err(|e| ApiError::new("INIT_SERVICE", e.to_string()))?;
    service
        .fetch_and_store(&st)
        .await
        .map_err(|e| ApiError::new("UPSTREAM_ISS", e.to_string()))?;

    last_iss(State(st)).await
}

pub async fn iss_trend(State(st): State<AppState>) -> ApiResult<serde_json::Value> {
    let service = IssService::new(&st).map_err(|e| ApiError::new("INIT_SERVICE", e.to_string()))?;
    let trend = service
        .trend(&st)
        .await
        .map_err(|e| ApiError::new("DB_ERROR", e.to_string()))?;

    let data = json!(trend);

    Ok(Json(ApiResponse {
        ok: true,
        data: Some(data),
        error: None,
    }))
}
