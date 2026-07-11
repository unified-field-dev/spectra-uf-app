use leptos::prelude::*;
use spectra_core::{EventAggregateResult, EventExploreView, EventQueryResult};

use super::{EventLogPanel, EventPiePanel, EventSeriesPanel};

#[component]
pub fn EventExploreViewport(
    view: EventExploreView,
    row_result: Option<EventQueryResult>,
    aggregate_result: Option<EventAggregateResult>,
) -> impl IntoView {
    view! {
        <div data-testid="spectra-event-explore-viewport">
            {match view {
                EventExploreView::EventLog => {
                    row_result.map(|r| view! { <EventLogPanel result=r /> }.into_any())
                }
                EventExploreView::TimeSeries | EventExploreView::LineChart => {
                    aggregate_result.map(|r| view! { <EventSeriesPanel result=r /> }.into_any())
                }
                EventExploreView::PieChart | EventExploreView::BarChart => {
                    aggregate_result.map(|r| view! { <EventPiePanel result=r /> }.into_any())
                }
            }}
        </div>
    }
}
