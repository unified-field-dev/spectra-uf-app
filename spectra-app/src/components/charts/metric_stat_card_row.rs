use leptos::prelude::*;
use orbital::components::{StatCard, StatCardVariant};
use orbital::primitives::Flex;
use spectra_core::StatCardDto;

#[component]
pub fn MetricStatCardRow(headline: Vec<StatCardDto>) -> impl IntoView {
    view! {
        <Flex>
            {headline.into_iter().map(|c| {
                let label: &'static str =
                    Box::leak(c.label.clone().into_boxed_str());
                let value = Signal::derive(move || c.value.clone());
                view! {
                    <StatCard label=label value=value />
                }
            }).collect_view()}
        </Flex>
    }
}
