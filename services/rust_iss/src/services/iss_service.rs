use crate::{
    app_state::AppState,
    clients::iss_client::IssClient,
    errors::ApiError,
    repo::iss_repo::IssRepo,
};

use serde_json::Value;
use std::time::Duration;

pub struct IssService {
    client: IssClient,
}

impl IssService {
    pub fn new(state: &AppState) -> Result<Self, ApiError> {
        let timeout = Duration::from_secs(state.every_iss);

        Ok(Self {
            client: IssClient::new(timeout)?,
        })
    }

    pub async fn fetch_and_store(&self, state: &AppState) -> Result<(), ApiError> {
        let json: Value = self
            .client
            .fetch(&state.fallback_iss_url)
            .await?;

        IssRepo::insert(&state.pool, &state.fallback_iss_url, json).await?;

        Ok(())
    }

    pub async fn last(
        &self,
        state: &AppState,
    ) -> Result<Option<crate::domain::iss::IssLast>, ApiError> {
        Ok(IssRepo::last(&state.pool).await?)
    }

    pub async fn trend(
        &self,
        state: &AppState,
    ) -> Result<Vec<(chrono::DateTime<chrono::Utc>, Value)>, ApiError> {
        Ok(IssRepo::last_two(&state.pool).await?)
    }
}
