use serde::{Deserializer, Serializer};
use serde_with::{DeserializeAs, SerializeAs};
use std::fmt::Display;

pub struct Base64;

impl<T> SerializeAs<T> for Base64
where
  T: Display,
{
  fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serde_with::base64::Base64::<serde_with::base64::Standard>::serialize_as(
      &source.to_string(),
      serializer,
    )
  }
}

impl<'de, T> DeserializeAs<'de, T> for Base64
where
  T: TryFrom<Vec<u8>>,
{
  fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
  where
    D: Deserializer<'de>,
  {
    serde_with::base64::Base64::<serde_with::base64::Standard>::deserialize_as(deserializer)
  }
}
