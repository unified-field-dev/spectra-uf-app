use leptos::prelude::*;
use orbital::primitives::{Button, Flex};

const PRESETS: &[(&str, i64)] = &[
    ("1h", 3600),
    ("6h", 6 * 3600),
    ("24h", 24 * 3600),
    ("7d", 7 * 24 * 3600),
];

#[component]
pub fn TimeRangePicker(
    /// Reactive signal for the selected secs.
    #[prop(into)] selected_secs: Signal<i64>,
    /// Callback invoked when the value changes.
    on_change: Callback<i64>,
) -> impl IntoView {
    let _ = selected_secs;
    view! {
        <Flex>
            {PRESETS.iter().map(|(label, secs)| {
                let secs = *secs;
                let label = *label;
                view! {
                    <Button on:click=move |_| on_change.run(secs)>
                        {label}
                    </Button>
                }
            }).collect_view()}
        </Flex>
    }
}
