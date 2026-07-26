//! Spectra app server functions.
//!
//! Leptos `#[server]` entrypoints resolve Higgs request context, then delegate to
//! [`spectra_backend`] helpers (and [`permissions`]) so catalog/query contracts can
//! be verified without a full host.

mod permissions;

use leptos::prelude::*;
pub use permissions::require_spectra_query;
pub use spectra_backend::{
    empty_event_aggregate_result, empty_event_query_result, empty_metrics_query_result,
    schema_metadata_detail, schema_metadata_list, validate_spectra_query_name,
};
use spectra_core::{
    EventAggregateRequest, EventAggregateResult, EventQuery, EventQueryResult, MetricsQuery,
    MetricsQueryResult, SchemaDetailDto, SchemaListItem,
};

/// List summary metadata for every registered schema.
#[uf_product_macros::server]
pub async fn list_schema_metadata() -> Result<Vec<SchemaListItem>, ServerFnError> {
    let _ctx = higgs::Higgs::from_request().await?;
    Ok(schema_metadata_list())
}

/// Fetch full detail for a single schema by name, if it exists.
#[uf_product_macros::server]
pub async fn get_schema_metadata(
    /// Name of the schema to fetch detail for.
    name: String,
) -> Result<Option<SchemaDetailDto>, ServerFnError> {
    let _ctx = higgs::Higgs::from_request().await?;
    Ok(schema_metadata_detail(&name))
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
    Ok(empty_metrics_query_result())
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
    Ok(empty_event_query_result(&query.table))
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
    Ok(empty_event_aggregate_result())
}
