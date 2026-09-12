use leptos::prelude::*;
use orbital::components::EmptyState;
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
        EventAggregateResult::Slices { .. } => view! {
            <div data-testid="spectra-event-empty-state">
                <EmptyState
                    message="No series data"
                    description="This view expects a time series. Switch to Bar or Pie for grouped slices."
                />
            </div>
        }
        .into_any(),
    }
}

#[component]
pub fn EventPiePanel(
    /// Current chart view.
    view: EventExploreView,
    /// Result data to render.
    result: EventAggregateResult,
    /// Aggregation group-by field from the toolbar (empty means user must choose).
    group_by_field: Option<String>,
) -> impl IntoView {
    let group_by_missing = group_by_field
        .as_ref()
        .map(|s| s.trim().is_empty())
        .unwrap_or(true);
    if group_by_missing {
        return view! {
            <div data-testid="spectra-event-empty-state">
                <EmptyState
                    message="Choose a group-by field"
                    description="Bar and pie charts need a field name to group event rows."
                />
            </div>
        }
        .into_any();
    }

    match result {
        EventAggregateResult::Slices { slices, headline } => {
            if slices.is_empty() {
                return view! {
                    <div data-testid="spectra-event-empty-state">
                        <EmptyState
                            message="No slice data"
                            description="Nothing to chart for this group-by field and time range."
                        />
                    </div>
                }
                .into_any();
            }
            view! {
                <EventAggregateStatRow headline=headline />
                {match view {
                    EventExploreView::BarChart => {
                        view! { <EventBarChart slices=slices.clone() /> }.into_any()
                    }
                    _ => view! { <EventPieChart slices=slices /> }.into_any(),
                }}
            }
            .into_any()
        }
        EventAggregateResult::TimeSeries { .. } => view! {
            <div data-testid="spectra-event-empty-state">
                <EmptyState
                    message="No slice data"
                    description="This view expects grouped slices. Check the group-by field and try again."
                />
            </div>
        }
        .into_any(),
    }
}
