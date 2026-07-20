use leptos::prelude::*;
use spectra_core::{EventAggregationSpec, EventExploreView};

use crate::components::query::{
    EventAggregationBar, EventViewPicker, QueryToolbarMaterial, TimeRangePicker,
};

#[component]
pub fn EventToolbar(
    /// Reactive signal for the range secs.
    #[prop(into)] range_secs: Signal<i64>,
    /// Callback invoked when range occurs.
    on_range: Callback<i64>,
    /// Reactive signal for the current view selection.
    #[prop(into)] view: Signal<EventExploreView>,
    /// Callback invoked when view occurs.
    on_view: Callback<EventExploreView>,
    /// Two-way signal holding the aggregation mode to apply.
    aggregation: RwSignal<EventAggregationSpec>,
) -> impl IntoView {
    view! {
        <QueryToolbarMaterial>
            <TimeRangePicker selected_secs=range_secs on_change=on_range />
            <EventViewPicker view=view on_change=on_view />
            <EventAggregationBar view=view.get_untracked() spec=aggregation />
        </QueryToolbarMaterial>
    }
}
