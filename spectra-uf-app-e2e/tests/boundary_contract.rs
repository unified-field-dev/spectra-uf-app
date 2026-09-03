//! Valence + Spectra ops boundary contracts for the lab host.
//!
//! These are not Playwright; they assert durable catalog/query postconditions on
//! the in-process mem Spectra stack after [`init_e2e_valence`].

#![allow(clippy::unwrap_used, clippy::expect_used, missing_docs)]

use chrono::Utc;
use serde_json::json;
use spectra::spectra_core::{EventQuery, GridFilterModel, GridPaginationModel, MetricsQuery};
use spectra_backend::{
    dashboard_catalog_summary, execute_event_query, execute_metrics_query, schema_metadata_list,
    validate_spectra_query_name, SpectraOpsError, SpectraQueryNameError,
};
use spectra_uf_app_e2e::{e2e_fixtures, e2e_spectra, init_e2e_valence};

#[tokio::test]
async fn ops_validate_query_name_sad() {
    assert_eq!(
        validate_spectra_query_name("").unwrap_err(),
        SpectraQueryNameError::EmptyTableName
    );
    assert_eq!(
        validate_spectra_query_name("../escape").unwrap_err(),
        SpectraQueryNameError::UnsafeTableName
    );
}

#[tokio::test]
async fn ops_catalog_lists_registered_schemas_happy() {
    init_e2e_valence().await;
    let fixtures = e2e_fixtures();
    let catalog = dashboard_catalog_summary(10);
    assert!(catalog.schema_count >= 1, "registry must include schemas");
    let names: Vec<_> = schema_metadata_list()
        .into_iter()
        .map(|s| s.table_or_metric)
        .collect();
    assert!(
        names.iter().any(|n| n == fixtures.event_table.as_str()),
        "seeded event table missing: {names:?}"
    );
}

#[tokio::test]
async fn ops_seeded_event_and_metric_queries_happy() {
    init_e2e_valence().await;
    let router = e2e_spectra().router();
    let now = Utc::now();
    let fixtures = e2e_fixtures();

    spectra::try_log_event_at(
        fixtures.event_table.as_str(),
        &json!({"id": "boundary-event", "message": "boundary seed", "severity": "info"}),
        now - chrono::Duration::minutes(1),
    );
    spectra::try_record_gauge_at(
        fixtures.metric_name.as_str(),
        &[("host", "boundary")],
        7.0,
        now - chrono::Duration::minutes(1),
    );
    e2e_spectra()
        .flush_persist()
        .await
        .expect("flush seeded rows");

    let events = execute_event_query(
        &router,
        &EventQuery {
            table: fixtures.event_table.clone(),
            start: now - chrono::Duration::hours(1),
            end: now,
            partition: None,
            pagination: GridPaginationModel::default(),
            sort: vec![],
            filter: GridFilterModel::default(),
        },
    )
    .await
    .expect("event query");
    assert!(
        events.row_count >= 1,
        "seeded event row must appear: {events:?}"
    );

    let metrics = execute_metrics_query(
        &router,
        &MetricsQuery {
            metric: fixtures.metric_name.clone(),
            start: now - chrono::Duration::hours(1),
            end: now,
            step_secs: Some(60),
            label_matchers: vec![],
        },
    )
    .await
    .expect("metric query");
    assert!(
        !metrics.series.is_empty() || !metrics.headline.is_empty(),
        "seeded metric must produce data: {metrics:?}"
    );
}

#[test]
fn ops_permission_denied_error_classifies_happy() {
    let err = SpectraOpsError::PermissionDenied {
        permission: "spectra.query.platform_smoke_event".into(),
    };
    let msg = err.to_string();
    assert!(spectra_backend::is_permission_denied_message(&msg));
    assert!(msg.contains("platform_smoke_event"));
}
