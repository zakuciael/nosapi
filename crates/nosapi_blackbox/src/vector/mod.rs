use crate::utils;
use crate::utils::generate_vector_string;
use bon::bon;
use chrono::{DateTime, TimeDelta};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use std::ops::Add;
use std::str::FromStr;
use std::string::FromUtf8Error;

#[cfg(test)]
use crate::mock::chrono::Utc;
#[cfg(not(test))]
use chrono::Utc;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct VectorString(pub String);

#[bon]
impl VectorString {
  pub fn new(data: String, timestamp: DateTime<chrono::Utc>) -> Self {
    VectorString(Self::format(data, timestamp))
  }

  #[builder]
  pub fn builder(
    #[builder(default = generate_vector_string())] data: String,
    #[builder(default = Utc::now())] timestamp: DateTime<chrono::Utc>,
  ) -> Self {
    VectorString::new(data, timestamp)
  }

  pub fn data(&self) -> Option<String> {
    let divider_index = self.divider_index()?;
    Some(self.0[0..divider_index].to_string())
  }

  pub fn timestamp(&self) -> Option<DateTime<chrono::Utc>> {
    let divider_index = self.divider_index()?;
    let raw = &self.0[divider_index + 1..].to_string();

    DateTime::<chrono::Utc>::from_timestamp_millis(i64::from_str(raw).ok()?)
  }

  pub fn update(&mut self) -> Option<()> {
    let last_time = self.timestamp()?;
    let data = self.data()?;
    let current_time = Utc::now();

    if current_time > last_time.add(TimeDelta::seconds(1)) {
      let mid = data[1..].to_string();
      let rand_char = utils::random_ascii_char().to_string();

      self.0 = Self::format(mid + &rand_char, current_time)
    }

    Some(())
  }

  fn divider_index(&self) -> Option<usize> {
    self.0.rfind(' ')
  }

  fn format(data: String, time: DateTime<chrono::Utc>) -> String {
    format!("{} {}", data, time.timestamp_millis())
  }
}

impl Display for VectorString {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", &self.0)
  }
}

impl Default for VectorString {
  fn default() -> Self {
    VectorString(Self::format(generate_vector_string(), Utc::now()))
  }
}

impl AsRef<[u8]> for VectorString {
  fn as_ref(&self) -> &[u8] {
    self.0.as_bytes()
  }
}

impl TryFrom<Vec<u8>> for VectorString {
  type Error = FromUtf8Error;

  fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
    Ok(VectorString(String::from_utf8(value)?))
  }
}

impl TryFrom<&[u8]> for VectorString {
  type Error = FromUtf8Error;

  fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
    Ok(VectorString(String::from_utf8(value.to_vec())?.to_string()))
  }
}

impl From<String> for VectorString {
  fn from(value: String) -> Self {
    VectorString(value)
  }
}

impl From<&str> for VectorString {
  fn from(value: &str) -> Self {
    VectorString(value.to_string())
  }
}

#[cfg(test)]
mod tests {
  use crate::mock::chrono::set_timestamp;
  use crate::mock::rand::set_seed;
  use crate::utils::generate_vector_string;
  use crate::vector::VectorString;
  use chrono::{DateTime, Utc};

  #[rstest::fixture]
  fn vector_data() -> &'static str {
    "jniYEuIsry_5[y,9wS+tK^C'g_`tFmUTHYw|AuQ|IP8&ZAl7uA7TxF_b.Lv8a{i_L/EO?_c<KYKRC1p?sPk8o${Y|;>-9<qO'9n7"
  }

  #[rstest::fixture]
  fn vector_timestamp() -> DateTime<Utc> {
    DateTime::from_timestamp_millis(1735390575328).unwrap()
  }

  #[rstest::fixture]
  fn vector_inst(vector_data: &str, vector_timestamp: DateTime<Utc>) -> VectorString {
    VectorString::new(vector_data.to_string(), vector_timestamp)
  }

  #[rstest::fixture]
  fn invalid_vector_inst(vector_data: &str, vector_timestamp: DateTime<Utc>) -> VectorString {
    VectorString(format!(
      "{}__{}",
      vector_data,
      vector_timestamp.timestamp_millis()
    ))
  }

  #[rstest::fixture]
  fn vector_string(vector_data: &str, vector_timestamp: DateTime<Utc>) -> String {
    format!("{} {}", vector_data, vector_timestamp.timestamp_millis())
  }

  #[rstest::rstest]
  fn should_correctly_serialize(vector_inst: VectorString, vector_string: String) {
    let res = serde_plain::to_string(&vector_inst);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), vector_string);
  }

  #[rstest::rstest]
  fn should_correctly_deserialize(vector_string: String, vector_inst: VectorString) {
    let res = serde_plain::from_str::<VectorString>(&vector_string);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), vector_inst);
  }

  #[rstest::rstest]
  fn should_correctly_return_data(vector_inst: VectorString, vector_data: &str) {
    assert!(vector_inst.data().is_some());
    assert_eq!(vector_inst.data().unwrap(), vector_data)
  }

  #[rstest::rstest]
  fn should_correctly_return_timestamp(vector_inst: VectorString, vector_timestamp: DateTime<Utc>) {
    assert!(vector_inst.timestamp().is_some());
    assert_eq!(
      vector_inst.timestamp().unwrap().timestamp_millis(),
      vector_timestamp.timestamp_millis()
    )
  }

  #[test]
  fn should_correctly_update() {
    // Set the random generator seed and the expected random character after the update
    let expected_rand_char = "j";
    set_seed(6572086166531);

    // Set the initial timestamp for the current time
    let mut timestamp = 1735498856847000i64;
    set_timestamp(timestamp);

    // Generate vector data and instance
    let data = generate_vector_string();
    let mut vector = VectorString::builder().data(data.clone()).build();

    // Advance the current time by 1500 milliseconds
    timestamp += 1500;
    set_timestamp(timestamp);

    // Assert that the update succeeded
    assert!(vector.update().is_some());

    // Assert that the timestamp got updated
    assert_eq!(vector.timestamp().unwrap().timestamp_millis(), timestamp);

    // Assert that the vector data got updated
    assert_eq!(
      vector.data().unwrap(),
      data[1..].to_owned() + expected_rand_char
    )
  }

  #[rstest::rstest]
  fn should_fail_returning_data(invalid_vector_inst: VectorString) {
    assert!(invalid_vector_inst.data().is_none());
  }

  #[rstest::rstest]
  fn should_fail_returning_timestamp(invalid_vector_inst: VectorString) {
    assert!(invalid_vector_inst.timestamp().is_none());
  }
}
