pub mod client;
pub mod error;
pub mod ping;
pub mod response;
pub mod simple;
pub mod transport;

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
}
