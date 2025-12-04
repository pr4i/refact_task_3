use anyhow::Result;
use redis::{AsyncCommands, Client};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct RateLimiter {
    client: Client,
}

impl RateLimiter {
    pub fn new(redis_url: &str) -> Result<Self> {
        let client = Client::open(redis_url)?;
        Ok(Self { client })
    }

    async fn conn(&self) -> Result<redis::aio::Connection> {
        Ok(self.client.get_async_connection().await?)
    }

    /// Простой rate-limit:
    ///  - key: логическое имя лимита (apod, neo, osdr_sync и т.п.)
    ///  - max: максимум запросов за интервал
    ///  - window_sec: длина окна в секундах
    pub async fn check(&self, key: &str, max: u32, window_sec: u64) -> Result<bool> {
        let mut conn = self.conn().await?;

        // Делаем "ведёрко" по интервалам: now / window_sec
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            / window_sec;

        let redis_key = format!("rl:{}:{}", key, now);

        let count: u32 = conn.incr(&redis_key, 1_u32).await?;

        if count == 1 {
            // первый инкремент — ставим TTL
            let _: () = conn.expire(&redis_key, window_sec as usize).await?;
        }

        Ok(count <= max)
    }
}
