use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing::{info, error};

use crate::routes::{
    iss_routes,
    osdr_routes,
    space_routes,
};


mod routes;
mod handlers;
mod services;
mod repo;
mod config;
mod cache;
mod rate_limit;
mod scheduler;
mod errors;
mod clients;
mod app_state;
mod domain;

use crate::app_state::AppState;
use crate::cache::RedisCache;
use crate::rate_limit::RateLimiter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    if let Err(e) = run().await {
        error!("FATAL ERROR in rust_iss: {e:?}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("(1) Loading config from env...");
    let cfg = config::Config::from_env()?;
    info!("(1 OK) Config loaded");

    info!("(2) Connecting to Postgres...");
    let pool = sqlx::PgPool::connect(&cfg.database_url).await?;
    info!("(2 OK) Connected to Postgres");

    info!("(3) Creating Redis pool...");
    let redis = deadpool_redis::Config::from_url(cfg.redis_url.clone())
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))?;
    info!("(3 OK) Redis pool created");

    info!("(4) Building AppState...");
    let limiter = RateLimiter::new(redis.clone());
    let cache   = RedisCache::new(redis.clone());

    let port = cfg.port;

    let state = AppState::new(
        pool,
        redis,
        cfg.clone(),
        limiter,
        cache,
    );
    info!("(4 OK) AppState ready");


    tokio::spawn(scheduler::start_schedulers(state.clone()));

    info!("(5) Building router...");
    let app = Router::new()
        .merge(iss_routes::router())
        .merge(osdr_routes::router())
        .merge(space_routes::router())
        .with_state(state.clone())
        .layer(CorsLayer::permissive());


    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("🚀 Rust server starting on http://{addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
