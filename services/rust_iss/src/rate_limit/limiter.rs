use anyhow::Result;
use deadpool_redis::{Pool, redis::AsyncCommands};
use std::time::Duration;

#[derive(Clone)]
pub struct RateLimiter {
    pool: Pool,
}

impl RateLimiter {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// Проверяет лимит: max_requests за window секунд
    pub async fn check(
        &self,
        key: &str,
        max_requests: u32,
        window_seconds: u64
    ) -> Result<bool> {
        let mut conn = self.pool.get().await?;

        let redis_key = format!("rate_limit:{}", key);

        // увеличиваем счетчик
        let count: i32 = conn.incr(&redis_key, 1).await?;

        // если ключ создан впервые — ставим TTL
        if count == 1 {
            let _: () = conn.expire(&redis_key, window_seconds as i64).await?;

        }

        Ok(count <= max_requests as i32)
    }
}
