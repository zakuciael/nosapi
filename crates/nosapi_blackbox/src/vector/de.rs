use super::Vector;
use base64::Engine;
use chrono::{DateTime, Utc};
use serde::de::{Error, Unexpected, Visitor};
use serde::{Deserialize, Deserializer};

struct VectorVisitor;

#[allow(clippy::needless_lifetimes)]
impl<'de> Visitor<'de> for VectorVisitor {
  type Value = Vector;

  fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "vector string")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: Error,
  {
    let decoded = {
      let buf = base64::engine::general_purpose::STANDARD
        .decode(v)
        .map_err(|_| Error::invalid_value(Unexpected::Str(v), &"a base64 encoded string"))?;

      String::from_utf8(buf).map_err(|err| {
        Error::invalid_value(Unexpected::Bytes(err.as_bytes()), &"a valid utf8 string")
      })?
    };

    let divider_index = decoded.rfind(' ').ok_or(Error::invalid_value(
      Unexpected::Str(&decoded),
      &"a string with a space separating values",
    ))?;

    let data = decoded[0..divider_index].to_owned();
    let time = {
      let raw = &decoded[divider_index + 1..];
      let value = raw
        .parse::<i64>()
        .map_err(|_| Error::invalid_type(Unexpected::Str(raw), &"a signed 64-bit integer"))?;

      DateTime::<Utc>::from_timestamp_millis(value).ok_or(Error::invalid_value(
        Unexpected::Signed(value),
        &"a valid timestamp",
      ))?
    };

    Ok(Vector::new(data, time))
  }
}

impl<'de> Deserialize<'de> for Vector {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_str(VectorVisitor)
  }
}
