use crate::api::error::Error;
use crate::api::response::Response;
use crate::api::Method;
use reqwest::Client;

pub struct Transport {}

impl Transport {
    pub async fn send(&self, method: Method, path: &str) -> Result<Response, Error> {
        let client = Client::new();
        let response = client.get(path).send().await;

        match response {
            Ok(resp) => Ok(Response::new(resp, method)),
            Err(err) => Err(err.into()),
        }

    }
}
