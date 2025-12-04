use redis::{AsyncCommands, Client};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use anyhow::Result;
use std::time::Duration;

#[derive(Clone)]
pub struct RedisCache {
    client: Client,
}

impl RedisCache {
    pub fn new(url: &str) -> Result<Self> {
        let client = Client::open(url)?;
        Ok(Self { client })
    }

    async fn conn(&self) -> Result<redis::aio::Connection> {
        Ok(self.client.get_async_connection().await?)
    }

    pub async fn set_json<T: Serialize>(&self, key: &str, value: &T, ttl: Duration) -> Result<()> {
        let mut conn = self.conn().await?;
        let payload = serde_json::to_string(value)?;
        let _: () = redis::cmd("SET").arg(key).arg(payload).arg("EX").arg(ttl.as_secs()).query_async(&mut conn).await?;
        Ok(())
    }

    pub async fn get_json<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>> {
        let mut conn = self.conn().await?;
        let data: Option<String> = conn.get(key).await?;
        if let Some(s) = data {
            let parsed = serde_json::from_str(&s)?;
            Ok(Some(parsed))
        } else {
            Ok(None)
        }
    }

    pub async fn exists(&self, key: &str) -> Result<bool> {
        let mut conn = self.conn().await?;
        let val: i32 = conn.exists(key).await?;
        Ok(val == 1)
    }

    pub async fn delete(&self, key: &str) -> Result<()> {
        let mut conn = self.conn().await?;
        let _: () = conn.del(key).await?;
        Ok(())
    }
}
