//! Spectra query permission gate.

pub use spectra_backend::{spectra_query_permission_name, validate_spectra_query_name};

/// Table/metric query permission check via Gauge `spectra.query.{table}`.
///
/// Validates the table/metric name (blank / oversized / path-unsafe rejected),
/// resolves Higgs request context, and calls [`gauge::service::actor_can`] for
/// the per-table permission.
///
/// Invalid names surface [`spectra_backend::SpectraQueryNameError`] Display text;
/// other failures stay as operator-facing strings at this gate.
pub async fn require_spectra_query(table: &str) -> Result<(), String> {
    #[cfg(feature = "ssr")]
    {
        validate_spectra_query_name(table).map_err(|e| e.to_string())?;

        let ctx = higgs::Higgs::from_request()
            .await
            .map_err(|e| format!("Failed to resolve request context: {e}"))?;

        let valence = ctx
            .valence()
            .map_err(|e| format!("Failed to resolve valence: {e}"))?;

        let permission = spectra_query_permission_name(table);
        let allowed = gauge::service::actor_can(&valence, &permission)
            .await
            .map_err(|e| format!("Permission check failed for `{permission}`: {e}"))?;

        if !allowed {
            return Err(format!(
                "Permission denied: `{permission}` is required to query this table"
            ));
        }
        Ok(())
    }

    #[cfg(not(feature = "ssr"))]
    {
        let _ = table;
        Ok(())
    }
}
