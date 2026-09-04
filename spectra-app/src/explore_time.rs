//! Shared time-range helpers for explore pages.

use chrono::{Duration, Utc};

/// Inclusive wall-clock window ending at now, spanning `secs` seconds.
pub fn range_from_secs(secs: i64) -> (chrono::DateTime<Utc>, chrono::DateTime<Utc>) {
    let end = Utc::now();
    let start = end - Duration::seconds(secs);
    (start, end)
}

#[cfg(test)]
mod tests {
    use super::range_from_secs;

    #[test]
    fn range_from_secs_spans_requested_duration_happy_path() {
        let (start, end) = range_from_secs(3_600);
        let delta = (end - start).num_seconds();
        assert_eq!(delta, 3_600);
        assert!(end >= start);
    }

    #[test]
    fn range_from_secs_zero_collapses_to_now_happy_path() {
        let (start, end) = range_from_secs(0);
        assert_eq!(start, end);
    }

    #[test]
    fn range_from_secs_negative_inverts_window_sad() {
        let (start, end) = range_from_secs(-60);
        assert!(start > end, "negative secs yields inverted window");
        assert_eq!((start - end).num_seconds(), 60);
    }
}
