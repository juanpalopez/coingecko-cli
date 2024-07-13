use crate::api::client::CoinGecko;
use crate::api::contract::ContractParts;
use anyhow::Result;
use clap::{Parser, Subcommand};
use serde_json::Value;

#[derive(Parser)]
pub struct ContractCtx {}

impl ContractCtx {
    pub async fn run_command(&self, client: &CoinGecko) -> Result<()> {
       
    }
}
