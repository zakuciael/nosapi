use chrono::{DateTime, Utc};

#[cfg(test)]
thread_local! {
    static TIMESTAMP: std::cell::Cell<i64> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
pub fn set_timestamp(timestamp: i64) {
  TIMESTAMP.set(timestamp);
}

pub fn current_datetime() -> DateTime<Utc> {
  #[cfg(test)]
  {
    TIMESTAMP
      .with(|timestamp| DateTime::<Utc>::from_timestamp_millis(timestamp.get()))
      .expect("a valid timestamp")
  }
  #[cfg(not(test))]
  Utc::now()
}
