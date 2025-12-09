use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde_json::Value;

use crate::clients::osdr_client::OsdrClient;
use crate::repo::osdr_repo::OsdrRepo;
use crate::AppState;

pub struct OsdrService {
    client: OsdrClient,
}

impl OsdrService {
    pub fn new(_state: &AppState) -> Result<Self> {
        Ok(Self {
            client: OsdrClient::new(std::time::Duration::from_secs(30))?,
        })
    }

    pub async fn sync(&self, state: &AppState) -> Result<usize> {
        let json = self.client.fetch(&state.nasa_url).await?;

        let items = Self::extract_items(&json);

        let mut written = 0usize;

        for item in items {
            let dataset_id =
                s_pick(&item, &["dataset_id", "id", "uuid", "studyId", "accession", "osdr_id"]);
            let title = s_pick(&item, &["title", "name", "label"]);
            let status = s_pick(&item, &["status", "state", "lifecycle"]);
            let updated =
                t_pick(&item, &["updated", "updated_at", "modified", "lastUpdated", "timestamp"]);

            OsdrRepo::upsert_item(&state.pool, dataset_id, title, status, updated, item.clone())
                .await?;

            written += 1;
        }

        Ok(written)
    }

    pub async fn list(&self, state: &AppState, limit: i64) -> Result<Vec<Value>> {
        OsdrRepo::list(&state.pool, limit).await
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
                return Some(Utc.timestamp(n, 0));
            }
        }
    }
    None
}
