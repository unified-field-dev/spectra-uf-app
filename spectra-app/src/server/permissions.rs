//! Spectra query permission gate.

pub use spectra_backend::{spectra_query_permission_name, validate_spectra_query_name, SpectraOpsError};

/// Table/metric query permission check via Gauge `spectra.query.{table}`.
///
/// Validates the table/metric name (blank / oversized / path-unsafe rejected),
/// resolves Higgs request context, and calls [`gauge::service::actor_can`] for
/// the per-table permission.
///
/// Invalid names surface [`spectra_backend::SpectraQueryNameError`] via
/// [`SpectraOpsError::Validation`]; Gauge denials use [`SpectraOpsError::PermissionDenied`].
///
/// ## Non-SSR builds
///
/// When the `ssr` feature is disabled (WASM client compile), this function is a
/// no-op that returns `Ok(())`. Permission enforcement runs only on the server
/// inside `#[server]` functions — never rely on this stub for authz in client code.
pub async fn require_spectra_query(table: &str) -> Result<(), SpectraOpsError> {
    #[cfg(feature = "ssr")]
    {
        validate_spectra_query_name(table)?;

        let ctx = higgs::Higgs::from_request()
            .await
            .map_err(|e| SpectraOpsError::ContextResolution(format!("Failed to resolve request context: {e}")))?;

        let valence = ctx
            .valence()
            .map_err(|e| SpectraOpsError::ContextResolution(format!("Failed to resolve valence: {e}")))?;

        let permission = spectra_query_permission_name(table);
        let allowed = gauge::service::actor_can(&valence, &permission)
            .await
            .map_err(|e| {
                SpectraOpsError::ContextResolution(format!(
                    "Permission check failed for `{permission}`: {e}"
                ))
            })?;

        if !allowed {
            return Err(SpectraOpsError::PermissionDenied { permission });
        }
        Ok(())
    }

    #[cfg(not(feature = "ssr"))]
    {
        let _ = table;
        Ok(())
    }
}
