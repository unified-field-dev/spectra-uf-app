use leptos::prelude::*;
use orbital::components::Caption1;
use orbital::primitives::{Flex, Input, Select};
use spectra_core::{EventAggregationSpec, EventExploreView, EventMeasure};

#[component]
pub fn EventAggregationBar(
    view: EventExploreView,
    spec: RwSignal<EventAggregationSpec>,
) -> impl IntoView {
    if view == EventExploreView::EventLog {
        return view! { <span></span> }.into_any();
    }

    let bucket = RwSignal::new(
        spec.get_untracked()
            .time_bucket_secs
            .unwrap_or(3600)
            .to_string(),
    );
    let group_by = RwSignal::new(
        spec.get_untracked()
            .group_by_field
            .clone()
            .unwrap_or_default(),
    );
    let measure_str = RwSignal::new(if spec.get_untracked().measure == EventMeasure::Count {
        "count".to_string()
    } else {
        "sum".to_string()
    });

    Effect::new(move |_| {
        if let Ok(v) = bucket.get().parse::<u64>() {
            spec.update(|s| s.time_bucket_secs = Some(v));
        }
        let gb = group_by.get();
        spec.update(|s| {
            s.group_by_field = if gb.is_empty() { None } else { Some(gb) };
        });
        let m = measure_str.get();
        spec.update(|s| {
            s.measure = if m == "sum" {
                EventMeasure::Sum
            } else {
                EventMeasure::Count
            };
        });
    });

    view! {
        <Flex vertical=true>
            <Caption1>"Measure"</Caption1>
            <Select bind=measure_str>
                <option value="count">"Count"</option>
                <option value="sum">"Sum"</option>
            </Select>
            {(view == EventExploreView::TimeSeries || view == EventExploreView::LineChart).then(|| view! {
                <>
                    <Caption1>"Time bucket (seconds)"</Caption1>
                    <Input bind=bucket />
                </>
            })}
            {(view == EventExploreView::PieChart || view == EventExploreView::BarChart).then(|| view! {
                <>
                    <Caption1>"Group by field"</Caption1>
                    <Input bind=group_by />
                </>
            })}
        </Flex>
    }
    .into_any()
}
