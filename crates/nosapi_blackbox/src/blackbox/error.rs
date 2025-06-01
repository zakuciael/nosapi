//! Errors related to the `blackbox` string

/// An error emitted when decrypting a `blackbox` string
#[derive(thiserror::Error, Debug)]
pub enum DecryptBlackboxError {
  #[error("failed to decode base64: {0}")]
  Base64(#[from] base64::DecodeError),
  #[error("received invalid utf8: {0}")]
  Utf8(#[from] std::string::FromUtf8Error),
  #[error("failed to deserialize: {0}")]
  Serde(#[from] serde_plain::Error),
}

/// An error emitted while encrypting a `blackbox` string
#[derive(thiserror::Error, Debug)]
#[error("failed to encrypt: {0}")]
pub struct EncryptBlackboxError(#[from] serde_plain::Error);
