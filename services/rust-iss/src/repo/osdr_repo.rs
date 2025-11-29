use anyhow::Result;
use sqlx::{PgPool, Row};
use chrono::{DateTime, Utc};
use serde_json::Value;

pub struct OsdrRepo;

impl OsdrRepo {
    pub async fn upsert_item(
        pool: &PgPool,
        dataset_id: Option<String>,
        title: Option<String>,
        status: Option<String>,
        updated_at: Option<DateTime<Utc>>,
        raw: Value,
    ) -> Result<()> {
        if let Some(ds) = dataset_id.clone() {
            sqlx::query(
                "INSERT INTO osdr_items (dataset_id, title, status, updated_at, raw)
                 VALUES ($1, $2, $3, $4, $5)
                 ON CONFLICT (dataset_id) DO UPDATE SET
                     title = EXCLUDED.title,
                     status = EXCLUDED.status,
                     updated_at = EXCLUDED.updated_at,
                     raw = EXCLUDED.raw"
            )
            .bind(ds)
            .bind(title)
            .bind(status)
            .bind(updated_at)
            .bind(raw)
            .execute(pool)
            .await?;
        } else {
            sqlx::query(
                "INSERT INTO osdr_items (dataset_id, title, status, updated_at, raw)
                 VALUES ($1, $2, $3, $4, $5)"
            )
            .bind::<Option<String>>(None)
            .bind(title)
            .bind(status)
            .bind(updated_at)
            .bind(raw)
            .execute(pool)
            .await?;
        }

        Ok(())
    }

    pub async fn list(pool: &PgPool, limit: i64) -> Result<Vec<Value>> {
        let rows = sqlx::query(
            "SELECT id, dataset_id, title, status, updated_at, inserted_at, raw
             FROM osdr_items
             ORDER BY inserted_at DESC
             LIMIT $1"
        )
        .bind(limit)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| {
            serde_json::json!({
                "id": r.get::<i64,_>("id"),
                "dataset_id": r.get::<Option<String>,_>("dataset_id"),
                "title": r.get::<Option<String>,_>("title"),
                "status": r.get::<Option<String>,_>("status"),
                "updated_at": r.get::<Option<DateTime<Utc>>,_>("updated_at"),
                "inserted_at": r.get::<DateTime<Utc>, _>("inserted_at"),
                "raw": r.get::<Value,_>("raw"),
            })
        }).collect())
    }
}
