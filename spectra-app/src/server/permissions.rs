//! Spectra query permission gate.

/// Rejects blank table/metric names before host permission checks run.
///
/// Only called from the `ssr`-gated branch of `require_spectra_query`.
#[cfg_attr(not(feature = "ssr"), allow(dead_code))]
pub fn validate_spectra_query_name(table: &str) -> Result<(), String> {
    if table.trim().is_empty() {
        Err("Spectra query table name is required".to_string())
    } else {
        Ok(())
    }
}

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

#[cfg(test)]
mod tests {
    use super::validate_spectra_query_name;

    #[test]
    fn validate_spectra_query_name_rejects_blank() {
        assert!(validate_spectra_query_name("").is_err());
        assert!(validate_spectra_query_name("   ").is_err());
    }

    #[test]
    fn validate_spectra_query_name_accepts_non_empty() {
        assert!(validate_spectra_query_name("events").is_ok());
        assert!(validate_spectra_query_name("  metrics.latency  ").is_ok());
    }
}
