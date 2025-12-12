use crate::{
    app_state::AppState,
    clients::{
        apod_client::ApodClient,
        neo_client::NeoClient,
        donki_client::DonkiClient,
        spacex_client::SpacexClient,
    },
    repo::space_cache_repo::SpaceCacheRepo,
    errors::ApiError,
};

use serde_json::Value;
use std::time::Duration;

pub struct SpaceService {
    apod: ApodClient,
    neo: NeoClient,
    donki: DonkiClient,
    spacex: SpacexClient,
}

impl SpaceService {
    pub fn new(state: &AppState) -> Result<Self, ApiError> {
        let timeout = Duration::from_secs(state.every_spacex);

        Ok(Self {
            apod: ApodClient::new(timeout)?,
            neo: NeoClient::new(timeout)?,
            donki: DonkiClient::new(timeout)?,
            spacex: SpacexClient::new(timeout)?,
        })
    }

    pub async fn refresh(
        &self,
        state: &AppState,
        src: &str,
    ) -> Result<(), ApiError> {
        match src {
            "apod" => {
                let v = self.apod.fetch(&state.nasa_key).await?;
                SpaceCacheRepo::write(&state.pool, "apod", v).await?;
            }
            "neo" => {
                let v = self.neo.fetch(&state.nasa_key).await?;
                SpaceCacheRepo::write(&state.pool, "neo", v).await?;
            }
            "flr" => {
                let v = self.donki.fetch_flr(&state.nasa_key).await?;
                SpaceCacheRepo::write(&state.pool, "flr", v).await?;
            }
            "cme" => {
                let v = self.donki.fetch_cme(&state.nasa_key).await?;
                SpaceCacheRepo::write(&state.pool, "cme", v).await?;
            }
            "spacex" => {
                let v = self.spacex.fetch().await?;
                SpaceCacheRepo::write(&state.pool, "spacex", v).await?;
            }
            _ => {}
        }

        Ok(())
    }

    pub async fn latest(
        &self,
        state: &AppState,
        src: &str,
    ) -> Result<Option<Value>, ApiError> {
        Ok(SpaceCacheRepo::latest(&state.pool, src).await?)
    }

    pub async fn summary(
        &self,
        state: &AppState,
    ) -> Result<Value, ApiError> {
        let osdr_count = SpaceCacheRepo::count_osdr(&state.pool).await?;

        Ok(serde_json::json!({
            "osdr_count": osdr_count,
            "apod":   SpaceCacheRepo::latest(&state.pool, "apod").await?,
            "neo":    SpaceCacheRepo::latest(&state.pool, "neo").await?,
            "flr":    SpaceCacheRepo::latest(&state.pool, "flr").await?,
            "cme":    SpaceCacheRepo::latest(&state.pool, "cme").await?,
            "spacex": SpaceCacheRepo::latest(&state.pool, "spacex").await?,
        }))
    }
}
