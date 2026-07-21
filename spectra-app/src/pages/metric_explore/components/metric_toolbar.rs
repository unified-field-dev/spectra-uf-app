use leptos::prelude::*;

use crate::components::query::{QueryToolbarMaterial, TimeRangePicker};

#[component]
pub fn MetricToolbar(
    /// Reactive signal for the range secs.
    #[prop(into)]
    range_secs: Signal<i64>,
    /// Callback invoked when range occurs.
    on_range: Callback<i64>,
) -> impl IntoView {
    view! {
        <QueryToolbarMaterial>
            <TimeRangePicker selected_secs=range_secs on_change=on_range />
        </QueryToolbarMaterial>
    }
}
