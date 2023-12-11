use clap::{Parser, Subcommand};

mod ping;
pub mod simple;
use crate::ping::PingCtx;
use crate::simple::SimpleCtx;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Crypto {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Ping(PingCtx),
    Simple(SimpleCtx)
}

#[tokio::main]
async fn main() {
    let cli = Crypto::parse();

    match &cli.commands {
        Commands::Ping(ctx) => PingCtx::run_command(ctx).await,
        Commands::Simple(ctx) => SimpleCtx::run_command(ctx).await,
    };
}
