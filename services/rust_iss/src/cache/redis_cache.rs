use anyhow::Result;
use deadpool_redis::{redis::AsyncCommands, Config, Pool, Runtime};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Clone)]
pub struct RedisCache {
    pool: Pool,
}

impl RedisCache {
    pub fn new(url: &str) -> Result<Self> {
        let mut cfg = Config::default();
        cfg.url = Some(url.to_string());
        let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
        Ok(Self { pool })
    }

    async fn conn(&self) -> Result<deadpool_redis::Connection> {
        Ok(self.pool.get().await?)
    }

    pub async fn get_json<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>> {
        let mut conn = self.conn().await?;
        let data: Option<String> = conn.get(key).await?;
        if let Some(s) = data {
            let value = serde_json::from_str::<T>(&s)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    pub async fn set_json<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        ttl_sec: u64,
    ) -> Result<()> {
        let mut conn = self.conn().await?;
        let payload = serde_json::to_string(value)?;
        let _: () = deadpool_redis::redis::cmd("SETEX")
            .arg(key)
            .arg(ttl_sec as i64)
            .arg(payload)
            .query_async(&mut conn)
            .await?;
        Ok(())
    }
}
