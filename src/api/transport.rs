use crate::api::error::Error;
use crate::api::response::Response;
use crate::api::Method;
use url::Url;

pub enum BuildError {
    Http(reqwest::Error),
}

impl From<reqwest::Error> for BuildError {
    fn from(err: reqwest::Error) -> BuildError {
        BuildError::Http(err)
    }
}

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

    pub fn build(self) -> Result<Transport, reqwest::Error> {
        let new_client = self.client_builder.build()?;
        Ok(Transport {
            client: new_client,
            base_url: self.url,
        })
    }
}

pub struct Transport {
    client: reqwest::Client,
    base_url: Url,
}

impl Transport {
    pub async fn send(&self, method: Method, path: &str) -> Result<Response, Error> {
        let url = self.base_url.join(path.trim_start_matches("/"))?;
        let response = self.client.get(url).send().await;

        match response {
            Ok(resp) => Ok(Response::new(resp, method)),
            Err(err) => Err(err.into()),
        }
    }
}
