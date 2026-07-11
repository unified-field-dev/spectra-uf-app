use leptos::prelude::*;
use spectra_core::MetricsQuery;

use crate::components::charts::{MetricStatCardRow, MetricTimeSeriesChart};
use crate::components::query::{ChartSkeleton, PermissionDeniedState};
use crate::explore_time::range_from_secs;
use crate::server::query_metrics;

use super::metric_toolbar::MetricToolbar;

#[component]
pub fn MetricExplorePanel(
    metric_name: Memo<String>,
    range_secs: ReadSignal<i64>,
    set_range_secs: WriteSignal<i64>,
) -> impl IntoView {
    let query_res = Resource::new(
        move || (metric_name.get(), range_secs.get()),
        |(metric, secs)| async move {
            let (start, end) = range_from_secs(secs);
            query_metrics(MetricsQuery {
                metric,
                start,
                end,
                step_secs: Some(60),
                label_matchers: Vec::new(),
            })
            .await
        },
    );

    view! {
        <MetricToolbar
            range_secs=Signal::derive(move || range_secs.get())
            on_range=Callback::new(move |s| set_range_secs.set(s))
        />
        <Transition fallback=ChartSkeleton>
            {move || match query_res.get() {
                Some(Ok(data)) => view! {
                    <MetricStatCardRow headline=data.headline />
                    <MetricTimeSeriesChart />
                }.into_any(),
                Some(Err(e)) if e.to_string().contains("Permission denied") => {
                    view! { <PermissionDeniedState /> }.into_any()
                }
                Some(Err(e)) => view! { <p>{e.to_string()}</p> }.into_any(),
                None => view! { <ChartSkeleton /> }.into_any(),
            }}
        </Transition>
    }
}
