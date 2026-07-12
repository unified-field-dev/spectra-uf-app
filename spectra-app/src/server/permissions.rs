//! Spectra query permission gate.

/// Table/metric query permission check.
///
/// Full Gauge `spectra.query.{table}` enforcement needs host-aligned gauge+valence
/// (same Valence crate graph). Deferred to Wave 7b host wiring — compile path only
/// validates a non-empty table name and that Higgs can be constructed.
pub async fn require_spectra_query(table: &str) -> Result<(), String> {
    #[cfg(feature = "ssr")]
    {
        let table = table.trim();
        if table.is_empty() {
            return Err("Spectra query table name is required".to_string());
        }

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
