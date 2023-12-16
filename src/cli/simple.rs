use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct SimpleCtx {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Get the current price of any cryptocurrencies in any other supported currencies that you need.
    Price(PriceCtx),
    /// Get current price of tokens (using contract addresses) for a
    /// given platform in any other currency that you need.
    TokenPrice(TokenPriceCtx),
    /// Get list of supported_vs_currencies.
    SupportedVsCurrencies(SupportedVsCurrenciesCtx),
}

#[derive(Debug, Parser)]
pub struct SupportedVsCurrenciesCtx {}

impl SupportedVsCurrenciesCtx {
    // fn parse_response(&self, body: &String) -> Vec<String> {
    // 	let data = serde_json::from_str(body.as_str()).unwrap();
    //     let mut result = Vec::new();

    // 	if let Value::Array(arr) = data {
    // 		for currency in arr {
    // 			if let Some(c) = currency.as_str() {
    // 				result.push(c.to_string());
    // 			}
    // 		}
    // 	}

    //     result
    // }
    pub async fn run_command(&self) {
        println!("Supported Vs currencies...");
        // let crypto_client = client::CryptoClientHTTP;

        // let client_res = match crypto_client.simple_supported_vs_currencies().await {
        //     Ok(res) => res,
        //     Err(error) => panic!("Problem with ping {}", error),
        // };
        // let currencies = self.parse_response(client_res.get_body());

        // for currency in currencies {
        //     println!("{}", currency);
        // }
    }
}

#[derive(Debug, Parser)]
pub struct PriceCtx {}

impl PriceCtx {
    pub async fn run_command(&self) {
        println!("Price...")
    }
}

#[derive(Debug, Parser)]
pub struct TokenPriceCtx {}

impl TokenPriceCtx {
    pub async fn run_command(&self) {
        println!("TokenPrice...")
    }
}

impl SimpleCtx {
    pub async fn run_command(&self) {
        match &self.commands {
            Commands::SupportedVsCurrencies(ctx) => {
                SupportedVsCurrenciesCtx::run_command(ctx).await;
            }
            Commands::Price(ctx) => {
                PriceCtx::run_command(ctx).await;
            }
            Commands::TokenPrice(ctx) => {
                TokenPriceCtx::run_command(ctx).await;
            }
        };
    }
}
