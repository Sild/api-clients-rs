use thiserror::Error;

/// Errors produced while building a client or executing an API request.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ApiClientsError {
    /// The request failed before an HTTP response was available, for example
    /// because of a connection, timeout, request, or response-body failure.
    #[error("NetworkError: {0}")]
    Network(String),
    /// The remote service returned an HTTP 5xx response.
    #[error("ServerError: code: {0}, msg: {1}")]
    Server(u16, String),
    /// The remote service returned an HTTP 4xx response.
    #[error("ClientError: code: {0}, msg: {1}")]
    Client(u16, String),
    /// An error did not match a more specific public classification.
    #[error("UnknownError: {0}")]
    Unknown(String),
    /// The client or executor could not be constructed.
    #[error("InternalError: {0}")]
    Internal(String),
    /// A successful HTTP or GraphQL response could not be interpreted as the
    /// response type required by the request.
    #[error("UnexpectedResponse: {0}")]
    UnexpectedResponse(String),
    /// Request parameters could not be serialized or otherwise validated.
    #[error("InvalidArgs: {0}")]
    InvalidArgs(String),
}

impl From<reqwest::Error> for ApiClientsError {
    fn from(err: reqwest::Error) -> Self {
        let status = err.status();
        let is_network = err.is_connect() || err.is_timeout() || err.is_request() || err.is_body();
        classify_http_error(status, is_network, err.to_string())
    }
}

impl From<reqwest_middleware::Error> for ApiClientsError {
    fn from(err: reqwest_middleware::Error) -> Self {
        let status = err.status();
        let is_network = err.is_connect() || err.is_timeout() || err.is_request() || err.is_body();
        classify_http_error(status, is_network, err.to_string())
    }
}

fn classify_http_error(status: Option<http::StatusCode>, is_network: bool, message: String) -> ApiClientsError {
    match status.map(|status| status.as_u16()) {
        Some(code @ 400..=499) => ApiClientsError::Client(code, message),
        Some(code @ 500..=599) => ApiClientsError::Server(code, message),
        _ if is_network => ApiClientsError::Network(message),
        _ => ApiClientsError::Unknown(message),
    }
}

/// Result type returned by the shared executor and service clients.
pub type ApiClientsResult<T> = Result<T, ApiClientsError>;

#[cfg(test)]
mod tests {
    use super::{classify_http_error, ApiClientsError};
    use http::StatusCode;

    #[test]
    fn test_classifies_statusless_transport_error_as_network() {
        let error = classify_http_error(None, true, "request timed out".to_string());

        assert!(matches!(error, ApiClientsError::Network(message) if message == "request timed out"));
    }

    #[test]
    fn test_classifies_http_error_statuses() {
        let client = classify_http_error(Some(StatusCode::BAD_REQUEST), false, "bad request".to_string());
        let server = classify_http_error(Some(StatusCode::SERVICE_UNAVAILABLE), false, "unavailable".to_string());

        assert!(matches!(client, ApiClientsError::Client(400, message) if message == "bad request"));
        assert!(matches!(server, ApiClientsError::Server(503, message) if message == "unavailable"));
    }

    #[test]
    fn test_preserves_unknown_non_transport_errors() {
        let error = classify_http_error(None, false, "middleware failed".to_string());

        assert!(matches!(error, ApiClientsError::Unknown(message) if message == "middleware failed"));
    }
}
