use leptos::prelude::*;
use orbital_charts::PieChart;

use super::chart_surface_material::ChartSurfaceMaterial;

#[component]
pub fn EventPieChart() -> impl IntoView {
    view! {
        <div data-testid="spectra-event-pie-chart">
            <ChartSurfaceMaterial>
                <PieChart />
            </ChartSurfaceMaterial>
        </div>
    }
}
