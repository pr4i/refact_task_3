use axum::{routing::get, Router};

use crate::handlers::osdr_handlers::{osdr_sync, osdr_list};
use crate::AppState;

pub fn osdr_router() -> Router<AppState> {
    Router::new()
        .route("/osdr/sync", get(osdr_sync))
        .route("/osdr/list", get(osdr_list))
}
