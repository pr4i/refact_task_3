use anyhow::Result;
use reqwest::Client;
use serde_json::Value;
use chrono::Utc;
use std::time::Duration;

pub struct DonkiClient {
    client: Client,
}

impl DonkiClient {
    pub fn new(timeout: Duration) -> Result<Self> {
        let client = Client::builder()
            .timeout(timeout)
            .user_agent("kasiopea-donki-client/1.0")
            .build()?;

        Ok(Self { client })
    }

    pub async fn fetch_flr(&self, nasa_key: &str) -> Result<Value> {
        let (from, to) = last_days(5);
        let url = "https://api.nasa.gov/DONKI/FLR";

        let req = self.client.get(url)
            .query(&[("startDate", from), ("endDate", to)])
            .query(&[("api_key", nasa_key)]);

        Ok(req.send().await?.json::<Value>().await?)
    }

    pub async fn fetch_cme(&self, nasa_key: &str) -> Result<Value> {
        let (from, to) = last_days(5);
        let url = "https://api.nasa.gov/DONKI/CME";

        let req = self.client.get(url)
            .query(&[("startDate", from), ("endDate", to)])
            .query(&[("api_key", nasa_key)]);

        Ok(req.send().await?.json::<Value>().await?)
    }
}

fn last_days(n: i64) -> (String, String) {
    let to = Utc::now().date_naive();
    let from = to - chrono::Days::new(n as u64);
    (from.to_string(), to.to_string())
}
