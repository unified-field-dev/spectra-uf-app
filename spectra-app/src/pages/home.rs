use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;

/// Spectra home: entry point listing registered schemas.
#[component]
pub fn SpectraHomePage() -> impl IntoView {
    let navigate = use_navigate();
    Effect::new(move |_| {
        navigate("/spectra/schema", NavigateOptions::default());
    });
    view! { <span>"Redirecting…"</span> }
}
