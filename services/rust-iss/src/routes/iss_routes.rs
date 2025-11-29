use axum::{routing::get, Router};

use crate::handlers::iss_handlers::{iss_trend, last_iss, trigger_iss};
use crate::AppState;

pub fn iss_router() -> Router<AppState> {
    Router::new()
        .route("/last", get(last_iss))
        .route("/fetch", get(trigger_iss))
        .route("/iss/trend", get(iss_trend))
}
