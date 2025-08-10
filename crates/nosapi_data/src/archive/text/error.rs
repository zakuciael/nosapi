use thiserror::Error;
#[derive(Error, Debug)]
#[error("Failed to parse the text archive")]
pub struct ArchiveError;

impl From<std::io::Error> for ArchiveError {
  fn from(_: std::io::Error) -> Self {
    ArchiveError
  }
}
