use crate::api::asset_platforms::AssetPlatformsListParts;
use crate::api::client::CoinGecko;
use anyhow::Result;
use clap::{Parser, Subcommand};
use serde_json::Value;

#[derive(Parser)]
pub struct AssetPlatformsCtx {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all asset platforms (Blockchain networks)
    List(ListCtx),
}

#[derive(Parser)]
pub struct ListCtx {
    /// List only platforms that support NFT
    #[arg(long, default_value_t = false)]
    only_nft_support: bool,
}

impl ListCtx {
    pub async fn run_command(&self, client: &CoinGecko) -> Result<()> {
        println!("List...");
        let response = client
            .asset_platforms()
            .list(AssetPlatformsListParts::None)
            .filter(self.only_nft_support)
            .send()
            .await?;

        let body: Value = response.json().await?;
        println!("{:#}", body);
        Ok(())
    }
}

impl AssetPlatformsCtx {
    pub async fn run_command(&self, client: &CoinGecko) -> Result<()> {
        println!("Asset Platforms...");
        match &self.commands {
            Commands::List(ctx) => {
                ListCtx::run_command(ctx, client).await?;
            }
        };
        Ok(())
    }
}
