#[derive(thiserror::Error, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Error {
  #[error("Invalid file provided, expected a valid pe32 file")]
  InvalidFile(#[from] pelite::Error),
  #[error("No resources section found in the executable file")]
  NoResources,
  #[error("Unable to find version info")]
  UnableToFindVersionInfo,
}

pub type Result<T> = core::result::Result<T, Error>;
