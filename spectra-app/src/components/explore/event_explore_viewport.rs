use leptos::prelude::*;
use orbital::primitives::{Flex, FlexGap};
use orbital_datatable::DataTableFilter;
use spectra_core::{EventAggregateResult, EventExploreView, EventQueryResult};

use crate::components::explore::{EventPiePanel, EventSeriesPanel};
use crate::components::query::normalize_view;
use crate::components::tables::SpectraEventDataGrid;

#[component]
pub fn EventExploreViewport(
    /// Applied view (after See / Show rows).
    view: EventExploreView,
    /// Event log rows when applied view is Event log.
    row_result: Option<EventQueryResult>,
    /// Aggregate payload for chart views.
    aggregate_result: Option<EventAggregateResult>,
    /// Group-by field for bar/pie EmptyState when unset.
    group_by_field: Option<String>,
    /// Column field/header pairs when chart mode has no row payload yet.
    column_fields: Vec<(String, String)>,
    /// Controlled DataTable filter.
    table_filter: RwSignal<Option<DataTableFilter>>,
    /// Bridge filter changes into the explore query model.
    on_filter_change: Callback<DataTableFilter>,
) -> impl IntoView {
    let view = normalize_view(view);
    let chart_mode = view != EventExploreView::EventLog;

    view! {
        <div id="spectra-event-explore-viewport" data-testid="spectra-event-explore-viewport">
            <Flex vertical=true gap=FlexGap::Small>
                <SpectraEventDataGrid
                    result=row_result.clone()
                    column_fields=column_fields
                    filter=table_filter
                    on_filter_change=on_filter_change
                    chart_mode=chart_mode
                />
                {match view {
                    EventExploreView::EventLog => ().into_any(),
                    EventExploreView::TimeSeries | EventExploreView::LineChart => {
                        aggregate_result
                            .map(|r| {
                                view! {
                                    <div data-testid="spectra-event-explore-chart-body">
                                        <EventSeriesPanel result=r />
                                    </div>
                                }
                                .into_any()
                            })
                            .unwrap_or_else(|| ().into_any())
                    }
                    EventExploreView::PieChart | EventExploreView::BarChart => {
                        let gb = group_by_field.clone();
                        aggregate_result
                            .map(|r| {
                                view! {
                                    <div data-testid="spectra-event-explore-chart-body">
                                        <EventPiePanel view=view result=r group_by_field=gb />
                                    </div>
                                }
                                .into_any()
                            })
                            .unwrap_or_else(|| ().into_any())
                    }
                }}
            </Flex>
        </div>
    }
}
