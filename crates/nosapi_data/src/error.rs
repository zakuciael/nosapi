use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
  #[error("TODO: Replace me")]
  Temp,
  #[error("IO Error: {0}")]
  Io(#[from] std::io::Error),
}
