#![recursion_limit = "256"]
//! Spectra observability UI (`/spectra`).

use leptos::prelude::*;
use leptos_router::{components::*, path};
use uf_product_macros::orbital_app;

pub mod components;
pub mod explore_time;
pub mod layout;
pub mod pages;
pub mod permissions;
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
    permission_manifest: SpectraPermission,
}

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
