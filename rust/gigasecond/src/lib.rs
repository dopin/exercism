use chrono::{DateTime, Duration, Utc};

pub fn after(date: DateTime<Utc>) -> DateTime<Utc> {
    date + Duration::seconds(1_000_000_000)
}
