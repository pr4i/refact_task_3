use anyhow::Result;
use sqlx::{PgPool, Row};
use chrono::{DateTime, Utc};
use serde_json::Value;

pub struct SpaceCacheRepo;

impl SpaceCacheRepo {
    pub async fn write(pool: &PgPool, source: &str, payload: Value) -> Result<()> {
        sqlx::query("INSERT INTO space_cache (source, payload) VALUES ($1, $2)")
            .bind(source)
            .bind(payload)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn latest(pool: &PgPool, source: &str) -> Result<Option<Value>> {
        let row = sqlx::query(
            "SELECT fetched_at, payload
             FROM space_cache
             WHERE source = $1
             ORDER BY id DESC
             LIMIT 1"
        )
        .bind(source)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| {
            serde_json::json!({
                "at": r.get::<DateTime<Utc>, _>("fetched_at"),
                "payload": r.get::<Value, _>("payload")
            })
        }))
    }

    pub async fn count_osdr(pool: &PgPool) -> Result<i64> {
        let row = sqlx::query("SELECT count(*) AS c FROM osdr_items")
            .fetch_one(pool)
            .await?;
        Ok(row.get::<i64, _>("c"))
    }
}
