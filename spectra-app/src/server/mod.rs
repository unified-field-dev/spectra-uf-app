//! Spectra app server functions.

mod permissions;

use leptos::prelude::*;
pub use permissions::require_spectra_query;
use spectra_core::{
    list_schemas, rows_to_event_result, schema_detail, EventAggregateRequest, EventAggregateResult,
    EventQuery, EventQueryResult, MetricsQuery, MetricsQueryResult, SchemaDetailDto, SchemaListItem,
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
    // Host-injected Spectra router wiring is deferred to Wave 7b; compile-only stub.
    Ok(MetricsQueryResult {
        series: Vec::new(),
        headline: Vec::new(),
    })
}

#[uf_product_macros::server]
pub async fn query_events(query: EventQuery) -> Result<EventQueryResult, ServerFnError> {
    let _ctx = higgs::Higgs::from_request().await?;
    require_spectra_query(&query.table)
        .await
        .map_err(ServerFnError::new)?;
    Ok(rows_to_event_result(&query.table, Vec::new(), 0))
}

#[uf_product_macros::server]
pub async fn query_event_aggregate(
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
