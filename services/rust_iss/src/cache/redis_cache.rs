use anyhow::Result;
use deadpool_redis::{redis::AsyncCommands, Pool};
use serde_json::Value;

#[derive(Clone)]
pub struct RedisCache {
    pub pool: Pool,
}

impl RedisCache {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn set_json(&self, key: &str, json: Value, ttl_seconds: usize) -> Result<()> {
        let mut conn = self.pool.get().await?;
        let s = serde_json::to_string(&json)?;
        let _: () = conn.set_ex(key, s, ttl_seconds as u64).await?;
        Ok(())
    }

    pub async fn get_json(&self, key: &str) -> Result<Option<Value>> {
        let mut conn = self.pool.get().await?;
        let s: Option<String> = conn.get(key).await?;
        Ok(s.map(|x| serde_json::from_str(&x)).transpose()?)
    }
}
