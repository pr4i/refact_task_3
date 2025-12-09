use axum::{Router, routing::get};
use crate::handlers::space_handlers::*;

pub fn space_router() -> Router {
    Router::new()
        .route("/space/:src/latest", get(space_latest))
        .route("/space/refresh", get(space_refresh))
        .route("/space/summary", get(space_summary))
}
