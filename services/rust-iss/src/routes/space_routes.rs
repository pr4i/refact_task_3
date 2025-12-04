use axum::{routing::get, Router};

use crate::handlers::space_handlers::{space_latest, space_refresh, space_summary};
use crate::AppState;

pub fn space_router() -> Router<AppState> {
    Router::new()
        .route("/space/:src/latest", get(space_latest))
        .route("/space/refresh", get(space_refresh))
        .route("/space/summary", get(space_summary))
}
