use crate::api::Method;

pub struct Response {
    response: reqwest::Response,
    method: Method,
}
