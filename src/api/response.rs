use crate::api::Method;

pub struct Response {
    response: reqwest::Response,
    method: Method,
}

impl Response {
    pub fn new(response: reqwest::Response, method: Method) -> Self {
        Response{
            response,
            method
        }
    }
}
