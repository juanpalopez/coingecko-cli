use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;

#[derive(Debug, Copy, Clone)]
pub enum CryptoClientStatus {
    Success,
    Error,
    Unknown,
}

#[derive(Debug)]
pub struct CryptoClientRes {
    status: CryptoClientStatus,
}

impl CryptoClientRes {
    pub fn get_status(&self) -> CryptoClientStatus {
        self.status
    }
}

#[async_trait]
pub trait CryptoClient {
    async fn ping(&self) -> Result<CryptoClientRes>;
}

pub struct CryptoClientHTTP;

impl CryptoClientHTTP {
    const BASE_API_URL: &str = "https://api.coingecko.com/api/v3";
}

#[async_trait]
impl CryptoClient for CryptoClientHTTP {
    async fn ping(&self) -> Result<CryptoClientRes> {
        let client = Client::new();
        let url = format!("{}/ping", Self::BASE_API_URL);
        let response = client.get(url).send().await?;

        let status = match &response.status().as_u16() {
            200..=299 => CryptoClientStatus::Success,
            300..=599 => CryptoClientStatus::Error,
            _ => CryptoClientStatus::Unknown,
        };

        Ok(CryptoClientRes { status })
    }
}
