//! Pure backend contracts for the Spectra UF app server surface.
//!
//! Leptos `#[server]` entrypoints in `spectra-app` resolve Higgs request context,
//! then call these helpers so catalog/query shapes stay unit- and
//! integration-testable without a full host or UI graph.
//!
//! ## Organized by task
//!
//! | Task | Start here |
//! |------|------------|
//! | **Validate explore table names** | [`SpectraQueryNameError`], [`validate_spectra_query_name`] |
//! | **Gauge permission names** | [`spectra_query_permission_name`] |
//! | **Schema catalog list/detail** | [`schema_metadata_list`], [`schema_metadata_detail`] |
//! | **Empty explore stubs** | [`empty_event_query_result`], [`empty_event_aggregate_result`], [`empty_metrics_query_result`] |
//! | **UI pages / `#[server]` wrappers** | `spectra-app` (not this crate) |
//!
//! ## Owns / does not own
//!
//! **Owns:** Pure validation, Gauge name formatting, schema catalog helpers, and
//! empty explore-query stub payloads used by the Spectra ops UI server surface.
//!
//! **Does not own:** Leptos pages, Higgs `#[server]` wrappers, or route registration
//! (`spectra-app`); Spectra core storage or live query backends (Spectra core / host).
//!
//! ## Concern → API
//!
//! | Concern | API | Owner |
//! |---------|-----|-------|
//! | Table/metric name validation | [`SpectraQueryNameError`], [`validate_spectra_query_name`] | this crate |
//! | Gauge `spectra.query.{table}` | [`spectra_query_permission_name`] | this crate |
//! | Schema catalog list/detail | [`schema_metadata_list`], [`schema_metadata_detail`] | this crate |
//! | Event explore stub | [`empty_event_query_result`], [`empty_event_aggregate_result`] | this crate |
//! | Metric explore stub | [`empty_metrics_query_result`] | this crate |
//! | Pages, routes, server fns | `spectra-app` (`SpectraRoutes`) | `spectra-app` |
//!
//! ## Examples ladder
//!
//! | Level | Where |
//! |-------|--------|
//! | Highlight | Concern → API table above |
//! | Mid | This crate's unit + integ suites (`docs/VERIFICATION.md`) |
//! | Detailed | `examples/protected-spectra-host` (inventory `spectra` / `/spectra`; copy README) |

use spectra_core::{
    list_schemas, rows_to_event_result, schema_detail, EventAggregateResult, EventQueryResult,
    MetricsQueryResult, SchemaDetailDto, SchemaListItem,
};

/// Blank table/metric name rejected before Gauge permission checks or explore stubs.
///
/// Callers map this into Leptos `ServerFnError` (or equivalent) at the `#[server]`
/// boundary; the Display text stays stable for UI and contract tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SpectraQueryNameError {
    /// Table or metric name was empty or whitespace-only.
    EmptyTableName,
}

impl std::fmt::Display for SpectraQueryNameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyTableName => write!(f, "Spectra query table name is required"),
        }
    }
}

impl std::error::Error for SpectraQueryNameError {}

/// Rejects blank table/metric names before host permission checks run.
///
/// # Errors
///
/// Returns [`SpectraQueryNameError::EmptyTableName`] when `table` is empty or
/// whitespace-only.
pub fn validate_spectra_query_name(table: &str) -> Result<(), SpectraQueryNameError> {
    if table.trim().is_empty() {
        Err(SpectraQueryNameError::EmptyTableName)
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
        validate_spectra_query_name, SpectraQueryNameError,
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
        assert_eq!(
            validate_spectra_query_name("").expect_err("blank"),
            SpectraQueryNameError::EmptyTableName
        );
        assert_eq!(
            validate_spectra_query_name("   ").expect_err("whitespace"),
            SpectraQueryNameError::EmptyTableName
        );
        assert!(SpectraQueryNameError::EmptyTableName
            .to_string()
            .contains("required"));
    }

    #[test]
    fn validate_spectra_query_name_accepts_non_empty_happy_path() {
        validate_spectra_query_name("events").expect("non-empty");
        validate_spectra_query_name("  metrics.latency  ").expect("trimmed non-empty");
    }
}
