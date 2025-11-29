use anyhow::Result;
use serde_json::Value;
use chrono::{DateTime, Utc};

use crate::AppState;
use crate::clients::iss_client::IssClient;
use crate::domain::iss::{IssLast, IssTrend};
use crate::repo::iss_repo::IssRepo;

pub struct IssService {
    client: IssClient,
}

impl IssService {
    pub fn new(state: &AppState) -> anyhow::Result<Self> {
        let client = IssClient::new(std::time::Duration::from_secs(20))?;
        Ok(Self { client })
    }

    pub async fn fetch_and_store(&self, state: &AppState) -> Result<()> {
        let json = self.client.fetch(&state.fallback_url).await?;
        IssRepo::insert(&state.pool, &state.fallback_url, json).await
    }

    pub async fn last(&self, state: &AppState) -> Result<Option<IssLast>> {
        IssRepo::last(&state.pool).await
    }

    pub async fn trend(&self, state: &AppState) -> Result<IssTrend> {
        let rows = IssRepo::last_two(&state.pool).await?;

        if rows.len() < 2 {
            return Ok(IssTrend {
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
            });
        }

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

        Ok(IssTrend {
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
        })
    }
}

// вспомогательные функции перенесли сюда из main.rs

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
