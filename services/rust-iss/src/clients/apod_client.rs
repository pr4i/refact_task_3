use anyhow::Result;
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;

pub struct ApodClient {
    client: Client,
}

impl ApodClient {
    pub fn new(timeout: Duration) -> Result<Self> {
        let client = Client::builder()
            .timeout(timeout)
            .user_agent("kasiopea-apod-client/1.0")
            .build()?;

        Ok(Self { client })
    }

    pub async fn fetch(&self, nasa_key: &str) -> Result<Value> {
        let url = "https://api.nasa.gov/planetary/apod";

        let mut req = self.client.get(url).query(&[("thumbs", "true")]);

        if !nasa_key.is_empty() {
            req = req.query(&[("api_key", nasa_key)]);
        }

        Ok(req.send().await?.json::<Value>().await?)
    }
}
