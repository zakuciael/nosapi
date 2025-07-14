//! Error types that can be returned when connecting to the API

use crate::header::HeaderMap;
use std::io;
use thiserror::Error;

pub use reqwest::Error as ClientError;

#[derive(Error, Debug)]
pub enum HttpError {
  #[error("status code: {status}")]
  Status { status: u16, headers: HeaderMap },
  #[error("request: {0}")]
  Client(ClientError),
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
