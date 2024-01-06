use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::api::client::CoinGecko;
use crate::api::response::Response;
use crate::api::transport::Transport;
use crate::api::Method;

pub enum SimpleSupportedVsCurrenciesParts {
    None,
}

impl SimpleSupportedVsCurrenciesParts {
    fn url(&self) -> &'static str {
        match self {
            SimpleSupportedVsCurrenciesParts::None => "/simple/supported_vs_currencies",
        }
    }
}

pub struct SimpleSupportedVsCurrencies<'a> {
    transport: &'a Transport,
    parts: SimpleSupportedVsCurrenciesParts,
}

impl<'a> SimpleSupportedVsCurrencies<'a> {
    pub fn new(transport: &'a Transport, parts: SimpleSupportedVsCurrenciesParts) -> Self {
        SimpleSupportedVsCurrencies { transport, parts }
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
pub struct SimpleSupportedVsCurrenciesResponse(Vec<String>);

impl SimpleSupportedVsCurrenciesResponse {
    pub fn coins(&self) -> &[String] {
        &self.0
    }
}

pub struct Simple<'a> {
    transport: &'a Transport,
}

impl<'a> Simple<'a> {
    fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }

    fn transport(&self) -> &Transport {
        self.transport
    }

    pub fn supported_vs_currencies(
        &self,
        parts: SimpleSupportedVsCurrenciesParts,
    ) -> SimpleSupportedVsCurrencies {
        SimpleSupportedVsCurrencies::new(self.transport(), parts)
    }
}

impl CoinGecko {
    pub fn simple(&self) -> Simple {
        Simple::new(self.transport())
    }
}
