use anyhow::Result;
use reqwest::Client;
use serde_json::Value;
use chrono::Utc;
use std::time::Duration;

pub struct NeoClient {
    client: Client,
}

impl NeoClient {
    pub fn new(timeout: Duration) -> Result<Self> {
        let client = Client::builder()
            .timeout(timeout)
            .user_agent("kasiopea-neo-client/1.0")
            .build()?;

        Ok(Self { client })
    }

    pub async fn fetch(&self, nasa_key: &str) -> Result<Value> {
        let today = Utc::now().date_naive();
        let start = today - chrono::Days::new(2);

        let url = "https://api.nasa.gov/neo/rest/v1/feed";

        let mut req = self.client.get(url).query(&[
            ("start_date", start.to_string()),
            ("end_date", today.to_string()),
        ]);

        if !nasa_key.is_empty() {
            req = req.query(&[("api_key", nasa_key)]);
        }

        Ok(req.send().await?.json::<Value>().await?)
    }
}
