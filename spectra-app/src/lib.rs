#![recursion_limit = "256"]
//! Spectra operations app: routes and UI composition for exploring logged events and
//! metrics under `/spectra`.
//!
//! Spectra itself is a log/metric storage crate with no built-in UI; this crate is the
//! `#[uf_product_macros::uf_app]`-registered operations surface a host mounts to give
//! operators a way to browse schemas and explore event/metric data.
//!
//! Orbital inventory macros (`uf_app!`, `orbital_routes_extract`) emit undocumented
//! associated items, so this crate allows `missing_docs` at the crate root while keeping
//! hand-written modules and items documented.
//!
//! ## Organized by task
//!
//! | Task | Start here |
//! |------|------------|
//! | **Mount `/spectra` routes** | [`SpectraRoutes`] |
//! | **Browse schema home / index** | [`SpectraHomePage`], [`SchemaIndexPage`], [`list_schema_metadata`] |
//! | **Inspect a schema** | [`SchemaDetailPage`], [`get_schema_metadata`] |
//! | **Explore events** | [`EventExplorePage`], [`query_events`], [`query_event_aggregate`] |
//! | **Explore a metric** | [`MetricExplorePage`], [`query_metrics`] |
//! | **Permission gates** | [`SpectraPermission`], [`SPECTRA_QUERY_PERMISSION`], [`server::require_spectra_query`] |
//! | **Pure catalog / query stubs** | `spectra-backend` (not this crate) |
//!
//! ## Owns / does not own
//!
//! **Owns:** Leptos pages, Higgs `#[server]` wrappers, layout/nav shell, permission
//! manifest, and `uf_app!` / [`SpectraRoutes`] registration.
//!
//! **Does not own:** Schema catalog or explore-query stub helpers (`spectra-backend`);
//! Spectra core storage or live query backends (Spectra core / host injection); full
//! Leptos SSR host binaries (live outside this repository).
//!
//! ## Concern → API
//!
//! | Concern | API | Owner |
//! |---------|-----|-------|
//! | Mount `/spectra` | [`SpectraRoutes`], [`SpectraLayout`] | this crate |
//! | Ops pages | [`SpectraHomePage`], [`SchemaIndexPage`], [`SchemaDetailPage`], [`EventExplorePage`], [`MetricExplorePage`] | this crate |
//! | Server fns | [`list_schema_metadata`], [`get_schema_metadata`], [`query_events`], [`query_event_aggregate`], [`query_metrics`], [`SPECTRA_QUERY_PERMISSION`] | this crate ([`mod@server`]) |
//! | Name validation / stubs | `spectra_backend::SpectraQueryNameError`, `spectra_backend::validate_spectra_query_name` | `spectra-backend` |
//! | Permission manifest | [`SpectraPermission`] | this crate |
//!
//! ## Routes (Concern → page → server fn)
//!
//! Mounted under `/spectra` by [`SpectraRoutes`]. Every server fn requires an
//! authenticated session and [`SPECTRA_QUERY_PERMISSION`]. Explore queries additionally call
//! [`server::require_spectra_query`] for Gauge `spectra.query.{table}` before the
//! (currently stubbed, host-injected) query backend runs.
//!
//! | Path | Page | Key server fn(s) |
//! |---|---|---|
//! | `/spectra` | [`SpectraHomePage`] | [`list_schema_metadata`] |
//! | `/spectra/schema` | [`SchemaIndexPage`] | [`list_schema_metadata`] |
//! | `/spectra/schema/:name` | [`SchemaDetailPage`] | [`get_schema_metadata`] |
//! | `/spectra/schema/:name/explore` | [`EventExplorePage`] | [`query_events`] → [`server::require_spectra_query`], [`query_event_aggregate`] → [`server::require_spectra_query`] |
//! | `/spectra/metric/:name/explore` | [`MetricExplorePage`] | [`query_metrics`] → [`server::require_spectra_query`] |
//!
//! ## Getting started
//!
//! Mount [`SpectraRoutes`] inside your host's `<Routes>`; it registers the `/spectra`
//! subtree (auth-gated) and, via `uf_app!`, its launcher metadata:
//!
//! ```rust,ignore
//! use leptos::prelude::*;
//! use leptos_router::components::Routes;
//! use spectra_app::SpectraRoutes;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <Routes fallback=|| "not found">
//!             <SpectraRoutes />
//!         </Routes>
//!     }
//! }
//! ```
//!
//! ## Examples ladder
//!
//! | Level | Where |
//! |-------|--------|
//! | Highlight | Getting started above |
//! | Mid | `spectra-backend` unit + integ suites (`docs/VERIFICATION.md`) |
//! | Detailed | `examples/protected-spectra-host` (deny/allow + schema index; inventory `spectra` / `/spectra`; copy README) |
//!
//! ## Where to look next
//!
//! - [`SpectraRoutes`] — the route entrypoint mounted by hosts.
//! - [`SpectraLayout`] — the shared app bar / nav shell wrapping every route.
//! - [`pages`] — the page components listed under Organized by task above.
//! - [`mod@server`] — server functions backing the UI, including the
//!   [`server::require_spectra_query`] permission gate.
//! - [`SpectraPermission`] / [`SPECTRA_QUERY_PERMISSION`] — permission enum and
//!   QueryTable name for host manifest wiring.
//! - `spectra_backend::SpectraQueryNameError` — typed blank-name rejection before
//!   Gauge / explore stubs.

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
    get_schema_metadata, list_schema_metadata, query_event_aggregate, query_events, query_metrics,
    require_spectra_query, SPECTRA_QUERY_PERMISSION,
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
