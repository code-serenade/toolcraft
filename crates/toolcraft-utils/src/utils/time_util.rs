use std::time::{Duration, SystemTime, UNIX_EPOCH};

use time::{OffsetDateTime, format_description::well_known::Rfc3339};

pub fn get_current_timestamp_secs() -> i64 {
    let duration = get_current_duration();
    duration.as_secs() as i64
}

pub fn get_current_timestamp_millis() -> i64 {
    let duration = get_current_duration();
    duration.as_millis() as i64
}

pub fn timestamp_to_rfc3339(timestamp: i64) -> String {
    let datetime = timestamp_to_datetime(timestamp);
    datetime.format(&Rfc3339).unwrap()
}

pub fn timestamp_to_date(timestamp: i64) -> String {
    let datetime = timestamp_to_datetime(timestamp);
    datetime.date().to_string()
}

fn get_current_duration() -> Duration {
    let now = SystemTime::now();
    now.duration_since(UNIX_EPOCH).unwrap()
}

fn timestamp_to_datetime(timestamp: i64) -> OffsetDateTime {
    OffsetDateTime::from_unix_timestamp(timestamp).unwrap()
}
