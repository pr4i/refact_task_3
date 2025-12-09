use anyhow::Result;
use chrono::Utc;
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;

pub struct DonkiClient {
    client: Client,
}

impl DonkiClient {
    pub fn new(timeout: Duration) -> Result<Self> {
        let client = Client::builder()
            .timeout(timeout)
            .user_agent("rust_iss-donki/1.0")
            .build()?;
        Ok(Self { client })
    }

    fn last_days(n: i64) -> (String, String) {
        let to = Utc::now().date_naive();
        let from = to - chrono::Days::new(n as u64);
        (from.to_string(), to.to_string())
    }

    pub async fn fetch_flr(&self, nasa_key: &str) -> Result<Value> {
        let (from, to) = Self::last_days(5);
        let url = "https://api.nasa.gov/DONKI/FLR";
        let mut req = self.client.get(url).query(&[("startDate", from), ("endDate", to)]);
        if !nasa_key.is_empty() {
            req = req.query(&[("api_key", nasa_key)]);
        }
        let json = req.send().await?.json::<Value>().await?;
        Ok(json)
    }

    pub async fn fetch_cme(&self, nasa_key: &str) -> Result<Value> {
        let (from, to) = Self::last_days(5);
        let url = "https://api.nasa.gov/DONKI/CME";
        let mut req = self.client.get(url).query(&[("startDate", from), ("endDate", to)]);
        if !nasa_key.is_empty() {
            req = req.query(&[("api_key", nasa_key)]);
        }
        let json = req.send().await?.json::<Value>().await?;
        Ok(json)
    }
}
