use crate::nos::NOSFileType;

#[derive(thiserror::Error, Debug)]
#[error("{0}")]
pub struct NOSFileHeaderError(pub String);

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
