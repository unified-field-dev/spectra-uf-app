use leptos::prelude::*;
use spectra_core::EventAggregateResult;

use crate::components::charts::{EventAggregateStatRow, EventPieChart};

#[component]
pub fn EventPiePanel(
    /// Result data to render.
    result: EventAggregateResult,
) -> impl IntoView {
    match result {
        EventAggregateResult::Slices { headline, .. } => {
            view! {
                <EventAggregateStatRow headline=headline />
                <EventPieChart />
            }
            .into_any()
        }
        _ => view! { <span>"No slice data"</span> }.into_any(),
    }
}
