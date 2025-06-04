use crate::api::client::CoinGecko;
use crate::api::simple::{SimpleSupportedVsCurrenciesParts, SimpleTokenPriceParts};
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
pub struct TokenPriceCtx {
    /// The id of the platform issuing the token.
    #[arg(long)]
    id: String,
    /// The contract address of the token.
    #[arg(long)]
    contract_addresses: String,
    /// The target currency of market data.
    #[arg(long)]
    vs_currency: String,
    /// Include market capialization.
    #[arg(long, default_value_t = false)]
    include_market_cap: bool,
    /// Include 24hr trading volume.
    #[arg(long, default_value_t = false)]
    include_24hr_vol: bool,
    /// Include 24hr change.
    #[arg(long, default_value_t = false)]
    include_24hr_change: bool,
    /// Include last updated timestamp.
    #[arg(long, default_value_t = false)]
    include_last_updated_at: bool,
    /// Precision of the market data.
    #[arg(long)]
    precision: Option<u8>,
}

impl TokenPriceCtx {
    pub async fn run_command(&self, client: &CoinGecko) -> Result<()> {
        println!("TokenPrice...");
        let response = client
            .simple()
            .token_price(
                SimpleTokenPriceParts::Id(self.id.as_str()),
                self.contract_addresses.as_str(),
                self.vs_currency.as_str(),
            )
            .include_market_cap(self.include_market_cap)
            .include_24hr_vol(self.include_24hr_vol)
            .include_24hr_change(self.include_24hr_change)
            .include_last_updated_at(self.include_last_updated_at)
            .precision(self.precision)
            .send()
            .await?;

        let body: Value = response.json().await?;
        println!("{:#}", body);
        Ok(())
    }
}

impl SimpleCtx {
    pub async fn run_command(&self, client: &CoinGecko) -> Result<()> {
        match &self.commands {
            Commands::SupportedVsCurrencies(ctx) => {
                SupportedVsCurrenciesCtx::run_command(ctx, client).await?;
            }
            Commands::Price(ctx) => {
                PriceCtx::run_command(ctx).await;
            }
            Commands::TokenPrice(ctx) => {
                TokenPriceCtx::run_command(ctx, client).await?;
            }
        };
        Ok(())
    }
}
