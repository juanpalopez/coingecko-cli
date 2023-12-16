// use anyhow::Result;
// use async_trait::async_trait;
// use reqwest::Client;

// use client::Transport;

// #[derive(Debug, Copy, Clone)]
// pub enum CryptoClientStatus {
//     Success,
//     Error,
//     Unknown,
// }

// pub struct CryptoClientRes {
//     status: CryptoClientStatus,
//     body: String,
// }

// impl CryptoClientRes {
//     pub fn get_status(&self) -> CryptoClientStatus {
//         self.status
//     }

//     pub fn get_body(&self) -> &String {
//         &self.body
//     }
// }

// #[async_trait]
// pub trait CryptoClient {
//     async fn ping(&self) -> Result<CryptoClientRes>;
//     async fn simple_supported_vs_currencies(&self) -> Result<CryptoClientRes>;
// }

// pub struct CryptoClientHTTP;

// impl CryptoClientHTTP {
//     const BASE_API_URL: &'static str = "https://api.coingecko.com/api/v3";

//     async fn get(&self, endpoint: &str) -> Result<CryptoClientRes> {
//         let client = Client::new();
//         let url = format!("{}/{}", Self::BASE_API_URL, endpoint);
//         let response = client.get(url).send().await?;
//         let status = response.status();
//         let body = response.text().await?;

//         let status = match &status.as_u16() {
//             200..=299 => CryptoClientStatus::Success,
//             300..=599 => CryptoClientStatus::Error,
//             _ => CryptoClientStatus::Unknown,
//         };

//         Ok(CryptoClientRes { status, body })
//     }
// }


// #[async_trait]
// impl CryptoClient for CryptoClientHTTP {
//     async fn ping(&self) -> Result<CryptoClientRes> {
//         let response = self.get("/ping").await?;
//         Ok(response)
//     }

//     async fn simple_supported_vs_currencies(&self) -> Result<CryptoClientRes> {
//         let response = self.get("/simple/supported_vs_currencies").await?;
//         Ok(response)
//     }
// }

// enum SimpleSupportedVsCurrenciesParts {
//     None,
// }
// impl SimpleSupportedVsCurrenciesParts{
//     pub fn url(self) -> &'static str {
//         "/simple/supported_vs_currencies".into()
//     }
// }

// struct SimpleSupportedVsCurrencies<'a>{
//     transport: &'a Transport,
//     parts: SimpleSupportedVsCurrenciesParts
// }

// impl<'a> SimpleSupportedVsCurrencies<'a> {
//     fn new(transport: &'a Transport, parts:SimpleSupportedVsCurrenciesParts) -> Self{
//         SimpleSupportedVsCurrencies { transport, parts }
//     }

//     fn send(self)
// }

// struct Simple<'a>{
//     transport: &'a Transport
// }

// impl<'a> Simple<'a> {
//     fn new(transport: &'a Transport) -> Self{
//         Self { transport }
//     }
//     fn transport(&self) -> &Transport {
//         self.transport
//     }

//     fn supported_vs_currencies(&self, parts: SimpleSupportedVsCurrenciesParts) -> SimpleSupportedVsCurrencies {
//         SimpleSupportedVsCurrencies::new(self.transport(), parts)
//     }
// }


// impl CoinGecko {
//     fn simple(&self) -> Simple {
//         Simple::new(self.transport())
//     }
// }

