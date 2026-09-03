use leptos::prelude::*;
use orbital::primitives::{MessageBar, MessageBarIntent};
use serde::{Deserialize, Serialize};
use spectra_core::{
    EventAggregateRequest, EventAggregateResult, EventAggregationSpec, EventExploreView,
    EventQuery, EventQueryResult, GridFilterModel, GridPaginationModel, GridSortDirection,
    GridSortItem,
};

use crate::components::explore::{EventExploreSkeleton, EventExploreViewport};
use crate::components::query::PermissionDeniedState;
use crate::explore_time::range_from_secs;
use crate::server::{query_event_aggregate, query_events, server_fn_is_permission_denied};

use super::event_toolbar::EventToolbar;

#[derive(Clone, Serialize, Deserialize)]
enum ExploreData {
    Rows(EventQueryResult),
    Aggregate(EventAggregateResult),
}

#[component]
pub fn EventExplorePanel(
    /// Reactive signal for the table identifier.
    table: Memo<String>,
    /// Reactive signal for the range secs.
    range_secs: ReadSignal<i64>,
    /// Setter used to update the range secs.
    set_range_secs: WriteSignal<i64>,
    /// Reactive signal for the current view selection.
    view: ReadSignal<EventExploreView>,
    /// Setter used to update the view.
    set_view: WriteSignal<EventExploreView>,
    /// Two-way signal holding the aggregation mode to apply.
    aggregation: RwSignal<EventAggregationSpec>,
) -> impl IntoView {
    let explore_res = Resource::new(
        move || (table.get(), range_secs.get(), view.get(), aggregation.get()),
        |(table, secs, view, agg)| async move {
            let (start, end) = range_from_secs(secs);
            match view {
                EventExploreView::EventLog => {
                    let rows = query_events(EventQuery {
                        table: table.clone(),
                        start,
                        end,
                        partition: None,
                        pagination: GridPaginationModel::default(),
                        sort: vec![GridSortItem {
                            field: "ts".into(),
                            sort: GridSortDirection::Desc,
                        }],
                        filter: GridFilterModel::default(),
                    })
                    .await?;
                    Ok::<ExploreData, ServerFnError>(ExploreData::Rows(rows))
                }
                other => {
                    let agg = query_event_aggregate(EventAggregateRequest {
                        table: table.clone(),
                        start,
                        end,
                        partition: None,
                        filter: GridFilterModel {
                            items: Vec::new(),
                            ..GridFilterModel::default()
                        },
                        view: other,
                        aggregation: agg,
                    })
                    .await?;
                    Ok::<ExploreData, ServerFnError>(ExploreData::Aggregate(agg))
                }
            }
        },
    );

    view! {
        <EventToolbar
            range_secs=Signal::derive(move || range_secs.get())
            on_range=Callback::new(move |s| set_range_secs.set(s))
            view=Signal::derive(move || view.get())
            on_view=Callback::new(move |v| set_view.set(v))
            aggregation=aggregation
        />
        <Transition fallback=move || view! { <EventExploreSkeleton view=view.get() /> }>
            {move || match explore_res.get() {
                Some(Ok(ExploreData::Rows(rows))) => view! {
                    <EventExploreViewport
                        view=EventExploreView::EventLog
                        row_result=Some(rows)
                        aggregate_result=None
                    />
                }.into_any(),
                Some(Ok(ExploreData::Aggregate(agg))) => view! {
                    <EventExploreViewport
                        view=view.get()
                        row_result=None
                        aggregate_result=Some(agg)
                    />
                }.into_any(),
                Some(Err(e)) if server_fn_is_permission_denied(&e) => {
                    view! { <PermissionDeniedState /> }.into_any()
                }
                Some(Err(e)) => view! {
                    <MessageBar intent=MessageBarIntent::Error>{e.to_string()}</MessageBar>
                }.into_any(),
                None => view! { <EventExploreSkeleton view=view.get() /> }.into_any(),
            }}
        </Transition>
    }
}
