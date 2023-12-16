use clap::{Parser, Subcommand};

mod api;
mod cli;

// mod simple;
use crate::cli::ping::PingCtx;
use crate::cli::simple::SimpleCtx;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Crypto {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Ping(PingCtx),
    Simple(SimpleCtx),
}

#[tokio::main]
async fn main() {
    let cli = Crypto::parse();

    match &cli.commands {
        Commands::Ping(ctx) => PingCtx::run_command(ctx).await,
        Commands::Simple(ctx) => SimpleCtx::run_command(ctx).await,
    };
}
