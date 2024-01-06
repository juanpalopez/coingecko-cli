use serde::{Deserialize, Serialize};

use crate::api::response::Response;
use crate::api::transport::Transport;
use crate::api::Method;
use anyhow::Result;

use crate::api::client::CoinGecko;

pub enum PingParts {
    None,
}

impl PingParts {
    fn url(&self) -> &'static str {
        match self {
            PingParts::None => "/ping",
        }
    }
}

pub struct PingPing<'a> {
    transport: &'a Transport,
    parts: PingParts,
}

impl<'a> PingPing<'a> {
    pub fn new(transport: &'a Transport, parts: PingParts) -> Self {
        PingPing { transport, parts }
    }

    pub async fn send(&self) -> Result<Response> {
        let path = self.parts.url();
        let method = Method::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams {}
            let query_params = QueryParams {};
            Some(query_params)
        };
        let response = self
            .transport
            .send(method, &path, query_string.as_ref())
            .await?;
        Ok(response)
    }
}

#[derive(Deserialize, Debug)]
pub struct PingPingResponse {
    gecko_says: String,
}

impl PingPingResponse {
    pub fn gecko_says(&self) -> &String {
        &self.gecko_says
    }
}
pub struct Ping<'a> {
    transport: &'a Transport,
}

impl<'a> Ping<'a> {
    fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }

    fn transport(&self) -> &Transport {
        self.transport
    }

    pub fn ping(&self, parts: PingParts) -> PingPing {
        PingPing::new(self.transport(), parts)
    }
}

impl CoinGecko {
    pub fn ping(&self) -> Ping {
        Ping::new(self.transport())
    }
}
