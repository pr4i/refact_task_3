use axum::{
    routing::get,
    Router,
    Json,
    extract::State,
};
use chrono::Utc;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

mod routes;
mod handlers;
mod services;
mod clients;
mod repo;
mod domain;
mod config;
mod scheduler;
mod cache;
mod rate_limit;
mod utils;

use crate::routes::{
    iss_routes::iss_router,
    osdr_routes::osdr_router,
    space_routes::space_router,
};

use crate::config::Config;
use crate::AppState;

#[derive(Debug, serde::Serialize)]
struct Health {
    status: &'static str,
    now: chrono::DateTime<Utc>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let cfg = Config::from_env()?;

    let state = AppState::new(&cfg).await?;
    
    // запускаем планировщики
    scheduler::start_all(state.clone());

    // собираем роутер
    let app = Router::new()
        .route("/health", get(|| async {
            Json(Health { status: "ok", now: Utc::now() })
        }))
        .merge(iss_router())
        .merge(osdr_router())
        .merge(space_router())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.port));
    println!("Backend running on http://{}", addr);

    // ✔ AXUM 0.6 правильный запуск сервера
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
