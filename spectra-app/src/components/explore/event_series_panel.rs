use leptos::prelude::*;
use spectra_core::{EventAggregateResult, EventExploreView};

use crate::components::charts::{
    EventAggregateStatRow, EventBarChart, EventPieChart, EventTimeSeriesChart,
};

#[component]
pub fn EventSeriesPanel(
    /// Result data to render.
    result: EventAggregateResult,
) -> impl IntoView {
    match result {
        EventAggregateResult::TimeSeries { series, headline } => view! {
            <EventAggregateStatRow headline=headline />
            <EventTimeSeriesChart series=series />
        }
        .into_any(),
        EventAggregateResult::Slices { .. } => view! { <span>"No series data"</span> }.into_any(),
    }
}

#[component]
pub fn EventPiePanel(
    /// Current chart view.
    view: EventExploreView,
    /// Result data to render.
    result: EventAggregateResult,
) -> impl IntoView {
    match result {
        EventAggregateResult::Slices { slices, headline } => view! {
            <EventAggregateStatRow headline=headline />
            {match view {
                EventExploreView::BarChart => view! { <EventBarChart slices=slices.clone() /> }.into_any(),
                _ => view! { <EventPieChart slices=slices /> }.into_any(),
            }}
        }
        .into_any(),
        EventAggregateResult::TimeSeries { .. } => {
            view! { <span>"No slice data"</span> }.into_any()
        }
    }
}
