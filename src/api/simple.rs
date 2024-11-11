use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::api::client::CoinGecko;
use crate::api::response::Response;
use crate::api::transport::Transport;
use crate::api::Method;

// Supported Currencies List
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

// Coin Price by Token Address
pub enum SimpleTokenPriceParts<'a> {
    Id(&'a str),
}

impl<'a> SimpleTokenPriceParts<'a> {
    fn url(self) -> String {
        match self {
            SimpleTokenPriceParts::Id(id) => {
                let p = "/simple/token_price/";
                let p = p.to_owned() + id;
                p
            },
        }
    }
}

pub struct SimpleTokenPrice<'a, 'b> {
    transport: &'a Transport,
    parts: SimpleTokenPriceParts<'b>,
    contract_addresses: &'b str,
    vs_currencies: &'b str,
    include_market_cap: Option<bool>,
    include_24hr_vol: Option<bool>,
    include_24hr_change: Option<bool>,
    include_last_updated_at: Option<bool>,
    precision: Option<u8>,
}

impl<'a, 'b> SimpleTokenPrice<'a, 'b> {
    pub fn new(
        transport: &'a Transport,
        parts: SimpleTokenPriceParts<'b>,
        contract_address: &'b str,
        vs_currencies: &'b str,
    ) -> Self {
        SimpleTokenPrice { 
            transport, 
            parts,
            contract_addresses: contract_address,
            vs_currencies,
            include_market_cap: None,
            include_24hr_vol: None,
            include_24hr_change: None,
            include_last_updated_at: None,
            precision: None,
        }
    }

    pub fn include_market_cap(mut self, include: bool) -> Self {
        if include {
            self.include_market_cap = Some(true);
        };
        self
    }

    pub fn include_24hr_vol(mut self, include: bool) -> Self {
        if include {
            self.include_24hr_vol = Some(true);
        };
        self
    }

    pub fn include_24hr_change(mut self, include: bool) -> Self {
        if include {
            self.include_24hr_change = Some(true);
        };
        self
    }

    pub fn include_last_updated_at(mut self, include: bool) -> Self {
        if include {
            self.include_last_updated_at = Some(true);
        };
        self
    }

    pub fn precision(mut self, precision: Option<u8>) -> Self {
        self.precision = precision;
        self
    }

    pub async fn send(self) -> Result<Response> {
        let path = self.parts.url();
        let method = Method::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                contract_addresses: &'a str,
                vs_currencies:&'a str,
                include_market_cap: Option<bool>,
                include_24hr_vol: Option<bool>,
                include_24hr_change: Option<bool>,
                include_last_updated_at: Option<bool>,
                precision: Option<u8>,
            }
            let query_params = QueryParams {
                contract_addresses: &self.contract_addresses,
                vs_currencies: &self.vs_currencies,
                include_market_cap: self.include_market_cap,
                include_24hr_vol: self.include_24hr_vol,
                include_24hr_change: self.include_24hr_change,
                include_last_updated_at: self.include_last_updated_at,
                precision: self.precision,
            };
            Some(query_params)
        };
        let response = self
            .transport
            .send(method, &path, query_string.as_ref())
            .await?;
        Ok(response)
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

    pub fn token_price(
        &self,
        parts: SimpleTokenPriceParts<'a>,
        contract_address: &'a str,
        vs_currencies: &'a str,
    ) -> SimpleTokenPrice {
        SimpleTokenPrice::new(self.transport(), parts, contract_address, vs_currencies)
    }
}

impl CoinGecko {
    pub fn simple(&self) -> Simple {
        Simple::new(self.transport())
    }
}
