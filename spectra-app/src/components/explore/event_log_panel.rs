use leptos::prelude::*;
use spectra_core::EventQueryResult;

use crate::components::tables::SpectraEventDataGrid;

#[component]
pub fn EventLogPanel(result: EventQueryResult) -> impl IntoView {
    view! { <SpectraEventDataGrid result=result /> }
}
