use thiserror::Error;

#[derive(Error, Debug)]
pub enum EncodeBlackboxError {
  #[error("failed to serialize fingerprint struct")]
  SerializationError(#[from] serde_json::Error),
  #[error("input is too big to be encoded")]
  InputTooBig,
}

#[derive(Error, Debug)]
pub enum DecodeBlackboxError {
  #[error("failed to decode base64 encoded string")]
  Base64DecodeError(#[from] base64::DecodeError),
  #[error("failed to deserialize fingerprint struct")]
  DeserializeError(#[from] serde_json::Error),
}

#[derive(Error, Debug)]
pub enum EncryptBlackboxError {
  #[error("the encrypted blackbox is too big to be base64 encoded")]
  TooBig,
  #[error("failed to encode blackbox")]
  EncodeBlackboxError(#[from] EncodeBlackboxError),
}

#[derive(Error, Debug)]
pub enum DecryptBlackboxError {
  #[error("failed to decode base64 encoded string")]
  Base64Error(#[from] base64::DecodeError),
  #[error("failed to decode blackbox")]
  DecodeBlackboxError(#[from] DecodeBlackboxError),
}
