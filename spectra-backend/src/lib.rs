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
//! | **Validate explore table names** | [`SpectraQueryNameError`], [`validate_spectra_query_name`], [`MAX_SPECTRA_QUERY_NAME_CHARS`] |
//! | **Gauge permission names** | [`spectra_query_permission_name`] |
//! | **Ops path encoding** | [`encode_ops_path_segment`], [`spectra_schema_path`], [`spectra_schema_explore_path`], [`spectra_metric_explore_path`] |
//! | **Schema catalog list/detail** | [`schema_metadata_list`], [`schema_metadata_detail`] |
//! | **Empty explore stubs** | [`empty_event_query_result`], [`empty_event_aggregate_result`], [`empty_metrics_query_result`] |
//! | **UI pages / `#[server]` wrappers** | `spectra-app` (not this crate) |
//!
//! ## Owns / does not own
//!
//! **Owns:** Pure validation, Gauge name formatting, ops path helpers, schema
//! catalog helpers, and empty explore-query stub payloads used by the Spectra
//! ops UI server surface.
//!
//! **Does not own:** Leptos pages, Higgs `#[server]` wrappers, or route registration
//! (`spectra-app`); Spectra core storage or live query backends (Spectra core / host).
//!
//! ## Concern → API
//!
//! | Concern | API | Owner |
//! |---------|-----|-------|
//! | Table/metric name validation | [`SpectraQueryNameError`], [`validate_spectra_query_name`], [`MAX_SPECTRA_QUERY_NAME_CHARS`] | this crate |
//! | Gauge `spectra.query.{table}` | [`spectra_query_permission_name`] | this crate |
//! | Ops path encoding | [`encode_ops_path_segment`], [`spectra_schema_path`], [`spectra_schema_explore_path`], [`spectra_metric_explore_path`] | this crate |
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

/// Maximum Unicode scalar count for table/metric names accepted by explore and
/// schema detail lookups.
pub const MAX_SPECTRA_QUERY_NAME_CHARS: usize = 256;

/// Blank, oversized, or path-unsafe table/metric name rejected before Gauge
/// permission checks or explore stubs.
///
/// Callers map this into Leptos `ServerFnError` (or equivalent) at the `#[server]`
/// boundary; the Display text stays stable for UI and contract tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SpectraQueryNameError {
    /// Table or metric name was empty or whitespace-only.
    EmptyTableName,
    /// Table or metric name exceeded [`MAX_SPECTRA_QUERY_NAME_CHARS`].
    TableNameTooLong,
    /// Table or metric name contained `/`, `\`, ASCII controls, or was `.` / `..`.
    UnsafeTableName,
}

impl std::fmt::Display for SpectraQueryNameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyTableName => write!(f, "Spectra query table name is required"),
            Self::TableNameTooLong => write!(
                f,
                "Spectra query table name exceeds {MAX_SPECTRA_QUERY_NAME_CHARS} characters"
            ),
            Self::UnsafeTableName => write!(
                f,
                "Spectra query table name contains unsafe path characters"
            ),
        }
    }
}

impl std::error::Error for SpectraQueryNameError {}

const fn is_unsafe_ops_id_char(c: char) -> bool {
    c.is_control() || c == '/' || c == '\\'
}

/// Rejects blank, oversized, path-separating, control, or `.` / `..` table/metric
/// names before host permission checks run.
///
/// Dotted names such as `ops.events` remain valid; only the exact segments `.`
/// and `..` are rejected.
///
/// # Errors
///
/// Returns a [`SpectraQueryNameError`] variant when `table` is empty/whitespace-only,
/// longer than [`MAX_SPECTRA_QUERY_NAME_CHARS`], contains `/` `\` or ASCII controls,
/// or is exactly `.` / `..`.
pub fn validate_spectra_query_name(table: &str) -> Result<(), SpectraQueryNameError> {
    let trimmed = table.trim();
    if trimmed.is_empty() {
        return Err(SpectraQueryNameError::EmptyTableName);
    }
    if trimmed.chars().count() > MAX_SPECTRA_QUERY_NAME_CHARS {
        return Err(SpectraQueryNameError::TableNameTooLong);
    }
    if trimmed == "." || trimmed == ".." {
        return Err(SpectraQueryNameError::UnsafeTableName);
    }
    if trimmed.chars().any(is_unsafe_ops_id_char) {
        return Err(SpectraQueryNameError::UnsafeTableName);
    }
    Ok(())
}

/// Gauge permission name for querying a Spectra table or metric.
#[must_use]
pub fn spectra_query_permission_name(table: &str) -> String {
    format!("spectra.query.{}", table.trim())
}

const HEX: &[u8; 16] = b"0123456789ABCDEF";

/// Percent-encode a single path segment for `/spectra/...` hrefs.
///
/// Leaves RFC 3986 unreserved characters alone (`ALPHA` / `DIGIT` / `-` `.` `_`
/// `~`). Encodes `/`, `\`, controls, spaces, and other bytes so raw `format!`
/// hrefs cannot smuggle extra path segments.
#[must_use]
pub fn encode_ops_path_segment(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for &b in raw.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                out.push(b as char);
            }
            _ => {
                out.push('%');
                out.push(HEX[(b >> 4) as usize] as char);
                out.push(HEX[(b & 0x0f) as usize] as char);
            }
        }
    }
    out
}

/// `/spectra/schema/{encoded}` detail href.
#[must_use]
pub fn spectra_schema_path(name: &str) -> String {
    format!("/spectra/schema/{}", encode_ops_path_segment(name))
}

/// `/spectra/schema/{encoded}/explore` event explore href.
#[must_use]
pub fn spectra_schema_explore_path(name: &str) -> String {
    format!("/spectra/schema/{}/explore", encode_ops_path_segment(name))
}

/// `/spectra/metric/{encoded}/explore` metric explore href.
#[must_use]
pub fn spectra_metric_explore_path(name: &str) -> String {
    format!("/spectra/metric/{}/explore", encode_ops_path_segment(name))
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
pub const fn empty_metrics_query_result() -> MetricsQueryResult {
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
pub const fn empty_event_aggregate_result() -> EventAggregateResult {
    EventAggregateResult::TimeSeries {
        series: Vec::new(),
        headline: Vec::new(),
    }
}

#[cfg(test)]
mod unit_tests;
