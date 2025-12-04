use anyhow::Result;
use serde_json::Value;
use std::time::Duration;

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
            apod: ApodClient::new(Duration::from_secs(30))?,
            neo: NeoClient::new(Duration::from_secs(30))?,
            donki: DonkiClient::new(Duration::from_secs(30))?,
            spacex: SpacexClient::new(Duration::from_secs(30))?,
        })
    }

    // ---------------------------------------------------------
    // REFRESH (обновление NASA/SpaceX + кэш + rate-limit)
    // ---------------------------------------------------------
    pub async fn refresh(&self, state: &AppState, src: &str) -> Result<()> {
        let redis_key = format!("space:{src}");

        match src {
            "apod" => {
                // не больше 3 запросов APOD в минуту
                if !state.limiter.check("apod", 3, 60).await? {
                    return Ok(());
                }

                let fresh = self.apod.fetch(&state.nasa_key).await?;
                SpaceCacheRepo::write(&state.pool, "apod", fresh.clone()).await?;
                state.redis.set_json(&redis_key, &fresh, Duration::from_secs(3600)).await?;
                let _ = state.redis.delete("space:summary").await;
            }

            "neo" => {
                // не больше 2 запросов NEO в минуту
                if !state.limiter.check("neo", 2, 60).await? {
                    return Ok(());
                }

                let fresh = self.neo.fetch(&state.nasa_key).await?;
                SpaceCacheRepo::write(&state.pool, "neo", fresh.clone()).await?;
                state.redis.set_json(&redis_key, &fresh, Duration::from_secs(3600)).await?;
                let _ = state.redis.delete("space:summary").await;
            }

            "flr" => {
                // не больше 2 запросов DONKI-FLR в минуту
                if !state.limiter.check("donki_flr", 2, 60).await? {
                    return Ok(());
                }

                let fresh = self.donki.fetch_flr(&state.nasa_key).await?;
                SpaceCacheRepo::write(&state.pool, "flr", fresh.clone()).await?;
                state.redis.set_json(&redis_key, &fresh, Duration::from_secs(1800)).await?;
                let _ = state.redis.delete("space:summary").await;
            }

            "cme" => {
                // не больше 2 запросов DONKI-CME в минуту
                if !state.limiter.check("donki_cme", 2, 60).await? {
                    return Ok(());
                }

                let fresh = self.donki.fetch_cme(&state.nasa_key).await?;
                SpaceCacheRepo::write(&state.pool, "cme", fresh.clone()).await?;
                state.redis.set_json(&redis_key, &fresh, Duration::from_secs(1800)).await?;
                let _ = state.redis.delete("space:summary").await;
            }

            "spacex" => {
                // не больше 5 запросов SpaceX в минуту
                if !state.limiter.check("spacex", 5, 60).await? {
                    return Ok(());
                }

                let fresh = self.spacex.fetch().await?;
                SpaceCacheRepo::write(&state.pool, "spacex", fresh.clone()).await?;
                state.redis.set_json(&redis_key, &fresh, Duration::from_secs(300)).await?;
                let _ = state.redis.delete("space:summary").await;
            }

            _ => {}
        }

        Ok(())
    }

    // ---------------------------------------------------------
    // LATEST (Redis → PostgreSQL)
    // ---------------------------------------------------------
    pub async fn latest(&self, state: &AppState, src: &str) -> Result<Option<Value>> {
        let key = format!("space:{src}");

        // 1. Пробуем Redis
        if let Some(cached) = state.redis.get_json::<Value>(&key).await? {
            return Ok(Some(cached));
        }

        // 2. Пробуем PostgreSQL
        let latest = SpaceCacheRepo::latest(&state.pool, src).await?;

        if let Some(ref payload) = latest {
            state.redis
                .set_json(&key, payload, Duration::from_secs(300))
                .await?;
        }

        Ok(latest)
    }

    // ---------------------------------------------------------
    // SUMMARY (общий дашборд + Redis)
    // ---------------------------------------------------------
    pub async fn summary(&self, state: &AppState) -> Result<Value> {
        let key = "space:summary";

        // 1. Кэш
        if let Some(cached) = state.redis.get_json::<Value>(key).await? {
            return Ok(cached);
        }

        // 2. Реальные данные
        let apod   = self.latest(state, "apod").await?;
        let neo    = self.latest(state, "neo").await?;
        let flr    = self.latest(state, "flr").await?;
        let cme    = self.latest(state, "cme").await?;
        let spacex = self.latest(state, "spacex").await?;
        let osdr_count = SpaceCacheRepo::count_osdr(&state.pool).await?;

        let summary = serde_json::json!({
            "apod": apod,
            "neo": neo,
            "flr": flr,
            "cme": cme,
            "spacex": spacex,
            "osdr_count": osdr_count
        });

        // 3. Кладём в Redis
        state.redis
            .set_json(key, &summary, Duration::from_secs(120))
            .await?;

        Ok(summary)
    }
}
