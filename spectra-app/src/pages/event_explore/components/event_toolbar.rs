use leptos::prelude::*;
use spectra_core::{EventAggregationSpec, EventExploreView};

use crate::components::query::{
    EventAggregationBar, EventViewPicker, QueryToolbarMaterial, TimeRangePicker,
};

#[component]
pub fn EventToolbar(
    #[prop(into)] range_secs: Signal<i64>,
    on_range: Callback<i64>,
    #[prop(into)] view: Signal<EventExploreView>,
    on_view: Callback<EventExploreView>,
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
