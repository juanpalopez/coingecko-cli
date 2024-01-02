use crate::api::error::ClientError;
use crate::api::response::Response;
use crate::api::Method;
use anyhow::Result;
use url::Url;

#[derive(Debug)]
pub struct TransportBuilder {
    client_builder: reqwest::ClientBuilder,
    url: Url,
}

impl TransportBuilder {
    pub fn new(base_path: Url) -> Self {
        Self {
            client_builder: reqwest::Client::builder(),
            url: base_path,
        }
    }

    pub fn build(self) -> Result<Transport> {
        let new_client = self.client_builder.build();
        match new_client {
            Ok(res_client) => Ok(Transport {
                client: res_client,
                base_url: self.url,
            }),
            Err(err) => Err(ClientError::Build(err).into()),
        }
    }
}

pub struct Transport {
    client: reqwest::Client,
    base_url: Url,
}

impl Transport {
    pub async fn send(&self, method: Method, path: &str) -> Result<Response> {
        let url = self.base_url.join(path.trim_start_matches("/"))?;
        let response = self.client.get(url).send().await;

        match response {
            Ok(resp) => Ok(Response::new(resp, method)),
            Err(err) => Err(ClientError::Http(err).into()),
        }
    }
}
