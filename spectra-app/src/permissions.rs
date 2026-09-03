//! Permission manifest for the Spectra operations app.

use uf_product_macros::UfPermissionManifest;

/// Query permission for Spectra catalog and explore server functions.
///
/// Synced into the `spectra` domain; gated with
/// `#[uf_product_macros::server(permission = "QueryTable")]`.
/// Per-table explore queries additionally check Gauge
/// `spectra.query.{table}` via [`crate::server::require_spectra_query`].
#[allow(clippy::expl_impl_clone_on_copy)]
#[derive(UfPermissionManifest)]
#[permission_manifest(
    domain_key = "spectra",
    domain_name = "Spectra",
    domain_description = "Spectra log and metric exploration"
)]
pub enum SpectraPermission {
    /// List schemas and run explore queries against registered tables/metrics.
    #[permission(description = "Query Spectra schemas, events, and metrics")]
    QueryTable,
}
