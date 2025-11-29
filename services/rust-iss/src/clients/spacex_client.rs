use anyhow::Result;
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;

pub struct SpacexClient {
    client: Client,
}

impl SpacexClient {
    pub fn new(timeout: Duration) -> Result<Self> {
        Ok(Self {
            client: Client::builder()
                .timeout(timeout)
                .user_agent("kasiopea-spacex-client/1.0")
                .build()?,
        })
    }

    pub async fn fetch(&self) -> Result<Value> {
        let url = "https://api.spacexdata.com/v4/launches/next";
        Ok(self.client.get(url).send().await?.json::<Value>().await?)
    }
}
