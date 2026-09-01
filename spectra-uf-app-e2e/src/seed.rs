//! Harness-only seed endpoint for Playwright.

use axum::http::StatusCode;
use axum::Json;
use chrono::Utc;
use gauge::service;
use gauge::types::PermissionCreateInput;
use serde::Deserialize;
use serde_json::json;
use spectra::{try_log_event_at, try_record_gauge_at};
use spectra_backend::spectra_query_permission_name;

use crate::e2e_spectra::e2e_spectra;
use crate::e2e_valence::{e2e_admin_valence, e2e_fixtures, E2E_EVENT_TABLE, E2E_METRIC_NAME};
use crate::gate_demos::{write_e2e_auth_kind, E2eAuthKind};

#[derive(Debug, Deserialize)]
pub struct SeedRequest {
    /// `anonymous` | `admin` | `outsider` | `unverified`
    #[serde(default = "default_auth")]
    pub auth: String,
}

fn default_auth() -> String {
    E2eAuthKind::Anonymous.as_str().to_string()
}

async fn ensure_and_grant_table_query_perm(
    admin: &valence::Valence,
    user_id: &str,
    table: &str,
) -> Result<(), StatusCode> {
    let perm_name = spectra_query_permission_name(table);
    let perms = service::list_permissions(admin, None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let perm_id = if let Some(existing) = perms.iter().find(|p| p.name == perm_name) {
        existing.id.clone()
    } else {
        let _created = service::create_permission(
            PermissionCreateInput {
                name: perm_name.clone(),
                description: format!("E2e query permission for {table}"),
                owners_group_id: String::new(),
                domain_id: "spectra".into(),
            },
            admin,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        service::list_permissions(admin, None)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .into_iter()
            .find(|p| p.name == perm_name)
            .map(|p| p.id)
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
    };
    service::grant_permission_to_user(&perm_id, user_id, admin)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}

async fn seed_spectra_rows(kind: E2eAuthKind) -> Result<(), StatusCode> {
    if !matches!(kind, E2eAuthKind::Admin) {
        return Ok(());
    }

    let admin = e2e_admin_valence();
    ensure_and_grant_table_query_perm(&admin, "admin", E2E_EVENT_TABLE).await?;
    ensure_and_grant_table_query_perm(&admin, "admin", E2E_METRIC_NAME).await?;

    let _ = e2e_spectra();
    let now = Utc::now();
    let event_ts = now - chrono::Duration::minutes(5);
    try_log_event_at(
        E2E_EVENT_TABLE,
        &json!({
            "id": "e2e-event-1",
            "message": "Playwright seed row",
            "severity": "info",
        }),
        event_ts,
    );
    try_record_gauge_at(E2E_METRIC_NAME, &[("host", "e2e")], 42.0, event_ts);

    e2e_spectra().flush_persist().await.map_err(|err| {
        log::error!("e2e seed: spectra flush_persist failed: {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(())
}

pub async fn seed_data(
    session: tower_sessions::Session,
    Json(body): Json<SeedRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let kind = E2eAuthKind::parse(&body.auth);
    write_e2e_auth_kind(&session, kind)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    seed_spectra_rows(kind).await?;

    let fixtures = e2e_fixtures();
    Ok(Json(serde_json::json!({
        "ok": true,
        "auth": kind.as_str(),
        "fixtures": {
            "event_table": fixtures.event_table,
            "metric_name": fixtures.metric_name,
        }
    })))
}
