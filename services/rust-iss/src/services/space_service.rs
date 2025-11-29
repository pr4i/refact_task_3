use anyhow::Result;
use serde_json::Value;

use crate::{AppState, repo::space_cache_repo::SpaceCacheRepo};
use crate::clients::{
    apod_client::ApodClient,
    neo_client::NeoClient,
    donki_client::DonkiClient,
    spacex_client::SpacexClient
};

pub struct SpaceService {
    apod: ApodClient,
    neo: NeoClient,
    donki: DonkiClient,
    spacex: SpacexClient,
}

impl SpaceService {
    pub fn new() -> Result<Self> {
        Ok(Self {
            apod: ApodClient::new(std::time::Duration::from_secs(30))?,
            neo: NeoClient::new(std::time::Duration::from_secs(30))?,
            donki: DonkiClient::new(std::time::Duration::from_secs(30))?,
            spacex: SpacexClient::new(std::time::Duration::from_secs(30))?,
        })
    }

    pub async fn refresh(&self, state: &AppState, src: &str) -> Result<()> {
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

    pub async fn latest(&self, state: &AppState, src: &str) -> Result<Option<Value>> {
        SpaceCacheRepo::latest(&state.pool, src).await
    }

    pub async fn summary(&self, state: &AppState) -> Result<Value> {
        let apod = self.latest(state, "apod").await?;
        let neo = self.latest(state, "neo").await?;
        let flr = self.latest(state, "flr").await?;
        let cme = self.latest(state, "cme").await?;
        let spacex = self.latest(state, "spacex").await?;

        let osdr_count = SpaceCacheRepo::count_osdr(&state.pool).await?;

        Ok(serde_json::json!({
            "apod": apod,
            "neo": neo,
            "flr": flr,
            "cme": cme,
            "spacex": spacex,
            "osdr_count": osdr_count
        }))
    }
}
