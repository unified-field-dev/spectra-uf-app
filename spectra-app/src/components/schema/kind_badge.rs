use leptos::prelude::*;
use orbital::components::Caption1;

#[component]
pub fn KindBadge(
    /// Kind or category.
    kind: String,
) -> impl IntoView {
    let label = match kind.as_str() {
        "metric" => "Metric",
        "event" => "Event",
        _ => "Schema",
    };
    view! { <Caption1>{label}</Caption1> }
}
