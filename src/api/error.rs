pub struct Error {
    error_type: ErrorType,
}

enum ErrorType {
    Http(reqwest::Error),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error {
            error_type: ErrorType::Http(err)
        }
    }
}
