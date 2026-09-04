//! Live Spectra query execution against an installed [`SpectraRouter`].
//!
//! Maps UI DTOs to storage filters, runs router queries, and maps results back.
//! Used by `spectra-app` server functions when a host registers a global router.

use spectra_core::{
    aggregate_request_to_filter, event_query_to_filter, metrics_query_to_range,
    points_to_metrics_result, rows_to_event_result, EventAggregateRequest, EventAggregateResult,
    EventQuery, EventQueryResult, MetricsQuery, MetricsQueryResult, SpectraRouter,
};

/// Runs a metric explore query through the router.
///
/// # Errors
///
/// Returns [`spectra_core::Error`] when identifier validation or backend query fails.
pub async fn execute_metrics_query(
    router: &SpectraRouter,
    query: &MetricsQuery,
) -> Result<MetricsQueryResult, spectra_core::Error> {
    let range = metrics_query_to_range(query);
    let points = router.query_metrics(range).await?;
    Ok(points_to_metrics_result(points))
}

/// Runs an event log query through the router.
///
/// Row count uses the returned row length until backends expose a separate total.
///
/// # Errors
///
/// Returns [`spectra_core::Error`] when identifier validation or backend query fails.
pub async fn execute_event_query(
    router: &SpectraRouter,
    query: &EventQuery,
) -> Result<EventQueryResult, spectra_core::Error> {
    let filter = event_query_to_filter(query);
    let table = query.table.clone();
    let rows = router.query_events(filter).await?;
    let row_count = u64::try_from(rows.len()).unwrap_or(u64::MAX);
    Ok(rows_to_event_result(&table, rows, row_count))
}

/// Runs an event aggregate (chart) query through the router.
///
/// # Errors
///
/// Returns [`spectra_core::Error`] when identifier validation or backend query fails.
pub async fn execute_event_aggregate(
    router: &SpectraRouter,
    request: &EventAggregateRequest,
) -> Result<EventAggregateResult, spectra_core::Error> {
    let filter = aggregate_request_to_filter(request);
    router.query_event_aggregate(filter).await
}
