use leptos::prelude::*;
use orbital::components::Caption1;
use orbital_charts::LineChart;

use super::chart_surface_material::ChartSurfaceMaterial;

#[component]
pub fn MetricTimeSeriesChart() -> impl IntoView {
    view! {
        <div data-testid="spectra-metric-time-series-chart">
            <ChartSurfaceMaterial>
                <Caption1>"Time series"</Caption1>
                <LineChart />
            </ChartSurfaceMaterial>
        </div>
    }
}
