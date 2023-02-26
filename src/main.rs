use clap::{Parser, Subcommand};

mod client;
use crate::client::CryptoClient;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Crypto {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Ping(PingCtx),
}

#[derive(Parser, Debug)]
struct PingCtx {}

impl PingCtx {
    async fn run_command(&self) {
        println!("Pinging...");
        let crypto_client = client::CryptoClientHTTP;

        let http_res = match crypto_client.ping().await {
            Ok(res) => res,
            Err(error) => panic!("Problem with ping {:?}", error),
        };
        println!("{:?}", http_res.get_status());
    }
}

#[tokio::main]
async fn main() {
    let cli = Crypto::parse();

    match &cli.commands {
        Commands::Ping(ctx) => PingCtx::run_command(ctx).await,
    };
}
