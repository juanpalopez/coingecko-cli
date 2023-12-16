pub struct Error {
    error_type: ErrorType,
}

enum ErrorType {
    Http(reqwest::Error),
}
