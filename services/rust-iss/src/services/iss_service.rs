use anyhow::Result;
use serde_json::Value;
use chrono::Utc;
use std::time::Duration;

use crate::AppState;
use crate::clients::iss_client::IssClient;
use crate::domain::iss::{IssLast, IssTrend};
use crate::repo::iss_repo::IssRepo;

pub struct IssService {
    client: IssClient,
}

impl IssService {
    pub fn new(_state: &AppState) -> anyhow::Result<Self> {
        let client = IssClient::new(Duration::from_secs(20))?;
        Ok(Self { client })
    }

    // ---------------------------------------------------------
    //     FETCH + STORE (фоновая задача) с rate-limit
    // ---------------------------------------------------------
    pub async fn fetch_and_store(&self, state: &AppState) -> Result<()> {
        // rate-limit: не больше 10 запросов к внешнему ISS API в минуту
        if !state.limiter.check("iss_fetch", 10, 60).await? {
            // лимит превышен — тихо выходим, без ошибки
            return Ok(());
        }

        let json = self.client.fetch(&state.fallback_url).await?;
        IssRepo::insert(&state.pool, &state.fallback_url, json).await?;

        // очистка кэшей, чтобы не отдавать старое
        let _ = state.redis.delete("iss:last").await;
        let _ = state.redis.delete("iss:trend").await;

        Ok(())
    }

    // ---------------------------------------------------------
    //     GET LAST (с кэшированием Redis)
    // ---------------------------------------------------------
    pub async fn last(&self, state: &AppState) -> Result<Option<IssLast>> {
        let cache_key = "iss:last";

        // 1) Берём из Redis
        if let Some(cached) = state.redis.get_json::<IssLast>(cache_key).await? {
            return Ok(Some(cached));
        }

        // 2) Если нет в Redis — читаем из БД
        let result = IssRepo::last(&state.pool).await?;

        // 3) Кладём в Redis только если данные есть
        if let Some(ref data) = result {
            state.redis
                .set_json(cache_key, data, Duration::from_secs(10))
                .await?;
        }

        Ok(result)
    }

    // ---------------------------------------------------------
    //     GET TREND (с кэшированием Redis)
    // ---------------------------------------------------------
    pub async fn trend(&self, state: &AppState) -> Result<IssTrend> {
        let cache_key = "iss:trend";

        // 1) пробуем кэш
        if let Some(cached) = state.redis.get_json::<IssTrend>(cache_key).await? {
            return Ok(cached);
        }

        // 2) вычисляем
        let rows = IssRepo::last_two(&state.pool).await?;

        let trend = if rows.len() < 2 {
            IssTrend {
                movement: false,
                delta_km: 0.0,
                dt_sec: 0.0,
                velocity_kmh: None,
                from_time: None,
                to_time: None,
                from_lat: None,
                from_lon: None,
                to_lat: None,
                to_lon: None,
            }
        } else {
            let (t2, p2) = &rows[0];
            let (t1, p1) = &rows[1];

            let lat1 = num(&p1["latitude"]);
            let lon1 = num(&p1["longitude"]);
            let lat2 = num(&p2["latitude"]);
            let lon2 = num(&p2["longitude"]);
            let v2 = num(&p2["velocity"]);

            let mut delta_km = 0.0;
            let mut movement = false;

            if let (Some(a1), Some(o1), Some(a2), Some(o2)) = (lat1, lon1, lat2, lon2) {
                delta_km = haversine_km(a1, o1, a2, o2);
                movement = delta_km > 0.1;
            }

            let dt_sec = (*t2 - *t1).num_milliseconds() as f64 / 1000.0;

            IssTrend {
                movement,
                delta_km,
                dt_sec,
                velocity_kmh: v2,
                from_time: Some(*t1),
                to_time: Some(*t2),
                from_lat: lat1,
                from_lon: lon1,
                to_lat: lat2,
                to_lon: lon2,
            }
        };

        // 3) Записываем в Redis
        state
            .redis
            .set_json(cache_key, &trend, Duration::from_secs(5))
            .await?;

        Ok(trend)
    }
}

// ---------------------------------------------------------
// вспомогательные функции
// ---------------------------------------------------------

fn num(v: &Value) -> Option<f64> {
    if let Some(x) = v.as_f64() {
        return Some(x);
    }
    if let Some(s) = v.as_str() {
        return s.parse::<f64>().ok();
    }
    None
}

fn haversine_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let rlat1 = lat1.to_radians();
    let rlat2 = lat2.to_radians();
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2)
        + rlat1.cos() * rlat2.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    6371.0 * c
}
