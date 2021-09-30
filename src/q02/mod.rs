///
/// http://llever.com/exercism-rust-zh/gigasecond/README.zh.html
///
extern crate chrono;
use chrono::{DateTime, Duration, Utc};

pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::seconds(1_000_000_000)
}
