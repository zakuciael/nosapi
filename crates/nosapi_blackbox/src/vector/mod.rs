mod de;
mod ser;

use crate::utils;
use crate::utils::generate_vector_string;
use bon::Builder;
use chrono::{DateTime, TimeDelta, Utc};
use std::fmt::{Display, Formatter};
use std::ops::Add;

#[derive(Builder, Clone, PartialOrd, Debug)]
pub struct VectorString {
  #[builder(default = utils::generate_vector_string())]
  data: String,
  #[builder(default = chrono::Utc::now())]
  time: DateTime<Utc>,
}

impl VectorString {
  pub fn new(data: String, time: DateTime<Utc>) -> Self {
    Self { data, time }
  }

  pub fn update(&mut self) {
    let current_time = Utc::now();

    if current_time > self.time.add(TimeDelta::seconds(1)) {
      let mid = self.data[1..].to_owned();
      let rand_char = utils::random_ascii_char().to_string();

      self.data = mid + &rand_char;
      self.time = current_time;
    }
  }
}

impl PartialEq for VectorString {
  fn eq(&self, other: &Self) -> bool {
    self.data.eq(&other.data)
      && self
        .time
        .timestamp_millis()
        .eq(&other.time.timestamp_millis())
  }
}

impl Display for VectorString {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {}", self.data, self.time.timestamp_millis())
  }
}

impl Default for VectorString {
  fn default() -> Self {
    Self {
      data: generate_vector_string(),
      time: Utc::now(),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::vector::VectorString;
  use chrono::{DateTime, Utc};
  use serde::de::{Error, Unexpected};

  #[rstest::fixture]
  fn vector_inst() -> VectorString {
    VectorString::new(
      "ZZUCXxH2g0NCAg4Iwo0g5dP2XjE05TCj".to_string(),
      DateTime::<Utc>::from_timestamp_millis(1735170917000).unwrap(),
    )
  }

  #[rstest::fixture]
  fn vector_str() -> &'static str {
    "WlpVQ1h4SDJnME5DQWc0SXdvMGc1ZFAyWGpFMDVUQ2ogMTczNTE3MDkxNzAwMA=="
  }

  #[rstest::rstest]
  fn should_correctly_serialize(vector_inst: VectorString, vector_str: &'static str) {
    let res = serde_plain::to_string(&vector_inst);

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), vector_str)
  }

  #[rstest::rstest]
  fn should_correctly_deserialize(vector_inst: VectorString, vector_str: &'static str) {
    let res = serde_plain::from_str::<VectorString>(vector_str);

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), vector_inst)
  }

  #[rstest::rstest]
  #[case("this is not a valid base64 string", Error::invalid_value(Unexpected::Str(input), &"a base64 encoded string"))]
  #[case("WlpVQ1h4SDJnME5DQWc0SXdvMGc1ZFAyWGpFMDVUQ2oxNzM1MTcwOTE3MDAw", Error::invalid_value(Unexpected::Str("ZZUCXxH2g0NCAg4Iwo0g5dP2XjE05TCj1735170917000"), &"a string with a space separating values"))]
  #[case("WlpVQ1h4SDJnME5DQWc0SXdvMGc1ZFAyWGpFMDVUQ2ogbm90X2FfbnVtYmVy", Error::invalid_type(Unexpected::Str("not_a_number"), &"a signed 64-bit integer"))]
  #[case("WlpVQ1h4SDJnME5DQWc0SXdvMGc1ZFAyWGpFMDVUQ2ogOTIyMzM3MjAzNjg1NDc3NTgwNw==", Error::invalid_value(Unexpected::Signed(9223372036854775807), &"a valid timestamp"))]
  fn should_fail_when_deserializing(
    #[case] input: &'static str,
    #[case] expected: serde_plain::Error,
  ) {
    let res = serde_plain::from_str::<VectorString>(input);

    assert!(res.is_err());
    assert_eq!(&res.unwrap_err().to_string(), &expected.to_string())
  }
}
