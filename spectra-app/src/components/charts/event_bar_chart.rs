use leptos::prelude::*;
use orbital_charts::BarChart;

use super::chart_surface_material::ChartSurfaceMaterial;

#[component]
pub fn EventBarChart() -> impl IntoView {
    view! {
        <ChartSurfaceMaterial>
            <BarChart />
        </ChartSurfaceMaterial>
    }
}
