mod errors;
mod config;
mod routes;
mod handlers;
mod services;
mod clients;
mod repo;
mod domain;
mod scheduler;
mod cache;
mod rate_limit;
mod utils;

use crate::routes::iss_routes::iss_router;
use crate::routes::osdr_routes::osdr_router;
use crate::routes::space_routes::space_router;

use crate::config::Config;
use crate::cache::redis_cache::RedisCache;
use crate::rate_limit::limiter::RateLimiter;

use crate::scheduler::{
    iss_scheduler::run_iss_scheduler,
    osdr_scheduler::run_osdr_scheduler,
    apod_scheduler::run_apod_scheduler,
    neo_scheduler::run_neo_scheduler,
    donki_scheduler::run_donki_scheduler,
    spacex_scheduler::run_spacex_scheduler,
};

use crate::errors::ok;

use axum::{routing::get, Router};
use chrono::Utc;
use serde::Serialize;

use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::info;
use tracing_subscriber::{EnvFilter, FmtSubscriber};


#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    now: String,
}


#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub redis: RedisCache,
    pub limiter: RateLimiter,

    pub nasa_url: String,
    pub nasa_key: String,
    pub fallback_url: String,

    pub every_osdr: u64,
    pub every_iss: u64,
    pub every_apod: u64,
    pub every_neo: u64,
    pub every_donki: u64,
    pub every_spacex: u64,
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ---------------------------
    // Logging
    // ---------------------------
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    dotenvy::dotenv().ok();

    // ---------------------------
    // Load config
    // ---------------------------
    let config = Config::from_env();

    // ---------------------------
    // PostgreSQL Pool
    // ---------------------------
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.db_url)
        .await?;

    init_db(&pool).await?;

    // ---------------------------
    // Redis + RateLimiter
    // ---------------------------
    let redis_url =
        std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://redis:6379".to_string());

    let redis = RedisCache::new(&redis_url)?;
    let limiter = RateLimiter::new(&redis_url)?;

    // ---------------------------
    // AppState
    // ---------------------------
    let state = AppState {
        pool: pool.clone(),
        redis: redis.clone(),
        limiter,

        nasa_url: config.nasa_url.clone(),
        nasa_key: config.nasa_key.clone(),
        fallback_url: config.iss_fallback_url.clone(),

        every_osdr: config.every_osdr,
        every_iss: config.every_iss,
        every_apod: config.every_apod,
        every_neo: config.every_neo,
        every_donki: config.every_donki,
        every_spacex: config.every_spacex,
    };

    // ---------------------------
    // Scheduler
    // ---------------------------
    run_iss_scheduler(state.clone());
    run_osdr_scheduler(state.clone());
    run_apod_scheduler(state.clone());
    run_neo_scheduler(state.clone());
    run_donki_scheduler(state.clone());
    run_spacex_scheduler(state.clone());

    // ---------------------------
    // Router
    // ---------------------------
    let app = Router::new()
        .route(
            "/health",
            get(|| async {
                ok(HealthResponse {
                    status: "ok",
                    now: Utc::now().to_rfc3339(),
                })
            }),
        )
        .merge(iss_router())
        .merge(osdr_router())
        .merge(space_router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", 3000)).await?;
    info!("rust_iss listening on 0.0.0.0:3000");

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}


// ------------------------------------
// DB INIT
// ------------------------------------
async fn init_db(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS iss_fetch_log(
            id BIGSERIAL PRIMARY KEY,
            fetched_at TIMESTAMPTZ NOT NULL DEFAULT now(),
            source_url TEXT NOT NULL,
            payload JSONB NOT NULL
        )"
    ).execute(pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS osdr_items(
            id BIGSERIAL PRIMARY KEY,
            dataset_id TEXT,
            title TEXT,
            status TEXT,
            updated_at TIMESTAMPTZ,
            inserted_at TIMESTAMPTZ NOT NULL DEFAULT now(),
            raw JSONB NOT NULL
        )"
    ).execute(pool).await?;

    sqlx::query(
        "CREATE UNIQUE INDEX IF NOT EXISTS ux_osdr_dataset_id
         ON osdr_items(dataset_id) WHERE dataset_id IS NOT NULL"
    ).execute(pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS space_cache(
            id BIGSERIAL PRIMARY KEY,
            source TEXT NOT NULL,
            fetched_at TIMESTAMPTZ NOT NULL DEFAULT now(),
            payload JSONB NOT NULL
        )"
    ).execute(pool).await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS ix_space_cache_source
         ON space_cache(source, fetched_at DESC)"
    ).execute(pool).await?;

    Ok(())
}
