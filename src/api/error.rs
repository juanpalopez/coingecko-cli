#[derive(Debug)]
pub struct Error {
    error_type: ErrorType,
}

#[derive(Debug)]
enum ErrorType {
    Http(reqwest::Error),
    Url(url::ParseError),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error {
            error_type: ErrorType::Http(err),
        }
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error {
            error_type: ErrorType::Url(err),
        }
    }
}
