//! Spectra query permission gate.

pub use spectra_backend::validate_spectra_query_name;

/// Table/metric query permission check.
///
/// Full Gauge `spectra.query.{table}` enforcement needs host-aligned gauge+valence
/// (same Valence crate graph). Until host wiring lands, this validates a non-empty
/// table/metric name and that Higgs request context can be constructed under `ssr`.
pub async fn require_spectra_query(table: &str) -> Result<(), String> {
    #[cfg(feature = "ssr")]
    {
        validate_spectra_query_name(table)?;

        let _ctx = higgs::Higgs::from_request()
            .await
            .map_err(|e| format!("Failed to resolve request context: {e}"))?;
        Ok(())
    }

    #[cfg(not(feature = "ssr"))]
    {
        let _ = table;
        Ok(())
    }
}
