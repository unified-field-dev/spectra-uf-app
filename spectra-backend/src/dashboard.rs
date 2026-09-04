//! Dashboard catalog summary helpers for the Spectra home page.

use serde::{Deserialize, Serialize};
use spectra_core::SchemaListItem;

use crate::schema_metadata_list;

/// Default number of recent schemas shown on the home dashboard.
pub const DEFAULT_RECENT_SCHEMA_LIMIT: usize = 5;

/// Catalog counts and recent schema rows for the home dashboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardCatalogSummary {
    /// Total registered schemas.
    pub schema_count: u64,
    /// Event table schemas.
    pub event_table_count: u64,
    /// Metric schemas.
    pub metric_count: u64,
    /// Recent schemas sorted by name.
    pub recent_schemas: Vec<SchemaListItem>,
}

/// Builds catalog summary from the schema registry (no router IO).
#[must_use]
pub fn dashboard_catalog_summary(recent_limit: usize) -> DashboardCatalogSummary {
    let mut schemas = schema_metadata_list();
    let schema_count = u64::try_from(schemas.len()).unwrap_or(u64::MAX);
    let mut event_table_count = 0u64;
    let mut metric_count = 0u64;
    for item in &schemas {
        match item.logging_kind.as_str() {
            "event" => event_table_count = event_table_count.saturating_add(1),
            "metric" => metric_count = metric_count.saturating_add(1),
            _ => {}
        }
    }
    schemas.sort_by(|a, b| a.table_or_metric.cmp(&b.table_or_metric));
    schemas.truncate(recent_limit);
    DashboardCatalogSummary {
        schema_count,
        event_table_count,
        metric_count,
        recent_schemas: schemas,
    }
}
