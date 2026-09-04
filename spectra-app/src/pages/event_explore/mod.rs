mod components;

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use orbital::components::{ContentContainer, SpacingSize, Title3};
use orbital::primitives::Flex;
use spectra_core::{EventAggregationSpec, EventExploreView, EventMeasure};

use self::components::EventExplorePanel;

/// Event explore view: query, filter, and visualize logged events for a schema.
#[component]
pub fn EventExplorePage() -> impl IntoView {
    let params = use_params_map();
    let table = Memo::new(move |_| params.with(|p| p.get("name").unwrap_or_default()));
    let (range_secs, set_range_secs) = signal(3600i64);
    let (view, set_view) = signal(EventExploreView::EventLog);
    let aggregation = RwSignal::new(EventAggregationSpec {
        measure: EventMeasure::Count,
        measure_field: None,
        time_bucket_secs: Some(3600),
        group_by_field: None,
    });

    view! {
        <ContentContainer data_testid="spectra-event-explore-panel">
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap()>
                <Title3>"Explore event rows"</Title3>
                <EventExplorePanel
                    table=table
                    range_secs=range_secs
                    set_range_secs=set_range_secs
                    view=view
                    set_view=set_view
                    aggregation=aggregation
                />
            </Flex>
        </ContentContainer>
    }
}
