use leptos::prelude::*;
use spectra_core::EventQueryResult;

use crate::components::tables::SpectraEventDataGrid;

#[component]
pub fn EventLogPanel(
    /// Result data to render.
    result: EventQueryResult,
) -> impl IntoView {
    view! { <SpectraEventDataGrid result=result /> }
}
