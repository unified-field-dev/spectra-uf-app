//! Integration contracts for stubbed explore query result shapes
//! (`query_events` / `query_event_aggregate` / `query_metrics`).

#![allow(missing_docs)]

use spectra_backend::{
    empty_event_aggregate_result, empty_event_query_result, empty_metrics_query_result,
    validate_spectra_query_name,
};
use spectra_core::EventAggregateResult;

#[test]
fn empty_metrics_query_result_shape_happy_path() {
    let r = empty_metrics_query_result();
    assert!(r.series.is_empty());
    assert!(r.headline.is_empty());
}

#[test]
fn empty_event_query_result_unknown_table_happy_path() {
    let r = empty_event_query_result("__spectra_uf_app_no_such_table__");
    assert!(r.rows.is_empty());
    assert_eq!(r.row_count, 0);
    assert_eq!(r.columns.len(), 1);
    assert_eq!(r.columns[0].field, "ts");
    assert_eq!(r.columns[0].header_name, "Timestamp");
}

#[test]
fn empty_event_aggregate_result_timeseries_stub_happy_path() {
    match empty_event_aggregate_result() {
        EventAggregateResult::TimeSeries { series, headline } => {
            assert!(series.is_empty());
            assert!(headline.is_empty());
        }
        other => panic!("expected empty TimeSeries stub, got {other:?}"),
    }
}

#[test]
fn validate_spectra_query_name_rejects_blank_sad() {
    let err = validate_spectra_query_name("").expect_err("blank name");
    assert!(
        err.contains("required"),
        "expected required-name message, got {err}"
    );
}

#[test]
fn validate_spectra_query_name_accepts_table_happy_path() {
    validate_spectra_query_name("ops.events").expect("non-empty table");
}
