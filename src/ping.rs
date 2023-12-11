use clap::Parser;

#[path = "./client.rs"]
mod client;
use client::CryptoClient;

#[derive(Parser, Debug)]
pub struct PingCtx {}

impl PingCtx {
    pub async fn run_command(&self) {
        println!("Pinging...");
        let crypto_client = client::CryptoClientHTTP;

        let http_res = match crypto_client.ping().await {
            Ok(res) => res,
            Err(error) => panic!("Problem with ping {}", error),
        };
        println!("{:?}", http_res.get_status());
    }
}
