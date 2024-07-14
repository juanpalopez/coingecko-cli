use crate::api::client::CoinGecko;
use clap::{Parser, Subcommand};

mod api;
mod cli;

// mod simple;
use crate::api::transport::TransportBuilder;
use crate::cli::asset_platforms::AssetPlatformsCtx;
use crate::cli::ping::PingCtx;
use crate::cli::simple::SimpleCtx;
use anyhow::Result;
use url::Url;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Crypto {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    AssetPlatforms(AssetPlatformsCtx),
    Ping(PingCtx),
    Simple(SimpleCtx),
    Contract(ContractCtx),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Crypto::parse();
    const BASE_API_URL: &'static str = "https://api.coingecko.com/api/v3/";
    let base_url = Url::parse(BASE_API_URL)?;
    let transport_builder = TransportBuilder::new(base_url);
    let transport = transport_builder.build()?;
    let client = CoinGecko::new(transport);

    match &cli.commands {
        Commands::Ping(ctx) => PingCtx::run_command(ctx, &client).await?,
        Commands::Simple(ctx) => SimpleCtx::run_command(ctx, &client).await?,
        Commands::AssetPlatforms(ctx) => AssetPlatformsCtx::run_command(ctx, &client).await?,
        Commands::Contract(ctx) => ContractCtx::run_command(ctx, &client).await?,
    };
    Ok(())
}
