use thiserror::Error as TError;

#[derive(Debug, TError)]
pub enum ClientError {
    #[error("reqwest error: {0}")]
    Http(reqwest::Error),
    #[error("reqwest error: {0}")]
    Serde(reqwest::Error),
    #[error("reqwest error: {0}")]
    Build(reqwest::Error),
    #[error("url error: {0}")]
    Url(url::ParseError),
}
