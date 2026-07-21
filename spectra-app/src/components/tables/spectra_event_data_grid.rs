use leptos::prelude::*;
use orbital_datatable::DataTable;
use spectra_core::EventQueryResult;

use super::event_grid_mapper::{to_column_defs, to_row_models};

#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn SpectraEventDataGrid(
    /// Result data to render.
    result: EventQueryResult,
) -> impl IntoView {
    let columns = to_column_defs(&result.columns);
    let items = RwSignal::new(to_row_models(&result.rows, &result.columns));
    view! {
        <div data-testid="spectra-event-data-grid">
            <DataTable columns=columns items=items sortable=true />
        </div>
    }
}
