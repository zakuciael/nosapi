use chrono::{DateTime, Utc};

#[cfg(test)]
thread_local! {
    static TIMESTAMP: std::cell::Cell<i64> = const { std::cell::Cell::new(0) };
}

/// A setter for the mock `timestamp` used in the [`current_datetime`] function.
#[cfg(test)]
pub fn set_timestamp(timestamp: i64) {
  TIMESTAMP.set(timestamp);
}

/// A function for generating a mockable current UTC date time.
pub fn current_datetime() -> DateTime<Utc> {
  #[cfg(test)]
  {
    TIMESTAMP
      .with(|timestamp| DateTime::<Utc>::from_timestamp_millis(timestamp.get()))
      .expect("a valid mock timestamp")
  }
  #[cfg(not(test))]
  Utc::now()
}
