#![recursion_limit = "256"]
//! Spectra operations app: routes and UI composition for exploring logged events and
//! metrics under `/spectra`.
//!
//! Spectra itself is a log/metric storage crate with no built-in UI; this crate is the
//! `#[uf_product_macros::orbital_app]`-registered operations surface a host mounts to give
//! operators a way to browse schemas and explore event/metric data.
//!
//! ## Features
//!
//! - **Home** — [`SpectraHomePage`] lists registered schemas as an entry point.
//! - **Schema browsing** — [`SchemaIndexPage`] / [`SchemaDetailPage`] for inspecting
//!   registered event/metric schemas.
//! - **Event explore** — [`EventExplorePage`] for querying and visualizing events in a table
//!   (log view, time series, and pie breakdowns).
//! - **Metric explore** — [`MetricExplorePage`] for querying and charting a single metric
//!   over time.
//! - **Read API** — [`server`] exposes the SSR-only server functions backing the pages
//!   above, gated by [`server::require_spectra_query`].
//!
//! ## Getting started
//!
//! Mount [`SpectraRoutes`] inside your host's `<Routes>`; it registers the `/spectra`
//! subtree (auth-gated) and, via `orbital_app!`, its launcher metadata:
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
//! ## Where to look next
//!
//! - [`SpectraRoutes`] — the route entrypoint mounted by hosts.
//! - [`SpectraLayout`] — the shared app bar / nav shell wrapping every route.
//! - [`mod@server`] — server functions backing the UI.

#![allow(missing_docs)]
#![allow(clippy::unused_unit, unused_imports)]
#![cfg_attr(
    feature = "ssr",
    allow(
        dead_code,
        unused_imports,
        unused_variables,
        unknown_lints,
        clippy::all,
    )
)]
use leptos::prelude::*;
use leptos_router::{components::*, path};
use uf_product_macros::orbital_app;

mod components;
mod explore_time;
mod layout;
pub mod pages;
mod permissions;
pub mod server;

pub use permissions::SpectraPermission;

pub use layout::SpectraLayout;
pub use pages::{
    EventExplorePage, MetricExplorePage, SchemaDetailPage, SchemaIndexPage, SpectraHomePage,
};
#[cfg(feature = "ssr")]
pub use server::{
    get_schema_metadata, list_schema_metadata, query_event_aggregate, query_events, query_metrics,
};

#[component]
fn SpectraAuthGuard() -> impl IntoView {
    view! {
        <div data-testid="spectra-auth-guard-root">
            <orbital::routes::RequireAuthenticated>
                <SpectraLayout />
            </orbital::routes::RequireAuthenticated>
        </div>
    }
}

orbital_app! {
    name: "Spectra",
    id: "spectra",
    description: "Log and metric explorer",
    icon: "📊",
    version: "0.1.0",
    routes: SpectraRoutes,
    route_path: "/spectra",
}

/// Spectra's nested route tree, gated behind an auth guard and mounted at `/spectra`.
///
/// Registers the home, schema index/detail, and event/metric explore routes. Intended to be
/// used inside a host `<Routes>` component, e.g. `<SpectraRoutes />`.
#[allow(missing_docs)]
#[orbital_macros::orbital_routes_extract]
#[component(transparent)]
pub fn SpectraRoutes() -> impl leptos_router::MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("spectra") view=SpectraAuthGuard>
            <Route path=path!("") view=SpectraHomePage />
            <Route path=path!("schema") view=SchemaIndexPage />
            <Route path=path!("schema/:name") view=SchemaDetailPage />
            <Route path=path!("metric/:name/explore") view=MetricExplorePage />
            <Route path=path!("schema/:name/explore") view=EventExplorePage />
        </ParentRoute>
    }
    .into_inner()
}
