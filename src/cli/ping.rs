use std::collections::HashMap;

use clap::Parser;
use crate::api::client::CoinGecko;
use crate::api::error::Error;
use crate::api::ping::{PingParts, PingPingResponse};

#[derive(Parser, Debug)]
pub struct PingCtx {}

impl PingCtx {
    pub async fn run_command(&self, client: &CoinGecko) -> Result<(), Error>{
        println!("Pinging...");
        let response = client.
        ping().
        ping(PingParts::None).
        send().await?;

        let body:PingPingResponse = response.json().await?;
        println!("{:?}", body);

        // match body {
        //     Ok(resp) => println!("{:?}",resp),
        //     Err(err) => print!("{:?}", err)
        // }
        Ok(())
        
    }
}
