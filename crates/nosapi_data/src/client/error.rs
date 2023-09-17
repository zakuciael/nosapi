//! Contains error types returned by functions in the [`client`](crate::client) module

/// An error that occurs when extracting the [client version](crate::client::ClientVersion)
#[derive(thiserror::Error, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ClientVersionError {
  #[error("Invalid file provided, expected a valid pe32 file")]
  InvalidFile(#[from] pelite::Error),
  #[error("No resources section found in the executable file")]
  NoResources,
  #[error("Unable to find version info")]
  UnableToFindVersionInfo,
}
