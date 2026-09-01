//! Live query helper contracts against mem Spectra backends.

#![allow(missing_docs, clippy::expect_used)]

use std::sync::Arc;

use chrono::Utc;
use spectra::{MemEventsBackend, MemMetricsBackend, Spectra};
use spectra_backend::{
    dashboard_catalog_summary, execute_event_query, execute_metrics_query,
    DEFAULT_RECENT_SCHEMA_LIMIT,
};
use spectra_core::{EventQuery, GridPaginationModel, MetricsQuery, SpectraRouter};

fn mem_router() -> Arc<SpectraRouter> {
    let metrics: Arc<dyn spectra_core::MetricsStorageBackend> = Arc::new(MemMetricsBackend::new());
    let events: Arc<dyn spectra_core::EventStorageBackend> = Arc::new(MemEventsBackend::new());
    let spectra = Spectra::builder()
        .metrics_backend(metrics)
        .events_backend(events)
        .embedded()
        .build()
        .expect("mem spectra");
    spectra.router()
}

#[tokio::test]
async fn execute_event_query_empty_table_happy_path() {
    let router = mem_router();
    let result = execute_event_query(
        &router,
        &EventQuery {
            table: "e2e.events".into(),
            start: Utc::now() - chrono::Duration::hours(1),
            end: Utc::now(),
            partition: None,
            pagination: GridPaginationModel::default(),
            sort: vec![],
            filter: Default::default(),
        },
    )
    .await
    .expect("query");
    assert_eq!(result.row_count, 0);
    assert!(result.rows.is_empty());
}

#[tokio::test]
async fn execute_metrics_query_empty_happy_path() {
    let router = mem_router();
    let result = execute_metrics_query(
        &router,
        &MetricsQuery {
            metric: "e2e.cpu".into(),
            start: Utc::now() - chrono::Duration::hours(1),
            end: Utc::now(),
            step_secs: Some(60),
            label_matchers: vec![],
        },
    )
    .await
    .expect("query");
    assert!(result.headline.is_empty() || !result.series.is_empty());
}

#[test]
fn dashboard_catalog_summary_counts_happy_path() {
    let summary = dashboard_catalog_summary(DEFAULT_RECENT_SCHEMA_LIMIT);
    assert_eq!(
        summary.schema_count,
        summary.event_table_count
            + summary.metric_count
            + u64::try_from(
                summary
                    .recent_schemas
                    .iter()
                    .filter(|s| s.logging_kind != "event" && s.logging_kind != "metric")
                    .count()
            )
            .unwrap_or(0)
    );
    assert!(summary.recent_schemas.len() <= DEFAULT_RECENT_SCHEMA_LIMIT);
}
