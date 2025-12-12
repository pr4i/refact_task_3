use axum::{Router, routing::get};
use crate::{
    AppState,
    handlers::osdr_handlers::{osdr_sync, osdr_list},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/osdr/sync", get(osdr_sync))
        .route("/osdr/list", get(osdr_list))
}
