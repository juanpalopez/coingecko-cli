use crate::api::client::CoinGecko;
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct CoinsCtx {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get list of all supported coins id, name and symbol (placeholder)
    List(ListCtx),
}

#[derive(Parser)]
pub struct ListCtx {}

impl ListCtx {
    pub async fn run_command(&self, _client: &CoinGecko) -> Result<()> {
        println!("Coins list command (not yet implemented)");
        Ok(())
    }
}

impl CoinsCtx {
    pub async fn run_command(&self, client: &CoinGecko) -> Result<()> {
        match &self.commands {
            Commands::List(ctx) => {
                ListCtx::run_command(ctx, client).await?;
            }
        };
        Ok(())
    }
}