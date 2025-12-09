use axum::{Router, routing::get};
use crate::handlers::iss_handlers::*;

pub fn iss_router() -> Router {
    Router::new()
        .route("/iss/last", get(get_iss_last))
        .route("/iss/trend", get(get_iss_trend))
        .route("/iss/fetch", get(trigger_iss_fetch))
}
