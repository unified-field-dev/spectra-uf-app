use leptos::prelude::*;
use orbital_datatable::{
    DataTable, DataTableEmptyView, DataTableEvents, DataTableFeatures, DataTableFilter,
    DataTableFooterSlot, DataTableNoResultsView, PagingMode,
};
use spectra_core::EventQueryResult;

use super::event_grid_mapper::{to_column_defs, to_row_models};

#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn SpectraEventDataGrid(
    /// Event log query result (rows + columns). When `None`, `column_fields` supplies columns.
    #[prop(default = None)]
    result: Option<EventQueryResult>,
    /// Column field/header pairs when `result` is `None`.
    #[prop(default = Vec::new())]
    column_fields: Vec<(String, String)>,
    /// Controlled filter model shared with explore queries.
    filter: RwSignal<Option<DataTableFilter>>,
    /// Fires when the operator changes structured filters (header or Filters panel).
    on_filter_change: Callback<DataTableFilter>,
    /// When true, hide rows/footer; chart is rendered by the parent under this grid.
    #[prop(default = false)]
    chart_mode: bool,
) -> impl IntoView {
    let columns = if let Some(ref result) = result {
        to_column_defs(&result.columns)
    } else {
        column_fields
            .into_iter()
            .map(|(field, header)| orbital_datatable::DataTableColumnDef::new(field, header))
            .collect()
    };

    let row_models = if chart_mode {
        Vec::new()
    } else if let Some(ref result) = result {
        to_row_models(&result.rows, &result.columns)
    } else {
        Vec::new()
    };
    let items = RwSignal::new(row_models);

    let features = if chart_mode {
        DataTableFeatures::empty()
    } else {
        DataTableFeatures::HEADER_FILTERS
    };

    let events = DataTableEvents {
        on_filter_change: Some(Callback::new(move |f: DataTableFilter| {
            filter.set(Some(f.clone()));
            on_filter_change.run(f);
        })),
        ..Default::default()
    };

    if chart_mode {
        view! {
            <div data-testid="spectra-event-data-grid">
                <DataTable
                    columns=columns
                    items=items
                    sortable=true
                    features=features
                    filter=Signal::derive(move || filter.get())
                    data_table_events=events
                    paging=PagingMode::None
                >
                    <DataTableFooterSlot slot>
                        <span></span>
                    </DataTableFooterSlot>
                    <DataTableEmptyView slot>
                        <div style="display:none" aria-hidden="true"></div>
                    </DataTableEmptyView>
                    <DataTableNoResultsView slot>
                        <div style="display:none" aria-hidden="true"></div>
                    </DataTableNoResultsView>
                </DataTable>
            </div>
        }
        .into_any()
    } else {
        view! {
            <div data-testid="spectra-event-data-grid">
                <DataTable
                    columns=columns
                    items=items
                    sortable=true
                    features=features
                    filter=Signal::derive(move || filter.get())
                    data_table_events=events
                    paging=PagingMode::Paged
                />
            </div>
        }
        .into_any()
    }
}
