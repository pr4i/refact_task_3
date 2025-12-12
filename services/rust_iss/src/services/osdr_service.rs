use crate::{
    app_state::AppState,
    clients::osdr_client::OsdrClient,
    errors::ApiError,
    repo::osdr_repo::OsdrRepo,
};

use chrono::{DateTime, TimeZone, Utc};
use serde_json::Value;
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

    pub async fn sync(&self, state: &AppState) -> Result<i64, ApiError> {
        // fetch возвращает JSON Value
        let json: Value = self.client.fetch(&state.nasa_url).await?;

        // ожидаем массив
        let datasets = json.as_array().cloned().unwrap_or_default();

        let mut written = 0_i64;

        for ds in datasets {
            let dataset_id = ds.get("dataset_id").and_then(|v| v.as_str()).map(str::to_owned);
            let title      = ds.get("title").and_then(|v| v.as_str()).map(str::to_owned);
            let status     = ds.get("status").and_then(|v| v.as_str()).map(str::to_owned);

            let updated_ts = ds.get("updated_at").and_then(|v| v.as_i64()).unwrap_or(0);
            let updated_at: Option<DateTime<Utc>> =
                Utc.timestamp_opt(updated_ts, 0).single();

            OsdrRepo::upsert_item(
                &state.pool,
                dataset_id,
                title,
                status,
                updated_at,
                ds,
            )
            .await?;

            written += 1;
        }

        Ok(written)
    }

    pub async fn list(
        &self,
        state: &AppState,
        limit: i64,
    ) -> Result<Vec<Value>, ApiError> {
        Ok(OsdrRepo::list(&state.pool, limit).await?)
    }
}
