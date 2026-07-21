//! Spectra app server functions.

mod permissions;

use leptos::prelude::*;
pub use permissions::require_spectra_query;
use spectra_core::{
    list_schemas, rows_to_event_result, schema_detail, EventAggregateRequest, EventAggregateResult,
    EventQuery, EventQueryResult, MetricsQuery, MetricsQueryResult, SchemaDetailDto,
    SchemaListItem,
};

/// List summary metadata for every registered schema.
#[uf_product_macros::server]
pub async fn list_schema_metadata() -> Result<Vec<SchemaListItem>, ServerFnError> {
    let _ctx = higgs::Higgs::from_request().await?;
    Ok(list_schemas())
}

/// Fetch full detail for a single schema by name, if it exists.
#[uf_product_macros::server]
pub async fn get_schema_metadata(
    /// Name of the schema to fetch detail for.
    name: String,
) -> Result<Option<SchemaDetailDto>, ServerFnError> {
    let _ctx = higgs::Higgs::from_request().await?;
    Ok(schema_detail(&name))
}

/// Run a metric query and return the resulting time series and headline values.
#[uf_product_macros::server]
pub async fn query_metrics(
    /// Metric query describing the metric, time range, and aggregation to run.
    query: MetricsQuery,
) -> Result<MetricsQueryResult, ServerFnError> {
    let _ctx = higgs::Higgs::from_request().await?;
    require_spectra_query(&query.metric)
        .await
        .map_err(ServerFnError::new)?;
    // Host-injected Spectra router wiring lands with deployment composition; return an
    // empty result shape until the host registers a live query backend.
    Ok(MetricsQueryResult {
        series: Vec::new(),
        headline: Vec::new(),
    })
}

/// Run an event query against a table and return matching rows.
#[uf_product_macros::server]
pub async fn query_events(
    /// Event query describing the table, filters, and paging to run.
    query: EventQuery,
) -> Result<EventQueryResult, ServerFnError> {
    let _ctx = higgs::Higgs::from_request().await?;
    require_spectra_query(&query.table)
        .await
        .map_err(ServerFnError::new)?;
    Ok(rows_to_event_result(&query.table, Vec::new(), 0))
}

/// Run an aggregate query (time series or headline) over events in a table.
#[uf_product_macros::server]
pub async fn query_event_aggregate(
    /// Aggregate query describing the table, grouping, and time range to run.
    request: EventAggregateRequest,
) -> Result<EventAggregateResult, ServerFnError> {
    let _ctx = higgs::Higgs::from_request().await?;
    require_spectra_query(&request.table)
        .await
        .map_err(ServerFnError::new)?;
    Ok(EventAggregateResult::TimeSeries {
        series: Vec::new(),
        headline: Vec::new(),
    })
}
