use clap::Parser;

#[derive(Parser, Debug)]
pub struct PingCtx {}

impl PingCtx {
    pub async fn run_command(&self) {
        println!("Pinging...");
        // let crypto_client = client_bk::CryptoClientHTTP;

        // let http_res = match crypto_client.ping().await {
        //     Ok(res) => res,
        //     Err(error) => panic!("Problem with ping {}", error),
        // };
        // println!("{:?}", http_res.get_status());
    }
}
