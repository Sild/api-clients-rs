use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ApiClientsError {
    #[error("NetworkError: code: {0}, msg: {1}")]
    Network(u16, String),
    #[error("ServerError: code: {0}, msg: {1}")]
    Server(u16, String),
    #[error("ClientError: code: {0}, msg: {1}")]
    Client(u16, String),
    #[error("UnknownError: {0}")]
    Unknown(String),
    #[error("InternalError: {0}")]
    Internal(String),
    #[error("UnexpectedResponse: {0}")]
    UnexpectedResponse(String),
    #[error("InvalidArgs: {0}")]
    InvalidArgs(String),
}

impl From<reqwest::Error> for ApiClientsError {
    fn from(err: reqwest::Error) -> Self {
        if let Some(status) = err.status() {
            match status.as_u16() {
                400..=499 => ApiClientsError::Client(status.as_u16(), err.to_string()),
                500..=599 => ApiClientsError::Server(status.as_u16(), err.to_string()),
                _ => ApiClientsError::Network(status.as_u16(), err.to_string()),
            }
        } else {
            ApiClientsError::Unknown(err.to_string())
        }
    }
}

impl From<reqwest_middleware::Error> for ApiClientsError {
    fn from(err: reqwest_middleware::Error) -> Self {
        if let Some(status) = err.status() {
            match status.as_u16() {
                400..=499 => ApiClientsError::Client(status.as_u16(), err.to_string()),
                500..=599 => ApiClientsError::Server(status.as_u16(), err.to_string()),
                _ => ApiClientsError::Network(status.as_u16(), err.to_string()),
            }
        } else {
            ApiClientsError::Unknown(err.to_string())
        }
    }
}

pub type ApiClientsResult<T> = Result<T, ApiClientsError>;
