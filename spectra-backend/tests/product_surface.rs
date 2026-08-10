//! Product surface contracts for spectra-app (sibling crate).
//!
//! Lives under `spectra-backend` so CI can gate route/testid/auth/QueryTable needles
//! without compiling Orbital/turf UI when host pins churn. Pattern matches
//! photon-uf-app / boson-uf-app / chronon-uf-app `*-backend/tests/product_surface.rs`,
//! gauge `gauge/tests/product_surface.rs`, and lepton-uf-app
//! `lepton-shell/tests/product_surface.rs`.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn read_app(rel: &str) -> String {
    let path = workspace_root().join("spectra-app").join("src").join(rel);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn spectra_routes_mount_happy_path() {
    let lib = read_app("lib.rs");
    for needle in [
        r#"path!("spectra")"#,
        r#"path!("")"#,
        r#"path!("schema")"#,
        r#"path!("schema/:name")"#,
        r#"path!("schema/:name/explore")"#,
        r#"path!("metric/:name/explore")"#,
        "SpectraLayoutRouteView",
        "id: \"spectra\"",
        "route_path: \"/spectra\"",
        "permission_manifest: permissions::SpectraPermission",
    ] {
        assert!(
            lib.contains(needle),
            "SpectraRoutes / uf_app missing `{needle}`"
        );
    }
}

#[test]
fn spectra_routes_drop_leaf_sad_path() {
    let lib = read_app("lib.rs");
    for needle in [
        r#"path!("schema")"#,
        r#"path!("schema/:name")"#,
        r#"path!("schema/:name/explore")"#,
        r#"path!("metric/:name/explore")"#,
    ] {
        assert!(
            lib.contains(needle),
            "removing `{needle}` drops a Spectra ops funnel entry"
        );
    }
    assert!(
        !lib.contains("unimplemented!"),
        "SpectraRoutes must not ship unimplemented placeholders"
    );
}

#[test]
fn uf_app_wrong_id_sad_path() {
    let lib = read_app("lib.rs");
    assert!(
        lib.contains("id: \"spectra\""),
        "wrong uf_app id breaks Orbital host registration"
    );
    assert!(
        !lib.contains("id: \"spectra-app\""),
        "uf_app id must stay `spectra` (product route id), not crate name spectra-app"
    );
}

#[test]
fn layout_auth_gate_and_nav_happy_path() {
    let layout = read_app("layout/spectra_layout.rs");
    for needle in [
        "spectra-app-root",
        "RequireAuthenticated",
        "Outlet",
        "nav-spectra-schemas",
        "AppBarUserMenu",
        "UnifiedFieldShellLayout",
    ] {
        assert!(
            layout.contains(needle),
            "SpectraLayout missing contract `{needle}`"
        );
    }
}

#[test]
fn layout_drop_auth_guard_sad_path() {
    let layout = read_app("layout/spectra_layout.rs");
    assert!(
        layout.contains("RequireAuthenticated") && layout.contains("<Outlet />"),
        "removing RequireAuthenticated opens /spectra pages to anonymous sessions"
    );
}

#[test]
fn layout_missing_nav_sad_path() {
    let layout = read_app("layout/spectra_layout.rs");
    assert!(
        layout.contains("nav-spectra-schemas"),
        "dropping nav-spectra-schemas breaks operator left-nav contract"
    );
}

#[test]
fn ops_reads_require_query_table_happy_path() {
    let server = read_app("server/mod.rs");
    for fn_name in [
        "list_schema_metadata",
        "get_schema_metadata",
        "query_metrics",
        "query_events",
        "query_event_aggregate",
    ] {
        assert!(server.contains(fn_name), "server missing `{fn_name}`");
    }
    let query_attr = r#"permission = "QueryTable""#;
    assert!(
        server.matches(query_attr).count() >= 5,
        "catalog + explore server fns must carry QueryTable permission attribute"
    );
    assert!(
        server.contains("SPECTRA_QUERY_PERMISSION: &str = \"QueryTable\""),
        "SPECTRA_QUERY_PERMISSION constant must stay QueryTable"
    );
}

