//! Spectra app server functions.

mod permissions;

use leptos::prelude::*;
pub use permissions::require_spectra_query;
use spectra_core::{
    aggregate_request_to_filter, event_query_to_filter, list_schemas, metrics_query_to_range,
    points_to_metrics_result, rows_to_event_result, schema_detail, EventAggregateRequest,
    EventAggregateResult, EventQuery, EventQueryResult, MetricsQuery, MetricsQueryResult,
    SchemaDetailDto, SchemaListItem,
};

#[uf_product_macros::server]
pub async fn list_schema_metadata() -> Result<Vec<SchemaListItem>, ServerFnError> {
    let _ctx = higgs::Higgs::from_request().await?;
    Ok(list_schemas())
}

#[uf_product_macros::server]
pub async fn get_schema_metadata(name: String) -> Result<Option<SchemaDetailDto>, ServerFnError> {
    let _ctx = higgs::Higgs::from_request().await?;
    Ok(schema_detail(&name))
}

#[uf_product_macros::server]
pub async fn query_metrics(query: MetricsQuery) -> Result<MetricsQueryResult, ServerFnError> {
    let _ctx = higgs::Higgs::from_request().await?;
    require_spectra_query(&query.metric)
        .await
        .map_err(ServerFnError::new)?;
    let range = metrics_query_to_range(&query);
    let points = spectra::storage_router()
        .query_metrics(range)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(points_to_metrics_result(points))
}

#[uf_product_macros::server]
pub async fn query_events(query: EventQuery) -> Result<EventQueryResult, ServerFnError> {
    let _ctx = higgs::Higgs::from_request().await?;
    require_spectra_query(&query.table)
        .await
        .map_err(ServerFnError::new)?;
    let filter = event_query_to_filter(&query);
    let table = query.table.clone();
    let rows = spectra::storage_router()
        .query_events(filter.clone())
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    let row_count = rows.len() as u64;
    Ok(rows_to_event_result(&table, rows, row_count))
}

#[uf_product_macros::server]
pub async fn query_event_aggregate(
    request: EventAggregateRequest,
) -> Result<EventAggregateResult, ServerFnError> {
    let _ctx = higgs::Higgs::from_request().await?;
    require_spectra_query(&request.table)
        .await
        .map_err(ServerFnError::new)?;
    let filter = aggregate_request_to_filter(&request);
    spectra::storage_router()
        .query_event_aggregate(filter)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
