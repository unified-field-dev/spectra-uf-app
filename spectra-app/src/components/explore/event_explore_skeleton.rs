use leptos::prelude::*;
use spectra_core::EventExploreView;

use crate::components::query::ChartSkeleton;

#[component]
pub fn EventExploreSkeleton(view: EventExploreView) -> impl IntoView {
    let _ = view;
    view! { <ChartSkeleton /> }
}
