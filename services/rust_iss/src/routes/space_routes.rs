use axum::{Router, routing::get};
use crate::{
    AppState,
    handlers::space_handlers::{
        space_latest,
        space_refresh,
        space_summary,
    },
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/space/latest/:src", get(space_latest))
        .route("/space/refresh", get(space_refresh))
        .route("/space/summary", get(space_summary))
}
