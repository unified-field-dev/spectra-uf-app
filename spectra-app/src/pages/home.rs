use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

/// Spectra home: entry point listing registered schemas.
#[component]
pub fn SpectraHomePage() -> impl IntoView {
    let navigate = use_navigate();
    Effect::new(move |_| {
        let _ = navigate("/spectra/schema", Default::default());
    });
    view! { <span>"Redirecting…"</span> }
}
