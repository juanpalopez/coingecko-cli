use crate::api::client::CoinGecko;
use crate::api::simple::SimpleSupportedVsCurrenciesParts;
use anyhow::Result;
use clap::{Parser, Subcommand};
use serde_json::Value;

#[derive(Parser)]
pub struct SimpleCtx {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get the current price of any cryptocurrencies in any other supported currencies that you need.
    Price(PriceCtx),
    /// Get current price of tokens (using contract addresses) for a
    /// given platform in any other currency that you need.
    TokenPrice(TokenPriceCtx),
    /// Get list of supported_vs_currencies.
    SupportedVsCurrencies(SupportedVsCurrenciesCtx),
}

#[derive(Parser)]
pub struct SupportedVsCurrenciesCtx {}

impl SupportedVsCurrenciesCtx {
    pub async fn run_command(&self, client: &CoinGecko) -> Result<()> {
        println!("Supported Vs currencies...");
        let response = client
            .simple()
            .supported_vs_currencies(SimpleSupportedVsCurrenciesParts::None)
            .send()
            .await?;

        let body: Value = response.json().await?;
        println!("{:#}", body);

        Ok(())
    }
}

#[derive(Parser)]
pub struct PriceCtx {}

impl PriceCtx {
    pub async fn run_command(&self) {
        println!("Price...")
    }
}

#[derive(Parser)]
pub struct TokenPriceCtx {}

impl TokenPriceCtx {
    pub async fn run_command(&self) {
        println!("TokenPrice...")
    }
}

impl SimpleCtx {
    pub async fn run_command(&self, client: &CoinGecko) -> Result<()> {
        match &self.commands {
            Commands::SupportedVsCurrencies(ctx) => {
                SupportedVsCurrenciesCtx::run_command(ctx, client).await;
            }
            Commands::Price(ctx) => {
                PriceCtx::run_command(ctx).await;
            }
            Commands::TokenPrice(ctx) => {
                TokenPriceCtx::run_command(ctx).await;
            }
        };
        Ok(())
    }
}
