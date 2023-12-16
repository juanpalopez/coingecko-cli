pub mod client;
mod error;
mod ping;
mod response;
mod simple;
mod transport;

enum Method {
    Get,
    Post,
}
