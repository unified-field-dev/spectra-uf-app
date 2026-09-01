use leptos::prelude::*;
use orbital::components::Caption1;
use orbital::primitives::{Button, ButtonAppearance, Flex};
use spectra_core::EventExploreView;

const VIEWS: &[(EventExploreView, &str)] = &[
    (EventExploreView::EventLog, "Event log"),
    (EventExploreView::TimeSeries, "Time series"),
    (EventExploreView::LineChart, "Line chart"),
    (EventExploreView::BarChart, "Bar chart"),
    (EventExploreView::PieChart, "Pie chart"),
];

#[component]
pub fn EventViewPicker(
    /// Reactive signal for the current view selection.
    #[prop(into)]
    view: Signal<EventExploreView>,
    /// Callback invoked when the value changes.
    on_change: Callback<EventExploreView>,
) -> impl IntoView {
    view! {
        <div data-testid="spectra-event-view-picker">
            <Caption1>"View"</Caption1>
            <Flex>
                {VIEWS.iter().map(|(v, label)| {
                    let v = *v;
                    let label = *label;
                    let appearance = move || {
                        if view.get() == v {
                            ButtonAppearance::Primary
                        } else {
                            ButtonAppearance::Secondary
                        }
                    };
                    view! {
                        <Button appearance=Signal::derive(appearance) on:click=move |_| on_change.run(v)>
                            {label}
                        </Button>
                    }
                }).collect_view()}
            </Flex>
        </div>
    }
}
