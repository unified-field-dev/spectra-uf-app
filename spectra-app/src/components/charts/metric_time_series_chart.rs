use leptos::prelude::*;
use orbital::components::Caption1;
use orbital_charts::LineChart;
use spectra_core::TimeSeriesDto;

use super::chart_series::chart_from_time_series;
use super::chart_surface_material::ChartSurfaceMaterial;

#[component]
pub fn MetricTimeSeriesChart(
    /// Metric time series to render.
    series: Vec<TimeSeriesDto>,
) -> impl IntoView {
    let (x_axis, chart_series) = chart_from_time_series(&series);
    view! {
        <div data-testid="spectra-metric-time-series-chart">
            <ChartSurfaceMaterial>
                <Caption1>"Time series"</Caption1>
                <LineChart x_axis=x_axis series=chart_series />
            </ChartSurfaceMaterial>
        </div>
    }
}
