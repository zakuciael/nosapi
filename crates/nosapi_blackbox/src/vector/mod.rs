mod error;
mod support;

use self::support::random_vector_string_data;
use crate::support::mockable::chrono::current_datetime;

use crate::vector::error::ParseVectorStringError;
use crate::vector::support::random_ascii_char;
use bon::Builder;
use chrono::Utc;
use chrono::{DateTime, TimeDelta};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::fmt::{Debug, Display, Formatter};
use std::ops::Add;
use std::str::{from_utf8, FromStr};

#[derive(Builder, DeserializeFromStr, SerializeDisplay, Clone, PartialEq, Debug)]
pub struct VectorString {
  #[builder(default = random_vector_string_data())]
  data: String,
  #[builder(default = current_datetime())]
  date_time: DateTime<Utc>,
}

impl VectorString {
  pub fn new(data: String, date_time: DateTime<Utc>) -> Self {
    Self { data, date_time }
  }

  pub fn parse<Value>(value: &Value) -> Result<VectorString, ParseVectorStringError>
  where
    Value: AsRef<str>,
  {
    let value = value.as_ref();
    let divider_index = value.rfind(' ').ok_or(ParseVectorStringError::NoDivider)?;

    let data = value[0..divider_index].to_string();
    let date_time =
      DateTime::<Utc>::from_timestamp_millis(i64::from_str(&value[divider_index + 1..])?)
        .ok_or(ParseVectorStringError::OutOfRangeTimestamp)?;

    Ok(VectorString { data, date_time })
  }

  pub fn data(&self) -> &str {
    &self.data
  }

  pub fn date_time(&self) -> &DateTime<Utc> {
    &self.date_time
  }

  pub fn update(mut self) -> Self {
    let current_datetime = current_datetime();

    if self.date_time.add(TimeDelta::milliseconds(1000)) < current_datetime {
      let mut data = String::with_capacity(self.data.len());
      data.extend(self.data.chars().skip(1));
      data.push(random_ascii_char());

      self.data = data;
      self.date_time = current_datetime;
    }

    self
  }
}

impl Default for VectorString {
  #[inline]
  fn default() -> Self {
    VectorString::builder().build()
  }
}

impl Display for VectorString {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {}", self.data, self.date_time.timestamp_millis())
  }
}

impl FromStr for VectorString {
  type Err = ParseVectorStringError;

  #[inline]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Self::parse(&s)
  }
}

impl TryFrom<String> for VectorString {
  type Error = ParseVectorStringError;

  #[inline]
  fn try_from(value: String) -> Result<Self, Self::Error> {
    Self::parse(&value)
  }
}

impl TryFrom<&str> for VectorString {
  type Error = ParseVectorStringError;

  #[inline]
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    Self::parse(&value)
  }
}

impl TryFrom<Vec<u8>> for VectorString {
  type Error = ParseVectorStringError;

  #[inline]
  fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
    Self::parse(&from_utf8(&value)?)
  }
}

impl TryFrom<&[u8]> for VectorString {
  type Error = ParseVectorStringError;

  #[inline]
  fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
    Self::parse(&from_utf8(value)?)
  }
}

#[cfg(test)]
mod tests {
  use crate::support::mockable::chrono::set_timestamp;
  use crate::support::mockable::rand::set_seed;
  use crate::vector::support::random_vector_string_data;
  use crate::vector::VectorString;
  use chrono::{DateTime, Utc};

  /* Fixtures */

  //noinspection SpellCheckingInspection
  #[rstest::fixture]
  fn vector_data() -> &'static str {
    "jniYEuIsry_5[y,9wS+tK^C'g_`tFmUTHYw|AuQ|IP8&ZAl7uA7TxF_b.Lv8a{i_L/EO?_c<KYKRC1p?sPk8o${Y|;>-9<qO'9n7"
  }

  #[rstest::fixture]
  fn vector_date_time() -> DateTime<Utc> {
    DateTime::from_timestamp_millis(1735390575328).unwrap()
  }

  #[rstest::fixture]
  fn vector_inst(vector_data: &str, vector_date_time: DateTime<Utc>) -> VectorString {
    VectorString::new(vector_data.to_string(), vector_date_time)
  }

  #[rstest::fixture]
  fn vector_string(vector_data: &str, vector_date_time: DateTime<Utc>) -> String {
    format!("{} {}", vector_data, vector_date_time.timestamp_millis())
  }

  #[rstest::fixture]
  fn invalid_vector_string(vector_data: &str, vector_date_time: DateTime<Utc>) -> String {
    format!("{}_**_{}", vector_data, vector_date_time.timestamp_millis())
  }

  /* Serialization & Deserialization */

  #[rstest::rstest]
  fn should_correctly_parse(vector_string: String, vector_inst: VectorString) {
    let res = VectorString::parse(&vector_string);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), vector_inst);
  }

  #[rstest::rstest]
  fn should_correctly_serialize(vector_inst: VectorString, vector_string: String) {
    assert_eq!(vector_inst.to_string(), vector_string);
  }

  #[rstest::rstest]
  fn should_correctly_serde_serialize(vector_inst: VectorString, vector_string: String) {
    let res = serde_plain::to_string(&vector_inst);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), vector_string);
  }

  #[rstest::rstest]
  fn should_correctly_serde_deserialize(vector_string: String, vector_inst: VectorString) {
    let res = serde_plain::from_str::<VectorString>(&vector_string);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), vector_inst);
  }

  #[rstest::rstest]
  fn should_fail_to_parse(invalid_vector_string: String) {
    assert!(VectorString::parse(&invalid_vector_string).is_err());
  }

  /* Getters */

  #[rstest::rstest]
  fn should_correctly_return_data(vector_inst: VectorString, vector_data: &str) {
    assert_eq!(vector_inst.data(), vector_data)
  }

  #[rstest::rstest]
  fn should_correctly_return_date_time(vector_inst: VectorString, vector_date_time: DateTime<Utc>) {
    assert_eq!(
      vector_inst.date_time().timestamp_millis(),
      vector_date_time.timestamp_millis()
    )
  }

  /* Logic */

  #[test]
  fn should_correctly_update() {
    // Set the random generator seed and the expected random character after the update
    let expected_rand_char = 'j';
    set_seed(6572086166531);

    // Set the initial timestamp for the current time
    let mut timestamp = 1735498856847000i64;
    set_timestamp(timestamp);

    // Generate vector data and instance
    let data = random_vector_string_data();
    let mut vector = VectorString::builder().data(data.clone()).build();

    // Advance the current time by 1500 milliseconds
    timestamp += 1500;
    set_timestamp(timestamp);

    // Update vector string
    vector = vector.update();

    // Assert that the date time got updated
    assert_eq!(vector.date_time().timestamp_millis(), timestamp);

    // Assert that the vector data got updated
    assert_eq!(&vector.data()[0..vector.data().len() - 1], &data[1..]);
    assert_eq!(vector.data().chars().last().unwrap(), expected_rand_char);
  }
}
