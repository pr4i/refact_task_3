use anyhow::Result;
use deadpool_redis::{redis::AsyncCommands, Config, Pool, Runtime};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct RateLimiter {
    pool: Pool,
}

impl RateLimiter {
    pub fn new(url: &str) -> Result<Self> {
        let mut cfg = Config::default();
        cfg.url = Some(url.to_string());
        let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
        Ok(Self { pool })
    }

    async fn conn(&self) -> Result<deadpool_redis::Connection> {
        Ok(self.pool.get().await?)
    }

    pub async fn check(&self, key: &str, limit: u64, window_sec: u64) -> Result<bool> {
        let mut conn = self.conn().await?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs() as i64;

        let bucket = now / (window_sec as i64);
        let redis_key = format!("ratelimit:{}:{}", key, bucket);

        let count: i64 = deadpool_redis::redis::cmd("INCR")
            .arg(&redis_key)
            .query_async(&mut conn)
            .await?;

        if count == 1 {
            let _: () = deadpool_redis::redis::cmd("EXPIRE")
                .arg(&redis_key)
                .arg(window_sec as i64)
                .query_async(&mut conn)
                .await?;
        }

        Ok(count as u64 <= limit)
    }
}
