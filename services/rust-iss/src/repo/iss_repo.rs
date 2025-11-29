use anyhow::Result;
use serde_json::Value;
use sqlx::{PgPool, Row};
use chrono::{DateTime, Utc};

use crate::domain::iss::IssLast;

pub struct IssRepo;

impl IssRepo {
    pub async fn insert(pool: &PgPool, source_url: &str, payload: Value) -> Result<()> {
        sqlx::query("INSERT INTO iss_fetch_log (source_url, payload) VALUES ($1, $2)")
            .bind(source_url)
            .bind(payload)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn last(pool: &PgPool) -> Result<Option<IssLast>> {
        let row = sqlx::query(
            "SELECT id, fetched_at, source_url, payload
             FROM iss_fetch_log
             ORDER BY id DESC LIMIT 1",
        )
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| IssLast {
            id: r.get("id"),
            fetched_at: r.get::<DateTime<Utc>, _>("fetched_at"),
            source_url: r.get::<String, _>("source_url"),
            payload: r.get::<Value, _>("payload"),
        }))
    }

    pub async fn last_two(pool: &PgPool) -> Result<Vec<(DateTime<Utc>, Value)>> {
        let rows = sqlx::query(
            "SELECT fetched_at, payload
             FROM iss_fetch_log
             ORDER BY id DESC LIMIT 2",
        )
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| {
                (
                    r.get::<DateTime<Utc>, _>("fetched_at"),
                    r.get::<Value, _>("payload"),
                )
            })
            .collect())
    }
}
