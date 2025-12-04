use axum::{routing::get, Router};

use crate::handlers::osdr_handlers::{osdr_list, osdr_sync};
use crate::AppState;

pub fn osdr_router() -> Router<AppState> {
    Router::new()
        .route("/osdr/list", get(osdr_list))
        .route("/osdr/sync", get(osdr_sync))
}
