mod components;

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use orbital::components::{ContentContainer, SpacingSize, Title3};
use orbital::primitives::Flex;

use self::components::MetricExplorePanel;

/// Metric explore view: query and chart a single metric over a selectable time range.
#[component]
pub fn MetricExplorePage() -> impl IntoView {
    let params = use_params_map();
    let metric_name = Memo::new(move |_| {
        params.with(|p| p.get("name").map(|s| s.to_string()).unwrap_or_default())
    });
    let (range_secs, set_range_secs) = signal(3600i64);

    view! {
        <ContentContainer data_testid="spectra-metric-explore-panel">
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap()>
                <Title3>{move || metric_name.get()}</Title3>
                <MetricExplorePanel
                    metric_name=metric_name
                    range_secs=range_secs
                    set_range_secs=set_range_secs
                />
            </Flex>
        </ContentContainer>
    }
}
