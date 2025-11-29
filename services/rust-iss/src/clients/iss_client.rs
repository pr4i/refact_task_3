use anyhow::Result;
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;

pub struct IssClient {
    client: Client,
}

impl IssClient {
    pub fn new(timeout: Duration) -> Result<Self> {
        let client = Client::builder()
            .timeout(timeout)
            .user_agent("kasiopea-rust-iss/1.0")
            .build()?;
        Ok(Self { client })
    }

    pub async fn fetch(&self, url: &str) -> Result<Value> {
        let resp = self.client.get(url).send().await?;
        let json = resp.json::<Value>().await?;
        Ok(json)
    }
}
