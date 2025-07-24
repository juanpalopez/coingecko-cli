use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::api::client::CoinGecko;
use crate::api::response::Response;
use crate::api::transport::Transport;
use crate::api::Method;

// Coins List
pub enum CoinsListParts {
    None,
}

impl CoinsListParts {
    fn url(&self) -> &'static str {
        match self {
            CoinsListParts::None => "/coins/list",
        }
    }
}

pub struct CoinsList<'a> {
    transport: &'a Transport,
    parts: CoinsListParts,
}

impl<'a> CoinsList<'a> {
    pub fn new(transport: &'a Transport, parts: CoinsListParts) -> Self {
        CoinsList { transport, parts }
    }

    pub async fn send(self) -> Result<Response> {
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
pub struct CoinsListResponse {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

pub struct Coins<'a> {
    transport: &'a Transport,
}

impl<'a> Coins<'a> {
    fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }

    fn transport(&self) -> &Transport {
        self.transport
    }

    pub fn list(&self, parts: CoinsListParts) -> CoinsList {
        CoinsList::new(self.transport(), parts)
    }
}

impl CoinGecko {
    pub fn coins(&self) -> Coins {
        Coins::new(self.transport())
    }
}