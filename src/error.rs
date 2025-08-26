use std::fmt;

#[derive(Debug)]
pub enum Error {
    RequestError(String),
    SerializationError(String),
    TransactionError(String),
    ValidationError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::RequestError(e) => write!(f, "Request error: {}", e),
            Error::SerializationError(e) => write!(f, "Serialization error: {}", e),
            Error::TransactionError(e) => write!(f, "Transaction error: {}", e),
            Error::ValidationError(e) => write!(f, "Validation error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::RequestError(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerializationError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
