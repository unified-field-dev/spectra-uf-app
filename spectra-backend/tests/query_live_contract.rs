//! Live query helper contracts against mem Spectra backends.

#![allow(missing_docs, clippy::expect_used)]

use std::sync::Arc;

use chrono::Utc;
use serde_json::json;
use spectra::{MemEventsBackend, MemMetricsBackend, Spectra};
use spectra_backend::{
    dashboard_catalog_summary, execute_event_query, execute_metrics_query,
    DEFAULT_RECENT_SCHEMA_LIMIT,
};
use spectra_core::{
    EventQuery, EventStorageBackend, GridFilterModel, GridPaginationModel, MetricsQuery,
    MetricsStorageBackend, SpectraRouter,
};

const SEED_TABLE: &str = "integ.events";
const SEED_METRIC: &str = "integ.cpu";

fn empty_router() -> Arc<SpectraRouter> {
    let metrics: Arc<dyn MetricsStorageBackend> = Arc::new(MemMetricsBackend::new());
    let events: Arc<dyn EventStorageBackend> = Arc::new(MemEventsBackend::new());
    Spectra::builder()
        .metrics_backend(metrics)
        .events_backend(events)
        .embedded()
        .build()
        .expect("mem spectra")
        .router()
}

async fn seeded_router() -> Arc<SpectraRouter> {
    let metrics: Arc<MemMetricsBackend> = Arc::new(MemMetricsBackend::new());
    let events: Arc<MemEventsBackend> = Arc::new(MemEventsBackend::new());
    let ts = Utc::now() - chrono::Duration::minutes(5);
    events
        .append_row(
            SEED_TABLE,
            &json!({
                "id": "integ-1",
                "message": "integration seed row",
                "severity": "info",
                "value": 10,
            }),
            ts,
            None,
        )
        .await
        .expect("append row 1");
    events
        .append_row(
            SEED_TABLE,
            &json!({
                "id": "integ-2",
                "message": "warn row",
                "severity": "warn",
                "value": 3,
            }),
            ts - chrono::Duration::minutes(1),
            None,
        )
        .await
        .expect("append row 2");
    metrics
        .record_gauge(SEED_METRIC, &json!({"host": "integ"}), 42.0, ts)
        .await
        .expect("record metric");
    Spectra::builder()
        .metrics_backend(Arc::clone(&metrics) as Arc<dyn MetricsStorageBackend>)
        .events_backend(Arc::clone(&events) as Arc<dyn EventStorageBackend>)
        .embedded()
        .build()
        .expect("mem spectra")
        .router()
}

#[tokio::test]
async fn execute_event_query_empty_table_happy_path() {
    let router = empty_router();
    let result = execute_event_query(
        &router,
        &EventQuery {
            table: "e2e.events".into(),
            start: Utc::now() - chrono::Duration::hours(1),
            end: Utc::now(),
            partition: None,
            pagination: GridPaginationModel::default(),
            sort: vec![],
            filter: GridFilterModel::default(),
        },
    )
    .await
    .expect("query");
    assert_eq!(result.row_count, 0);
    assert!(result.rows.is_empty());
}

#[tokio::test]
async fn execute_event_query_seeded_row_happy_path() {
    let router = seeded_router().await;
    let result = execute_event_query(
        &router,
        &EventQuery {
            table: SEED_TABLE.into(),
            start: Utc::now() - chrono::Duration::hours(1),
            end: Utc::now(),
            partition: None,
            pagination: GridPaginationModel::default(),
            sort: vec![],
            filter: GridFilterModel::default(),
        },
    )
    .await
    .expect("query");
    assert!(result.row_count >= 2, "expected seeded rows");
    let body = format!("{result:?}");
    assert!(
        body.contains("integration seed row"),
        "expected seed message in rows: {body}"
    );
}

#[tokio::test]
async fn execute_metrics_query_empty_happy_path() {
    let router = empty_router();
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

#[tokio::test]
async fn execute_metrics_query_seeded_point_happy_path() {
    let router = seeded_router().await;
    let result = execute_metrics_query(
        &router,
        &MetricsQuery {
            metric: SEED_METRIC.into(),
            start: Utc::now() - chrono::Duration::hours(1),
            end: Utc::now(),
            step_secs: Some(60),
            label_matchers: vec![],
        },
    )
    .await
    .expect("query");
    let snapshot = format!("{result:?}");
    assert!(
        snapshot.contains("42") || !result.series.is_empty(),
        "expected metric series or headline with seeded value: {snapshot}"
    );
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