#[test]
fn ops_reads_drop_query_table_sad_path() {
    let server = read_app("server/mod.rs");
    let query_attr = r#"permission = "QueryTable""#;
    assert!(
        server.matches(query_attr).count() >= 5,
        "dropping QueryTable from any server fn opens Spectra ops without the query gate"
    );
    assert!(
        !server.contains(r#"permission = "GaugeAdmin""#)
            && !server.contains(r#"permission = "PhotonAdmin""#)
            && !server.contains(r#"permission = "BosonAdmin""#)
            && !server.contains(r#"permission = "ChrononAdmin""#),
        "Spectra ops must not gate on GaugeAdmin / PhotonAdmin / BosonAdmin / ChrononAdmin"
    );
}

#[test]
fn server_require_session_happy_path() {
    let server = read_app("server/mod.rs");
    assert!(
        server.contains("fn require_session")
            && server.contains("Authentication is required")
            && server.contains("session_user_id()"),
        "server must fail closed without a session"
    );

    let perms = read_app("server/permissions.rs");
    assert!(
        perms.contains("fn require_spectra_query")
            && perms.contains("Permission denied:")
            && perms.contains("spectra_query_permission_name"),
        "require_spectra_query must deny when Gauge spectra.query.{{table}} is missing"
    );

    for call_site in [
        "list_schema_metadata",
        "get_schema_metadata",
        "query_events",
        "query_metrics",
        "query_event_aggregate",
    ] {
        assert!(server.contains(call_site), "server missing `{call_site}`");
    }
}

#[test]
fn server_drop_require_session_on_list_and_query_sad_path() {
    let server = read_app("server/mod.rs");

    let start = server
        .find("pub async fn list_schema_metadata")
        .expect("list_schema_metadata");
    let body = &server[start..start + 350.min(server.len() - start)];
    assert!(
        body.contains("require_session(&ctx)?"),
        "list_schema_metadata must call require_session before Spectra IO"
    );

    let start = server
        .find("pub async fn query_events")
        .expect("query_events");
    let body = &server[start..start + 450.min(server.len() - start)];
    assert!(
        body.contains("require_session(&ctx)?"),
        "query_events must call require_session before Spectra IO"
    );
    assert!(
        body.contains("require_spectra_query"),
        "query_events must call require_spectra_query before the explore stub"
    );
}

#[test]
fn explore_require_spectra_query_happy_path() {
    let server = read_app("server/mod.rs");
    for (fn_name, arg_needle) in [
        ("query_metrics", "require_spectra_query(&query.metric)"),
        ("query_events", "require_spectra_query(&query.table)"),
        (
            "query_event_aggregate",
            "require_spectra_query(&request.table)",
        ),
    ] {
        let start = server
            .find(&format!("pub async fn {fn_name}"))
            .unwrap_or_else(|| panic!("{fn_name}"));
        let body = &server[start..start + 500.min(server.len() - start)];
        assert!(
            body.contains(arg_needle),
            "{fn_name} must gate explore with `{arg_needle}`"
        );
    }
}

#[test]
fn explore_drop_require_spectra_query_sad_path() {
    let server = read_app("server/mod.rs");
    let start = server
        .find("pub async fn query_metrics")
        .expect("query_metrics");
    let body = &server[start..start + 500.min(server.len() - start)];
    assert!(
        body.contains("require_spectra_query(&query.metric)"),
        "dropping require_spectra_query on query_metrics skips per-table Gauge gate"
    );
    let start = server
        .find("pub async fn query_event_aggregate")
        .expect("query_event_aggregate");
    let body = &server[start..start + 500.min(server.len() - start)];
    assert!(
        body.contains("require_spectra_query(&request.table)"),
        "dropping require_spectra_query on query_event_aggregate skips per-table Gauge gate"
    );
}

#[test]
fn index_pages_testid_and_list_bindings_happy_path() {
    let index = read_app("pages/schema_index/mod.rs");
    assert!(
        index.contains("schema-index-page"),
        "SchemaIndexPage missing schema-index-page testid"
    );
    let section = read_app("pages/schema_index/components/schema_index_section.rs");
    assert!(
        section.contains("list_schema_metadata"),
        "SchemaIndexSection must bind list_schema_metadata"
    );

    let event = read_app("pages/event_explore/mod.rs");
    assert!(
        event.contains("spectra-event-explore-panel"),
        "EventExplorePage missing spectra-event-explore-panel testid"
    );
    let event_panel = read_app("pages/event_explore/components/event_explore_panel.rs");
    for needle in ["query_events", "query_event_aggregate"] {
        assert!(
            event_panel.contains(needle),
            "EventExplorePanel missing `{needle}`"
        );
    }

    let metric = read_app("pages/metric_explore/mod.rs");
    assert!(
        metric.contains("spectra-metric-explore-panel"),
        "MetricExplorePage missing spectra-metric-explore-panel testid"
    );
    let metric_panel = read_app("pages/metric_explore/components/metric_explore_panel.rs");
    assert!(
        metric_panel.contains("query_metrics"),
        "MetricExplorePanel must bind query_metrics"
    );
}

#[test]
fn index_drop_schema_testid_sad_path() {
    let index = read_app("pages/schema_index/mod.rs");
    assert!(
        index.contains("data_testid=\"schema-index-page\""),
        "dropping schema-index-page breaks host / future Playwright parity"
    );
    let event = read_app("pages/event_explore/mod.rs");
    assert!(
        event.contains("data_testid=\"spectra-event-explore-panel\""),
        "dropping spectra-event-explore-panel breaks host / future Playwright parity"
    );
    let metric = read_app("pages/metric_explore/mod.rs");
    assert!(
        metric.contains("data_testid=\"spectra-metric-explore-panel\""),
        "dropping spectra-metric-explore-panel breaks host / future Playwright parity"
    );
}

#[test]
fn detail_pages_bindings_happy_path() {
    let detail = read_app("pages/schema_detail/components/schema_detail_body.rs");
    assert!(
        detail.contains("get_schema_metadata"),
        "SchemaDetailBody must bind get_schema_metadata"
    );
    let home = read_app("pages/home.rs");
    assert!(
        home.contains("/spectra/schema"),
        "SpectraHomePage must redirect into the schema index funnel"
    );
}

#[test]
fn detail_pages_missing_bindings_sad_path() {
    let detail = read_app("pages/schema_detail/components/schema_detail_body.rs");
    assert!(
        detail.contains("get_schema_metadata"),
        "schema detail must bind get_schema_metadata"
    );
    assert!(
        !detail.contains("unimplemented!"),
        "schema detail must not ship unimplemented placeholders"
    );
}

#[test]
fn permission_manifest_query_table_happy_path() {
    let perms = read_app("permissions.rs");
    for needle in [
        "domain_key = \"spectra\"",
        "QueryTable",
        "UfPermissionManifest",
    ] {
        assert!(
            perms.contains(needle),
            "SpectraPermission manifest missing `{needle}`"
        );
    }
}

#[test]
fn protected_spectra_host_matches_uf_app_happy_path() {
    let host =
        fs::read_to_string(workspace_root().join("examples/protected-spectra-host/src/main.rs"))
            .expect("protected-spectra-host main.rs");
    for needle in [
        "\"app_id\": \"spectra\"",
        "\"route_path\": \"/spectra\"",
        "\"auth_gate\": \"RequireAuthenticated\"",
        "\"admin_permission\": \"QueryTable\"",
        "schema_metadata_list",
    ] {
        assert!(
            host.contains(needle),
            "protected-spectra-host missing contract `{needle}`"
        );
    }
    let lib = read_app("lib.rs");
    assert!(
        lib.contains("id: \"spectra\"") && lib.contains("route_path: \"/spectra\""),
        "host inventory must stay aligned with uf_app!"
    );
    let layout = read_app("layout/spectra_layout.rs");
    assert!(
        layout.contains("RequireAuthenticated"),
        "host auth_gate must stay aligned with SpectraLayout guard"
    );
    let perms = read_app("permissions.rs");
    assert!(
        perms.contains("QueryTable"),
        "host admin_permission must stay aligned with SpectraPermission"
    );
}

#[test]
fn lazy_routes_wire_pages_happy_path() {
    let lazy = read_app("lazy_routes.rs");
    for needle in [
        "SpectraHomePage",
        "SchemaIndexPage",
        "SchemaDetailPage",
        "EventExplorePage",
        "MetricExplorePage",
        "SpectraLayout",
    ] {
        assert!(
            lazy.contains(needle),
            "lazy_routes missing page wire `{needle}`"
        );
    }
}
