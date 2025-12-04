use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde_json::Value;
use std::time::Duration;

use crate::AppState;
use crate::clients::osdr_client::OsdrClient;
use crate::repo::osdr_repo::OsdrRepo;

pub struct OsdrService {
    client: OsdrClient,
}

impl OsdrService {
    pub fn new(_state: &AppState) -> Result<Self> {
        Ok(Self {
            client: OsdrClient::new(std::time::Duration::from_secs(30))?,
        })
    }

    // ---------------------------------------------------------
    // SYNC (пишет данные в БД, сбрасывает кэш, защищён rate-limit)
    // ---------------------------------------------------------
    pub async fn sync(&self, state: &AppState) -> Result<usize> {
        // не больше 5 синхронизаций за 5 минут
        if !state.limiter.check("osdr_sync", 5, 300).await? {
            // лимит превышен — тихо возвращаем 0
            return Ok(0);
        }

        let json = self.client.fetch(&state.nasa_url).await?;
        let items = Self::extract_items(&json);
        let mut written = 0usize;

        // Инвалидация кэша списков
        let _ = state.redis.delete("osdr:list:20").await;
        let _ = state.redis.delete("osdr:list:50").await;
        let _ = state.redis.delete("osdr:list:100").await;

        for item in items {
            let dataset_id = s_pick(&item, &[
                "dataset_id","id","uuid","studyId","accession","osdr_id"
            ]);

            let title = s_pick(&item, &["title","name","label"]);
            let status = s_pick(&item, &["status","state","lifecycle"]);
            let updated = t_pick(&item, &[
                "updated","updated_at","modified","lastUpdated","timestamp"
            ]);

            OsdrRepo::upsert_item(
                &state.pool,
                dataset_id,
                title,
                status,
                updated,
                item.clone()
            ).await?;

            written += 1;
        }

        Ok(written)
    }

    // ---------------------------------------------------------
    // LIST (с Redis-кэшем)
    // ---------------------------------------------------------
    pub async fn list(&self, state: &AppState, limit: i64) -> Result<Vec<Value>> {
        let key = format!("osdr:list:{limit}");

        // 1. Чтение из Redis
        if let Some(cached) = state.redis.get_json::<Vec<Value>>(&key).await? {
            return Ok(cached);
        }

        // 2. Чтение из PostgreSQL
        let items = OsdrRepo::list(&state.pool, limit).await?;

        // 3. Кладём в Redis на 60 секунд
        state.redis
            .set_json(&key, &items, Duration::from_secs(60))
            .await?;

        Ok(items)
    }

    fn extract_items(json: &Value) -> Vec<Value> {
        if let Some(a) = json.as_array() {
            a.clone()
        } else if let Some(a) = json.get("items").and_then(|v| v.as_array()) {
            a.clone()
        } else if let Some(a) = json.get("results").and_then(|v| v.as_array()) {
            a.clone()
        } else {
            vec![json.clone()]
        }
    }
}

// =============================================================
// Utilities
// =============================================================

fn s_pick(v: &Value, keys: &[&str]) -> Option<String> {
    for k in keys {
        if let Some(x) = v.get(*k) {
            if let Some(s) = x.as_str() {
                if !s.is_empty() {
                    return Some(s.to_string());
                }
            } else if x.is_number() {
                return Some(x.to_string());
            }
        }
    }
    None
}

fn t_pick(v: &Value, keys: &[&str]) -> Option<DateTime<Utc>> {
    for k in keys {
        if let Some(x) = v.get(*k) {
            if let Some(s) = x.as_str() {
                if let Ok(dt) = s.parse::<DateTime<Utc>>() {
                    return Some(dt);
                }
                if let Ok(ndt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
                    return Some(Utc.from_utc_datetime(&ndt));
                }
            } else if let Some(n) = x.as_i64() {
                return Some(Utc.timestamp_opt(n, 0).single().unwrap_or_else(Utc::now));
            }
        }
    }
    None
}
