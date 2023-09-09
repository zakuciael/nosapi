use core::fmt;

use crate::nos::NOSFileType;

#[derive(thiserror::Error, Debug)]
pub struct CustomError(String);

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("Invalid file header detected")]
  InvalidFileHeader(#[source] CustomError),
  #[error("Invalid file type detected, expected {expected:?} got {received:?}")]
  InvalidFileType {
    expected: NOSFileType,
    received: NOSFileType,
  },
  #[error("Invalid OLE time detected")]
  InvalidOLETime(#[from] time::error::ComponentRange),
  #[error("Unable to read the file due to an IO error")]
  IO(#[from] std::io::Error),
}

pub type Result<T> = core::result::Result<T, Error>;

impl CustomError {
  pub fn new(desc: String) -> Self {
    Self(desc)
  }
}

impl fmt::Display for CustomError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}
