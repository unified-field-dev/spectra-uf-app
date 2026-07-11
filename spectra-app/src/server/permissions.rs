//! Spectra query permission gate via Gauge.

#[cfg(feature = "ssr")]
use gauge::instrumentation::PermissionCheckCallerGuard;

/// Table/metric query permission check — requires `spectra.query.{table}` on SSR.
pub async fn require_spectra_query(table: &str) -> Result<(), String> {
    #[cfg(feature = "ssr")]
    {
        let table = table.trim();
        if table.is_empty() {
            return Err("Spectra query table name is required".to_string());
        }

        let ctx = higgs::Higgs::from_request()
            .await
            .map_err(|e| format!("Failed to resolve request context: {e}"))?;
        let permission_name = format!("spectra.query.{table}");
        let _caller = PermissionCheckCallerGuard::new("spectra_query");
        let allowed = gauge::service::has_permission(ctx.valence(), &permission_name)
            .await
            .map_err(|e| format!("Failed to check permission: {e}"))?;
        if allowed {
            Ok(())
        } else {
            Err(format!(
                "Permission denied: `{permission_name}` is required to query this table"
            ))
        }
    }

    #[cfg(not(feature = "ssr"))]
    {
        let _ = table;
        Ok(())
    }
}
