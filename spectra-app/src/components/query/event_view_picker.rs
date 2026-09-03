use leptos::prelude::*;
use orbital::components::Caption1;
use orbital::primitives::{Button, ButtonAppearance, Flex};
use spectra_core::EventExploreView;

const VIEWS: &[(EventExploreView, &str, &str)] = &[
    (
        EventExploreView::EventLog,
        "Event log",
        "spectra-event-view-event-log",
    ),
    (
        EventExploreView::TimeSeries,
        "Time series",
        "spectra-event-view-time-series",
    ),
    (
        EventExploreView::LineChart,
        "Line chart",
        "spectra-event-view-line-chart",
    ),
    (
        EventExploreView::BarChart,
        "Bar chart",
        "spectra-event-view-bar-chart",
    ),
    (
        EventExploreView::PieChart,
        "Pie chart",
        "spectra-event-view-pie-chart",
    ),
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
        <div id="spectra-event-view-picker" data-testid="spectra-event-view-picker">
            <Caption1>"View"</Caption1>
            <Flex>
                {VIEWS.iter().map(|(v, label, test_id)| {
                    let v = *v;
                    let label = *label;
                    let test_id = *test_id;
                    let appearance = move || {
                        if view.get() == v {
                            ButtonAppearance::Primary
                        } else {
                            ButtonAppearance::Secondary
                        }
                    };
                    view! {
                        <span data-testid=test_id>
                            <Button appearance=Signal::derive(appearance) on:click=move |_| on_change.run(v)>
                                {label}
                            </Button>
                        </span>
                    }
                }).collect_view()}
            </Flex>
        </div>
    }
}
