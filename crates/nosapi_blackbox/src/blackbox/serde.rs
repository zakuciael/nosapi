use super::Blackbox;
use serde::de::Error;
use serde::{Deserializer, Serializer};
use std::fmt::Formatter;

impl serde::Serialize for Blackbox {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(
      &self
        .encode()
        .map_err(|err| serde::ser::Error::custom(err.to_string()))?,
    )
  }
}

impl<'de> serde::Deserialize<'de> for Blackbox {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct Visitor;
    impl serde::de::Visitor<'_> for Visitor {
      type Value = Blackbox;

      fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "a blackbox string")
      }

      fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
      where
        E: Error,
      {
        Blackbox::decode(v).map_err(|err| Error::custom(err.to_string()))
      }
    }

    deserializer.deserialize_str(Visitor)
  }
}
