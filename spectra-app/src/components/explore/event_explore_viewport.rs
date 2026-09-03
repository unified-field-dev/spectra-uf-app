use leptos::prelude::*;
use spectra_core::{EventAggregateResult, EventExploreView, EventQueryResult};

use super::{EventLogPanel, EventPiePanel, EventSeriesPanel};

#[component]
pub fn EventExploreViewport(
    /// Current view selection.
    view: EventExploreView,
    /// Optional row result.
    row_result: Option<EventQueryResult>,
    /// Optional aggregate result.
    aggregate_result: Option<EventAggregateResult>,
) -> impl IntoView {
    view! {
        <div id="spectra-event-explore-viewport" data-testid="spectra-event-explore-viewport">
            {match view {
                EventExploreView::EventLog => {
                    row_result.map(|r| view! { <EventLogPanel result=r /> }.into_any())
                }
                EventExploreView::TimeSeries | EventExploreView::LineChart => {
                    aggregate_result.map(|r| view! { <EventSeriesPanel result=r /> }.into_any())
                }
                EventExploreView::PieChart | EventExploreView::BarChart => {
                    aggregate_result.map(|r| view! {
                        <EventPiePanel view=view result=r />
                    }.into_any())
                }
            }}
        </div>
    }
}
