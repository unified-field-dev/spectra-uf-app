//! Shared time-range helpers for explore pages.

use chrono::{Duration, Utc};

pub fn range_from_secs(secs: i64) -> (chrono::DateTime<Utc>, chrono::DateTime<Utc>) {
    let end = Utc::now();
    let start = end - Duration::seconds(secs);
    (start, end)
}
