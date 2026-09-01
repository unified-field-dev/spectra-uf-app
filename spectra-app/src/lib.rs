#![recursion_limit = "256"]
//! Spectra operations app — browse registered schemas and explore event logs and metrics.
//!
//! Leptos UI mounted under `/spectra` so operators can inspect schema metadata and run
//! explore queries without building custom pages. Registers alongside other product apps via
//! `uf_app!` and requires an authenticated session with `QueryTable` before server functions
//! load catalog or explore data.
//!
//! Orbital inventory macros (`uf_app!`, `orbital_routes_extract`) emit undocumented
//! associated items, so this crate allows `missing_docs` at the crate root while keeping
//! hand-written modules and items documented.
//!
//! ## Features
//!
//! - **Spectra admin routes** — Nested `/spectra` route tree behind auth for schema home,
//!   index, detail, and explore pages. Mount once when the host router starts.
//!   [Get started](#mount-spectra-routes)
//! - **Schema catalog** — [`SchemaIndexPage`] and [`SchemaDetailPage`] list and inspect
//!   registered schemas via [`list_schema_metadata`] and [`get_schema_metadata`].
//!   [Get started](#browse-schemas)
//! - **Event explore** — [`EventExplorePage`] loads paginated event rows and chart
//!   aggregates via [`query_events`] and [`query_event_aggregate`].
//!   [Get started](#explore-events)
//! - **Metric explore** — [`MetricExplorePage`] loads time-series and headline stats via
//!   [`query_metrics`]. [Get started](#explore-metrics)
//! - **Server function wrappers** — [`mod@server`] Higgs `#[server]` fns, per-table Gauge
//!   checks via [`require_spectra_query`], and re-exports of [`spectra_backend`] helpers.
//! - **Permission manifest** — [`SpectraPermission`] and [`SPECTRA_QUERY_PERMISSION`] for
//!   host manifest wiring.
//!
//! ## Mount Spectra routes
//!
//! [`SpectraRoutes`] nests the full `/spectra` subtree inside a host Leptos `<Routes>` tree.
//! Operators get schema catalog pages and event/metric explore views. Mount during host router
//! setup at startup, alongside other `uf_app!` product routes — the macro registers launcher
//! metadata and the `/spectra` inventory entry.
//!
//! **Prerequisites:** `ssr` on this crate; authenticated session; `QueryTable` permission
//! ([`SPECTRA_QUERY_PERMISSION`]); host-injected Spectra query backend for live explore data.
//!
//! ```rust,ignore
//! use leptos::prelude::*;
//! use leptos_router::components::Routes;
//! use spectra_app::SpectraRoutes;
//!
//! view! {
//!     <Routes fallback=|| "not found">
//!         <SpectraRoutes />
//!     </Routes>
//! }
//! ```
//!
//! On success `/spectra` resolves to the schema home, `/spectra/schema` lists registered
//! schemas, and nested detail and explore routes load per-schema pages. Unauthenticated
//! sessions are rejected by server functions — see root `SECURITY.md`.
//!
//! ## Browse schemas
//!
//! Schema pages list every registered Spectra schema and show field metadata on detail.
//! [`SchemaIndexPage`] and [`SpectraHomePage`] call [`list_schema_metadata`] for the index;
//! [`SchemaDetailPage`] calls [`get_schema_metadata`] for one schema name. Open these routes
//! when operators need column types, partition hints, or quick links into explore views.
//!
//! **Prerequisites:** [`SpectraRoutes`] mounted; `ssr` feature; `QueryTable` permission;
//! schema names must pass `spectra_backend::validate_spectra_query_name` on detail lookup.
//!
//! ```rust,ignore
//! use spectra_app::{list_schema_metadata, get_schema_metadata, SchemaIndexPage, SchemaDetailPage};
//! use spectra_core::SchemaListItem;
//!
//! let _index: SchemaIndexPage;
//! let _detail: SchemaDetailPage;
//! let schemas: Vec<SchemaListItem> = list_schema_metadata().await?;
//! let schema_name = schemas.first().map(|s| s.table_or_metric.as_str());
//! assert_eq!(schema_name, Some("ops.events"));
//!
//! let detail = get_schema_metadata("ops.events".into()).await?;
//! assert_eq!(detail.as_ref().map(|d| d.table_or_metric.as_str()), Some("ops.events"));
//! ```
//!
//! On success the index returns sorted [`spectra_core::SchemaListItem`] rows and detail resolves one schema
//! or returns `None` when the name is unknown. Blank or path-unsafe names fail validation
//! before catalog lookup.
//!
//! ## Explore events
//!
//! Event explore pages show a paginated event log grid and optional chart aggregates for one
//! table. [`EventExplorePage`] calls [`query_events`] for row data and [`query_event_aggregate`]
//! for time-series or headline charts. Use this route when operators audit recent rows or
//! bucketed counts for a schema-backed event table.
//!
//! **Prerequisites:** Routes mounted; `QueryTable` plus per-table Gauge `spectra.query.{table}`
//! via [`require_spectra_query`]; table names must pass `spectra_backend::validate_spectra_query_name`.
//!
//! ```rust,ignore
//! use chrono::Utc;
//! use spectra_app::{query_events, query_event_aggregate, EventExplorePage};
//! use spectra_core::{EventQuery, EventAggregateRequest, GridPaginationModel};
//!
//! let query = EventQuery {
//!     table: "ops.events".into(),
//!     start: Utc::now() - chrono::Duration::hours(24),
//!     end: Utc::now(),
//!     partition: None,
//!     pagination: GridPaginationModel::default(),
//!     sort: vec![],
//!     filter: Default::default(),
//! };
//! let rows = query_events(query).await?;
//! assert_eq!(rows.row_count, 0);
//!
//! let agg = EventAggregateRequest {
//!     table: "ops.events".into(),
//!     start: Utc::now() - chrono::Duration::hours(24),
//!     end: Utc::now(),
//!     aggregation: Default::default(),
//!     view: Default::default(),
//! };
//! let chart = query_event_aggregate(agg).await?;
//! assert!(matches!(chart, spectra_core::EventAggregateResult::TimeSeries { .. }));
//! ```
//!
//! On success `rows` carries column definitions and a page of grid rows (empty until the host
//! wires a live backend); chart queries return an aggregate payload shape the Orbital charts
//! can render. Denied Gauge permissions surface as `ServerFnError` before query stubs run.
//!
//! ## Explore metrics
//!
//! Metric explore pages chart time-series and headline stats for one metric family.
//! [`MetricExplorePage`] calls [`query_metrics`] with a [`spectra_core::MetricsQuery`] describing the
//! metric name, time range, and label matchers. Open this route when operators inspect
//! throughput, latency, or custom counters registered in Spectra.
//!
//! **Prerequisites:** Routes mounted; `QueryTable` plus per-metric Gauge `spectra.query.{metric}`
//! via [`require_spectra_query`]; metric names must pass `spectra_backend::validate_spectra_query_name`.
//!
//! ```rust,ignore
//! use chrono::Utc;
//! use spectra_app::{query_metrics, MetricExplorePage};
//! use spectra_core::MetricsQuery;
//!
//! let query = MetricsQuery {
//!     metric: "ops.request.duration".into(),
//!     start: Utc::now() - chrono::Duration::hours(1),
//!     end: Utc::now(),
//!     step_secs: Some(60),
//!     label_matchers: vec![],
//! };
//! let result = query_metrics(query).await?;
//! assert_eq!(result.series.len(), 0);
//! ```
//!
//! On success `result` carries `series` and `headline` vectors (empty until the host injects
//! a live metrics backend). Oversized or path-unsafe metric names are rejected before permission
//! checks.
//!
//! ## Feature flags
//!
//! | Flag | Effect |
//! |------|--------|
//! | `ssr` | Server-side Leptos split; required for `#[server]` fns and Higgs/Gauge IO. |
//! | `hydrate` | Client-side hydration for routed pages and Orbital shell components. |
//!
//! ## Routes
//!
//! Mounted under `/spectra` by [`SpectraRoutes`]. Every server fn requires an authenticated
//! session and [`SPECTRA_QUERY_PERMISSION`]. Explore queries additionally call
//! [`server::require_spectra_query`] for Gauge `spectra.query.{table}` before the
//! (currently stubbed, host-injected) query backend runs.
//!
//! | Path | Page | Key server fn(s) |
//! |---|---|---|
//! | `/spectra` | [`SpectraHomePage`] | [`list_schema_metadata`] |
//! | `/spectra/schema` | [`SchemaIndexPage`] | [`list_schema_metadata`] |
//! | `/spectra/schema/:name` | [`SchemaDetailPage`] | [`get_schema_metadata`] |
//! | `/spectra/schema/:name/explore` | [`EventExplorePage`] | [`query_events`], [`query_event_aggregate`] |
//! | `/spectra/metric/:name/explore` | [`MetricExplorePage`] | [`query_metrics`] |
//!
//! ## Examples
//!
//! Start with [Mount Spectra routes](#mount-spectra-routes). The `spectra-backend` unit and integ
//! suites in `docs/VERIFICATION.md` cover server-fn contracts. Runnable host:
//! `examples/protected-spectra-host` (deny/allow + schema index; inventory `spectra` / `/spectra`).
//!
//! ## Where to look next
//!
//! - [`SpectraLayout`] — shared app bar / nav shell wrapping every route.
//! - [`mod@server`] — server functions backing the UI, including [`require_spectra_query`].
//! - [`SpectraPermission`] / [`SPECTRA_QUERY_PERMISSION`] — permission enum and QueryTable name.
//! - `spectra_backend` — name validation, catalog helpers, and explore stub payloads.

