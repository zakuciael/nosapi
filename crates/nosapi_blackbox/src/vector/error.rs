use std::num::ParseIntError;
use std::str::Utf8Error;
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Error, Clone, PartialEq, Eq, Debug)]
pub enum ParseVectorStringError {
  #[error("no data divider found")]
  NoDivider,
  #[error("invalid timestamp")]
  InvalidTimestamp(#[from] ParseIntError),
  #[error("timestamp is out-of-range")]
  OutOfRangeTimestamp,
  #[error("invalid utf8 values")]
  InvalidUtf8,
}

impl From<Utf8Error> for ParseVectorStringError {
  fn from(_: Utf8Error) -> Self {
    ParseVectorStringError::InvalidUtf8
  }
}

impl From<FromUtf8Error> for ParseVectorStringError {
  fn from(_: FromUtf8Error) -> Self {
    ParseVectorStringError::InvalidUtf8
  }
}
