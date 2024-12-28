use super::VectorString;
use base64::Engine;
use serde::{Serialize, Serializer};

impl Serialize for VectorString {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let str = format!("{} {}", self.data, self.time.timestamp_millis());
    let encoded = base64::engine::general_purpose::STANDARD.encode(&str);

    serializer.serialize_str(&encoded)
  }
}
