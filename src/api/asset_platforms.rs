use anyhow::Result;
use serde::Serialize;

use crate::api::response::Response;
use crate::api::transport::Transport;
use crate::api::Method;

use super::client::CoinGecko;

pub enum AssetPlatformsListParts {
    None,
}

impl<'b> AssetPlatformsListParts {
    fn url(&self) -> &'static str {
        match self {
            AssetPlatformsListParts::None => "/asset_platforms",
        }
    }
}

pub struct AssetPlatformsList<'a, 'b> {
    transport: &'a Transport,
    parts: AssetPlatformsListParts,
    filter: Option<&'b str>,
}

impl<'a, 'b> AssetPlatformsList<'a, 'b> {
    pub fn new(transport: &'a Transport, parts: AssetPlatformsListParts) -> Self {
        AssetPlatformsList {
            transport,
            parts,
            filter: None,
        }
    }

    pub fn filter(mut self, only_nft_support: bool) -> Self {
        if only_nft_support {
            self.filter = Some("nft")
        };
        self
    }

    pub async fn send(self) -> Result<Response> {
        let path = self.parts.url();
        let method = Method::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                filter: Option<&'b str>,
            }
            let query_params = QueryParams {
                filter: self.filter,
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

pub struct AssetPlatforms<'a> {
    transport: &'a Transport,
}

impl<'a> AssetPlatforms<'a> {
    fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }

    fn transport(&self) -> &Transport {
        self.transport
    }

    pub fn list(&self, parts: AssetPlatformsListParts) -> AssetPlatformsList {
        AssetPlatformsList::new(self.transport(), parts)
    }
}

impl CoinGecko {
    pub fn asset_platforms(&self) -> AssetPlatforms {
        AssetPlatforms::new(self.transport())
    }
}
