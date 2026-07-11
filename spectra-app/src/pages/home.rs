use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn SpectraHomePage() -> impl IntoView {
    let navigate = use_navigate();
    Effect::new(move |_| {
        let _ = navigate("/spectra/schema", Default::default());
    });
    view! { <span>"Redirecting…"</span> }
}
