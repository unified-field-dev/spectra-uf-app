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
        EventAggregateResult::Slices { .. } => {
            panic!("expected TimeSeries stub, got Slices")
        }
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
