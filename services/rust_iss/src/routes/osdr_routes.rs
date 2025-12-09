use axum::{Router, routing::get};
use crate::handlers::osdr_handlers::*;

pub fn osdr_router() -> Router {
    Router::new()
        .route("/osdr/sync", get(osdr_sync))
        .route("/osdr/list", get(osdr_list))
}
