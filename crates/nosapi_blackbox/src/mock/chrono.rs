pub struct Utc;

use std::cell::Cell;

thread_local! {
    static TIMESTAMP: Cell<i64> = const { Cell::new(0) };
}

impl Utc {
  pub fn now() -> chrono::DateTime<chrono::Utc> {
    TIMESTAMP
      .with(|timestamp| chrono::DateTime::<chrono::Utc>::from_timestamp_millis(timestamp.get()))
      .expect("a valid timestamp set")
  }
}

pub fn set_timestamp(timestamp: i64) {
  TIMESTAMP.set(timestamp);
}
