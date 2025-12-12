use sqlx::PgPool;
use deadpool_redis::Pool as RedisPool;

use crate::rate_limit::limiter::RateLimiter;
use crate::cache::RedisCache;
use crate::config::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub redis: RedisPool,

    pub fallback_iss_url: String,
    pub nasa_url: String,
    pub nasa_key: String,

    pub every_iss: u64,
    pub every_osdr: u64,
    pub every_spacex: u64,
    pub every_apod: u64,
    pub every_neo: u64,
    pub every_donki: u64,

    pub limiter: RateLimiter,
    pub cache: RedisCache,
}

impl AppState {
    pub fn new(
        pool: PgPool,
        redis: RedisPool,
        cfg: Config,
        limiter: RateLimiter,
        cache: RedisCache,
    ) -> Self {
        Self {
            pool,
            redis,

            fallback_iss_url: cfg.fallback_iss_url,
            nasa_url: cfg.nasa_url,
            nasa_key: cfg.nasa_key,

            every_iss: cfg.every_iss,
            every_osdr: cfg.every_osdr,
            every_spacex: cfg.every_spacex,
            every_apod: cfg.every_apod,
            every_neo: cfg.every_neo,
            every_donki: cfg.every_donki,

            limiter,
            cache,
        }
    }
}
