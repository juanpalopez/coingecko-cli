use clap::{Parser, Subcommand};

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
    fn run_command(&self) {
        println!("Pinging...");
    }
}

fn main() {
    let cli = Crypto::parse();

    match &cli.commands {
        Commands::Ping(ctx) => PingCtx::run_command(ctx),
    };
}
