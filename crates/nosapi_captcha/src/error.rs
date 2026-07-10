//! Error types returned by the captcha client.

use crate::header::HeaderMap;
use std::io;
use thiserror::Error;

pub use reqwest::Error as ClientError;

/// Error returned by [`crate::Client`] methods.
///
/// Non-success HTTP responses are converted into [`HttpError::Status`] so
/// callers can inspect the status code and response headers. Request building,
/// transport, and JSON/body decoding errors are reported as [`HttpError::Client`].
#[derive(Error, Debug)]
pub enum HttpError {
    /// The API returned a non-success HTTP status.
    #[error("status code: {status}")]
    Status { status: u16, headers: HeaderMap },
    /// `reqwest` failed while building, sending, or decoding the request.
    #[error("request: {0}")]
    Client(ClientError),
    /// An I/O error occurred while reading response bodies.
    #[error("I/O: {0}")]
    IO(#[from] io::Error),
}

impl From<ClientError> for HttpError {
    fn from(value: ClientError) -> Self {
        match value.status() {
            Some(status) => HttpError::Status {
                status: status.as_u16(),
                headers: HeaderMap::new(),
            },
            None => HttpError::Client(value),
        }
    }
}

impl From<reqwest::Response> for HttpError {
    fn from(value: reqwest::Response) -> Self {
        HttpError::Status {
            status: value.status().as_u16(),
            headers: value.headers().clone(),
        }
    }
}
