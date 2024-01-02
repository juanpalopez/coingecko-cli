use anyhow::Result;
use serde::de::DeserializeOwned;

use crate::api::error::ClientError;
use crate::api::Method;

#[derive(Debug)]
pub struct Response {
    response: reqwest::Response,
    method: Method,
}

impl Response {
    pub fn new(response: reqwest::Response, method: Method) -> Self {
        Self { response, method }
    }
    pub fn response(&self) -> &reqwest::Response {
        &self.response
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub async fn text(self) -> Result<String> {
        let body = self.response.text().await;
        match body {
            Ok(res) => Ok(res),
            Err(err) => Err(ClientError::Serde(err).into()),
        }
    }

    pub async fn json<B>(self) -> Result<B>
    where
        B: DeserializeOwned,
    {
        let body = self.response.json::<B>().await;
        match body {
            Ok(res) => Ok(res),
            Err(err) => Err(ClientError::Serde(err).into()),
        }
    }
}
