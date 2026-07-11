use leptos::prelude::*;
use spectra_core::EventAggregateResult;

use crate::components::charts::{EventAggregateStatRow, EventTimeSeriesChart};

#[component]
pub fn EventSeriesPanel(result: EventAggregateResult) -> impl IntoView {
    match result {
        EventAggregateResult::TimeSeries { headline, .. } => {
            view! {
                <EventAggregateStatRow headline=headline />
                <EventTimeSeriesChart />
            }
            .into_any()
        }
        _ => view! { <span>"No series data"</span> }.into_any(),
    }
}
