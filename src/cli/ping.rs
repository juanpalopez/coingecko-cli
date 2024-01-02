use crate::api::client::CoinGecko;
use crate::api::ping::PingParts;
use anyhow::Result;
use clap::Parser;
use serde_json::Value;

#[derive(Parser, Debug)]
pub struct PingCtx {}

impl PingCtx {
    pub async fn run_command(&self, client: &CoinGecko) -> Result<()> {
        println!("Pinging...");
        let response = client.ping().ping(PingParts::None).send().await?;

        let body: Value = response.json().await?;
        println!("{:#}", body);

        Ok(())
    }
}
