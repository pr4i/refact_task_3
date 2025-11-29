mod errors;
mod config;
mod routes;
mod handlers;
mod services;
mod clients;
mod repo;
mod domain;
mod scheduler;

use crate::routes::iss_routes::iss_router;
use crate::routes::osdr_routes::osdr_router;
use crate::routes::space_routes::space_router;
use crate::config::Config;
use crate::scheduler::{
    iss_scheduler::run_iss_scheduler,
    osdr_scheduler::run_osdr_scheduler,
    apod_scheduler::run_apod_scheduler,
    neo_scheduler::run_neo_scheduler,
    donki_scheduler::run_donki_scheduler,
    spacex_scheduler::run_spacex_scheduler,
};

use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Serialize;
use serde_json::Value;
use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use std::{collections::HashMap, time::Duration};
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[derive(Serialize)]
struct Health {
    status: &'static str,
    now: DateTime<Utc>,
}

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
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
    // Logging
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);

    dotenvy::dotenv().ok();

    // Load config
    let config = Config::from_env();

    // Create DB pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.db_url)
        .await?;
    init_db(&pool).await?;

    // Global state
    let state = AppState {
        pool: pool.clone(),
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
    //   ФОНОВЫЕ ЗАДАЧИ (scheduler)
    // ---------------------------
    run_iss_scheduler(state.clone());
    run_osdr_scheduler(state.clone());
    run_apod_scheduler(state.clone());
    run_neo_scheduler(state.clone());
    run_donki_scheduler(state.clone());
    run_spacex_scheduler(state.clone());

    // ---------------------------
    //         ROUTER
    // ---------------------------
    let app = Router::new()
        .route(
            "/health",
            get(|| async {
                Json(Health {
                    status: "ok",
                    now: Utc::now(),
                })
            }),
        )
        .merge(iss_router())   // ISS вынесен в архитектуру
        .merge(osdr_router())  // OSDR полностью вынесен в архитектуру
        .merge(space_router()) // Space cache/APOD/NEO/DONKI/SpaceX
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", 3000)).await?;
    info!("rust_iss listening on 0.0.0.0:3000");
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
