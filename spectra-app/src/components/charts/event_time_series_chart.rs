use leptos::prelude::*;
use orbital_charts::LineChart;

use super::chart_surface_material::ChartSurfaceMaterial;

#[component]
pub fn EventTimeSeriesChart() -> impl IntoView {
    view! {
        <div data-testid="spectra-event-time-series-chart">
            <ChartSurfaceMaterial>
                <LineChart />
            </ChartSurfaceMaterial>
        </div>
    }
}
