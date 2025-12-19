use anyhow::Result;
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{PgPool, Row};

use crate::domain::iss::IssLast;

pub struct IssRepo;

impl IssRepo {
    pub async fn insert(pool: &PgPool, source_url: &str, payload: Value) -> Result<()> {
        sqlx::query(
            "INSERT INTO iss_fetch_log (source_url, payload) VALUES ($1, $2)",
        )
        .bind(source_url)
        .bind(payload)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn last(pool: &PgPool) -> Result<Option<IssLast>> {
        let row_opt = sqlx::query(
            "SELECT id, fetched_at, source_url, payload
             FROM iss_fetch_log
             ORDER BY id DESC LIMIT 1",
        )
        .fetch_optional(pool)
        .await?;

        if let Some(row) = row_opt {
            let id: i64 = row.get("id");
            let fetched_at: DateTime<Utc> = row.get("fetched_at");
            let source_url: String = row.get("source_url");
            let payload: Value = row.try_get("payload").unwrap_or(serde_json::json!({}));
            Ok(Some(IssLast {
                id,
                fetched_at,
                source_url,
                payload,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn last_two(pool: &PgPool) -> Result<Vec<(DateTime<Utc>, Value)>> {
        let rows = sqlx::query("SELECT fetched_at, payload FROM iss_fetch_log ORDER BY id DESC LIMIT 2")
            .fetch_all(pool)
            .await?;

        let mut out = Vec::new();
        for r in rows {
            let t: DateTime<Utc> = r.get("fetched_at");
            let p: Value = r.get("payload");
            out.push((t, p));
        }
        Ok(out)
    }
    
    pub async fn last_n(pool: &PgPool, limit: i64) -> Result<Vec<(DateTime<Utc>, Value)>> {
        let rows = sqlx::query(
            "SELECT fetched_at, payload
            FROM iss_fetch_log
            ORDER BY id DESC
            LIMIT $1"
        )
        .bind(limit)
        .fetch_all(pool)
        .await?;

        let mut out = Vec::new();
        for r in rows {
            let t: DateTime<Utc> = r.get("fetched_at");
            let p: Value = r.get("payload");
            out.push((t, p));
        }
        Ok(out)
    }

}
