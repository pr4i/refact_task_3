use anyhow::Result;
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{PgPool, Row};

pub struct SpaceCacheRepo;

impl SpaceCacheRepo {
    pub async fn write(pool: &PgPool, source: &str, payload: Value) -> Result<()> {
        sqlx::query("INSERT INTO space_cache(source, payload) VALUES ($1,$2)")
            .bind(source)
            .bind(payload)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn latest(pool: &PgPool, src: &str) -> Result<Option<Value>> {
        let row = sqlx::query(
            "SELECT fetched_at, payload FROM space_cache
             WHERE source = $1 ORDER BY id DESC LIMIT 1",
        )
        .bind(src)
        .fetch_optional(pool)
        .await?;

        if let Some(r) = row {
            let fetched_at: DateTime<Utc> = r.get("fetched_at");
            let payload: Value = r.get("payload");
            Ok(Some(serde_json::json!({
                "fetched_at": fetched_at,
                "payload": payload
            })))
        } else {
            Ok(None)
        }
    }

    pub async fn count_osdr(pool: &PgPool) -> Result<i64> {
        let row = sqlx::query("SELECT count(*) AS c FROM osdr_items")
            .fetch_one(pool)
            .await?;
        Ok(row.get::<i64, _>("c"))
    }
}
