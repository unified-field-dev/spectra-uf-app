//! Pure backend contracts for the Spectra UF app server surface.
//!
//! Leptos `#[server]` entrypoints in `spectra-app` resolve Higgs request context,
//! then call these helpers so catalog/query shapes stay unit- and
//! integration-testable without a full host or UI graph.

use spectra_core::{
    list_schemas, rows_to_event_result, schema_detail, EventAggregateResult, EventQueryResult,
    MetricsQueryResult, SchemaDetailDto, SchemaListItem,
};

/// Rejects blank table/metric names before host permission checks run.
///
/// # Errors
///
/// Returns an error message when `table` is empty or whitespace-only.
pub fn validate_spectra_query_name(table: &str) -> Result<(), String> {
    if table.trim().is_empty() {
        Err("Spectra query table name is required".to_string())
    } else {
        Ok(())
    }
}

/// Gauge permission name for querying a Spectra table or metric.
#[must_use]
pub fn spectra_query_permission_name(table: &str) -> String {
    format!("spectra.query.{}", table.trim())
}

/// Catalog listing used by `list_schema_metadata` after request context resolves.
#[must_use]
pub fn schema_metadata_list() -> Vec<SchemaListItem> {
    list_schemas()
}

/// Schema detail used by `get_schema_metadata` after request context resolves.
#[must_use]
pub fn schema_metadata_detail(name: &str) -> Option<SchemaDetailDto> {
    schema_detail(name)
}

/// Empty metrics explore payload returned until the host injects a live backend.
#[must_use]
pub fn empty_metrics_query_result() -> MetricsQueryResult {
    MetricsQueryResult {
        series: Vec::new(),
        headline: Vec::new(),
    }
}

/// Empty event-log payload for a table (unknown schema → default `ts` column).
#[must_use]
pub fn empty_event_query_result(table: &str) -> EventQueryResult {
    rows_to_event_result(table, Vec::new(), 0)
}

/// Empty time-series aggregate payload returned until the host injects a live backend.
#[must_use]
pub fn empty_event_aggregate_result() -> EventAggregateResult {
    EventAggregateResult::TimeSeries {
        series: Vec::new(),
        headline: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        empty_event_aggregate_result, empty_event_query_result, empty_metrics_query_result,
        schema_metadata_detail, schema_metadata_list, spectra_query_permission_name,
        validate_spectra_query_name,
    };
    use spectra_core::EventAggregateResult;

    #[test]
    fn spectra_query_permission_name_formats_table_happy_path() {
        assert_eq!(
            spectra_query_permission_name("my_events"),
            "spectra.query.my_events"
        );
        assert_eq!(
            spectra_query_permission_name("  metric_a  "),
            "spectra.query.metric_a"
        );
    }

    #[test]
    fn schema_metadata_list_returns_vec_happy_path() {
        let items = schema_metadata_list();
        assert!(items.iter().all(|i| !i.table_or_metric.is_empty()));
    }

    #[test]
    fn schema_metadata_detail_unknown_is_none_sad() {
        assert!(schema_metadata_detail("__spectra_backend_missing_schema__").is_none());
    }

    #[test]
    fn empty_metrics_query_result_happy_path() {
        let r = empty_metrics_query_result();
        assert!(r.series.is_empty());
        assert!(r.headline.is_empty());
    }

    #[test]
    fn empty_event_query_result_unknown_table_has_ts_column_happy_path() {
        let r = empty_event_query_result("__spectra_backend_missing_table__");
        assert!(r.rows.is_empty());
        assert_eq!(r.row_count, 0);
        assert_eq!(r.columns.len(), 1);
        assert_eq!(r.columns[0].field, "ts");
    }

    #[test]
    fn empty_event_aggregate_result_is_empty_timeseries_happy_path() {
        match empty_event_aggregate_result() {
            EventAggregateResult::TimeSeries { series, headline } => {
                assert!(series.is_empty());
                assert!(headline.is_empty());
            }
            other => panic!("expected TimeSeries stub, got {other:?}"),
        }
    }

    #[test]
    fn validate_spectra_query_name_rejects_blank_sad() {
        let err = validate_spectra_query_name("").expect_err("blank");
        assert!(err.contains("required"), "{err}");
        let err = validate_spectra_query_name("   ").expect_err("whitespace");
        assert!(err.contains("required"), "{err}");
    }

    #[test]
    fn validate_spectra_query_name_accepts_non_empty_happy_path() {
        validate_spectra_query_name("events").expect("non-empty");
        validate_spectra_query_name("  metrics.latency  ").expect("trimmed non-empty");
    }
}
