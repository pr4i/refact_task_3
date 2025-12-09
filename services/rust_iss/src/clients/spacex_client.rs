use anyhow::Result;
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;

pub struct SpacexClient {
    client: Client,
}

impl SpacexClient {
    pub fn new(timeout: Duration) -> Result<Self> {
        let client = Client::builder()
            .timeout(timeout)
            .user_agent("rust_iss-spacex/1.0")
            .build()?;
        Ok(Self { client })
    }

    pub async fn fetch(&self) -> Result<Value> {
        let url = "https://api.spacexdata.com/v4/launches/next";
        let json = self.client.get(url).send().await?.json::<Value>().await?;
        Ok(json)
    }
}
