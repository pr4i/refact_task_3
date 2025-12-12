use axum::{Router, routing::get};
use crate::{
    AppState,
    handlers::iss_handlers::{last, trend},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/iss/last", get(last))
        .route("/iss/trend", get(trend))
}
