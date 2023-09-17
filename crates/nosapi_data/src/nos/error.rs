//! Contains error types returned by functions in the [`nos`](crate::nos) module

use crate::nos::NOSFileType;

/// An error that occurs when parsing the [file type](NOSFileType)
#[derive(thiserror::Error, Debug)]
#[error("{0}")]
pub struct NOSFileHeaderError(pub String);

/// An error that occurs when parsing the `.NOS` file
#[derive(thiserror::Error, Debug)]
pub enum NOSFileError {
  #[error("Invalid file header detected")]
  InvalidFileHeader(#[from] NOSFileHeaderError),
  #[error("Invalid file type detected, expected {expected:?} got {received:?}")]
  InvalidFileType {
    expected: NOSFileType,
    received: NOSFileType,
  },
  #[error("An IO error occurred")]
  IO(#[from] std::io::Error),
}
