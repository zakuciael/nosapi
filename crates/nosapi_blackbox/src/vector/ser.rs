use super::VectorString;
use base64::Engine;
use serde::{Serialize, Serializer};

impl Serialize for VectorString {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&base64::engine::general_purpose::STANDARD.encode(&self.to_string()))
  }
}