#![allow(missing_docs)]
#![cfg_attr(
    feature = "ssr",
    allow(dead_code, unused_imports, unused_variables, unknown_lints)
)]
use leptos::prelude::*;
use leptos_router::{
    components::{ParentRoute, Route, ToHref},
    path, Lazy,
};
use uf_product_macros::uf_app;

mod components;
mod explore_time;
mod layout;
mod lazy_routes;
pub mod pages;
mod permissions;
pub mod server;

pub use permissions::SpectraPermission;

pub use layout::SpectraLayout;
pub use lazy_routes::{
    prefetch_family, EventExploreRoute, MetricExploreRoute, SchemaDetailRoute, SchemaIndexRoute,
    SpectraHomeRoute, SpectraLayoutRouteView,
};
pub use pages::{
    EventExplorePage, MetricExplorePage, SchemaDetailPage, SchemaIndexPage, SpectraHomePage,
};
pub use server::{
    get_schema_metadata, get_spectra_dashboard_summary, list_schema_metadata,
    query_event_aggregate, query_events, query_metrics, require_spectra_query,
    SpectraDashboardSummary, SPECTRA_QUERY_PERMISSION,
};

uf_app! {
    name: "Spectra",
    id: "spectra",
    description: "Log and metric explorer",
    icon: "📊",
    version: "0.1.0",
    routes: SpectraRoutes,
    route_path: "/spectra",
    permission_manifest: permissions::SpectraPermission,
}

/// Spectra's nested route tree, gated behind an auth guard and mounted at `/spectra`.
///
/// Leaf pages are [`LazyRoute`](leptos_router::LazyRoute) views so
/// `cargo leptos --split` can emit a separate WASM chunk for this family.
/// Registers the home, schema index/detail, and event/metric explore routes. Intended to be
/// used inside a host `<Routes>` component, e.g. `<SpectraRoutes />`.
#[allow(missing_docs)]
#[orbital_macros::orbital_routes_extract]
#[component(transparent)]
pub fn SpectraRoutes() -> impl leptos_router::MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("spectra") view=SpectraLayoutRouteView>
            <Route path=path!("") view={Lazy::<SpectraHomeRoute>::new()} />
            <Route path=path!("schema") view={Lazy::<SchemaIndexRoute>::new()} />
            <Route path=path!("schema/:name") view={Lazy::<SchemaDetailRoute>::new()} />
            <Route path=path!("metric/:name/explore") view={Lazy::<MetricExploreRoute>::new()} />
            <Route path=path!("schema/:name/explore") view={Lazy::<EventExploreRoute>::new()} />
        </ParentRoute>
    }
    .into_inner()
}
