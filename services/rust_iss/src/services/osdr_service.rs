use crate::{
    app_state::AppState,
    clients::osdr_client::OsdrClient,
    errors::ApiError,
    repo::osdr_repo::OsdrRepo,
};

use chrono::{DateTime, TimeZone, Utc};
use serde_json::{Map, Value};
use std::time::Duration;

pub struct OsdrService {
    client: OsdrClient,
}

impl OsdrService {
    pub fn new(state: &AppState) -> Result<Self, ApiError> {
        let timeout = Duration::from_secs(state.every_osdr);
        Ok(Self {
            client: OsdrClient::new(timeout)?,
        })
    }

    fn extract_datasets(json: &Value) -> Vec<Value> {
        if let Some(arr) = json.as_array() {
            return arr.clone();
        }

        for key in ["results", "items", "data"] {
            if let Some(arr) = json.get(key).and_then(|v| v.as_array()) {
                return arr.clone();
            }
        }

        if let Some(obj) = json.as_object() {
            let mut entries: Vec<(&String, &Value)> = obj.iter().collect();
            entries.sort_by(|(a, _), (b, _)| a.cmp(b));

            let mut out: Vec<Value> = Vec::with_capacity(entries.len());

            for (k, v) in entries {
                let rest_url = v
                    .get("REST_URL")
                    .or_else(|| v.get("rest_url"))
                    .and_then(|x| x.as_str())
                    .or_else(|| v.as_str());

                let mut m = Map::new();
                m.insert("dataset_id".to_string(), Value::String(k.clone()));
                if let Some(u) = rest_url {
                    m.insert("rest_url".to_string(), Value::String(u.to_string()));
                }
                m.insert("raw_index".to_string(), v.clone());

                out.push(Value::Object(m));
            }

            return out;
        }

        vec![]
    }

    fn parse_updated_at(ds: &Value) -> Option<DateTime<Utc>> {
        // unix (секунды)
        if let Some(ts) = ds.get("updated_at").and_then(|v| v.as_i64()) {
            return Utc.timestamp_opt(ts, 0).single();
        }
        // RFC3339 строка
        if let Some(s) = ds.get("updated_at").and_then(|v| v.as_str()) {
            if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
                return Some(dt.with_timezone(&Utc));
            }
        }
        None
    }

    // ✅ ДОБАВЛЕНО: доставать title из metadata (OSDR так и отдаёт)
    fn extract_title(ds: &Value) -> Option<String> {
        // если вдруг есть top-level title
        if let Some(t) = ds.get("title").and_then(|v| v.as_str()) {
            let t = t.trim();
            if !t.is_empty() {
                return Some(t.to_string());
            }
        }

        // OSDR: metadata -> "study title" / "project title"
        let meta = ds.get("metadata")?.as_object()?;

        for key in ["study title", "project title", "project identifier", "study identifier"] {
            if let Some(v) = meta.get(key).and_then(|v| v.as_str()) {
                let v = v.trim();
                if !v.is_empty() {
                    return Some(v.to_string());
                }
            }
        }

        None
    }

    // ✅ ДОБАВЛЕНО: updated_at из metadata ("study public release date" = unix seconds)
    fn extract_updated_at(ds: &Value) -> Option<DateTime<Utc>> {
        // сначала старое поведение
        if let Some(dt) = Self::parse_updated_at(ds) {
            return Some(dt);
        }

        let meta = ds.get("metadata")?.as_object()?;

        if let Some(ts) = meta.get("study public release date").and_then(|v| v.as_i64()) {
            return Utc.timestamp_opt(ts, 0).single();
        }

        None
    }

    pub async fn sync(&self, state: &AppState) -> Result<i64, ApiError> {
        let index_json: Value = self.client.fetch(&state.nasa_url).await?;
        let datasets = Self::extract_datasets(&index_json);

        const MAX_SYNC: usize = 50;

        // из .../v2/datasets/?format=json делаем .../v2
        let base_v2 = state
            .nasa_url
            .split("/v2/")
            .next()
            .unwrap_or(&state.nasa_url)
            .to_string()
            + "/v2";

        let mut written = 0_i64;

        for ds in datasets.into_iter().take(MAX_SYNC) {
            let dataset_id = ds
                .get("dataset_id")
                .or_else(|| ds.get("id"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            if dataset_id.is_empty() {
                continue;
            }

            let full_url = format!("{}/dataset/{}/?format=json", base_v2, dataset_id);

            let full: Value = match self.client.fetch(&full_url).await {
                Ok(v) => v,
                Err(e) => serde_json::json!({
                    "dataset_id": dataset_id,
                    "rest_url": ds.get("rest_url")
                        .and_then(|v| v.as_str())
                        .or_else(|| ds.get("raw_index").and_then(|x| x.get("REST_URL")).and_then(|v| v.as_str()))
                        .unwrap_or(""),
                    "fetch_error": e.to_string(),
                    "raw_index": ds
                }),
            };

            let full_unwrapped = full.get(&dataset_id).cloned().unwrap_or(full);

            // ✅ ИЗМЕНЕНО: title/updated_at берём с fallback из metadata
            let title = Self::extract_title(&full_unwrapped);
            let status = full_unwrapped.get("status").and_then(|v| v.as_str()).map(str::to_owned);
            let updated_at = Self::extract_updated_at(&full_unwrapped);

            OsdrRepo::upsert_item(
                &state.pool,
                Some(dataset_id),
                title,
                status,
                updated_at,
                full_unwrapped,
            )
            .await?;

            written += 1;
        }

        Ok(written)
    }

    pub async fn list(&self, state: &AppState, limit: i64) -> Result<Vec<Value>, ApiError> {
        Ok(OsdrRepo::list(&state.pool, limit).await?)
    }
}
