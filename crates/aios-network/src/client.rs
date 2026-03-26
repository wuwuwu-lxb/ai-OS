//! HTTP Client

use reqwest::Client;
use anyhow::Result;

pub struct NetworkClient {
    client: Client,
}

impl NetworkClient {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        Ok(Self { client })
    }

    pub async fn get(&self, url: &str) -> Result<String> {
        let response = self.client.get(url).send().await?;
        Ok(response.text().await?)
    }

    pub async fn post(&self, url: &str, body: &str) -> Result<String> {
        let response = self.client.post(url).body(body.to_string()).send().await?;
        Ok(response.text().await?)
    }
}
