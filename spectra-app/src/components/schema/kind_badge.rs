use leptos::prelude::*;
use orbital::primitives::{Badge, BadgeAppearance, BadgeColor, BadgeSize};

#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn KindBadge(
    /// Kind or category.
    kind: String,
) -> impl IntoView {
    let (label, color) = match kind.as_str() {
        "metric" => ("Metric", BadgeColor::Informative),
        "event" => ("Event", BadgeColor::Success),
        _ => ("Schema", BadgeColor::Brand),
    };
    view! {
        <Badge
            appearance=Signal::from(BadgeAppearance::Tint)
            color=Signal::from(color)
            size=Signal::from(BadgeSize::Small)
        >
            {label}
        </Badge>
    }
}
