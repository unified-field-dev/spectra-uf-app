//! Map Spectra query DTOs to orbital-charts series and axis definitions.

use orbital_charts::{AxisDef, AxisPosition, ChartType, ScaleType, SeriesDef};
use spectra_core::{SliceDto, TimeSeriesDto};

/// Builds x-axis categories and line/bar series from time-series DTOs.
#[must_use]
pub fn chart_from_time_series(series: &[TimeSeriesDto]) -> (Vec<AxisDef>, Vec<SeriesDef>) {
    if series.is_empty() {
        return (Vec::new(), Vec::new());
    }

    let categories: Vec<String> = series[0]
        .points
        .iter()
        .map(|p| p.ts.format("%H:%M").to_string())
        .collect();

    let x_axis = vec![AxisDef {
        id: "x".to_string(),
        scale_type: ScaleType::Band,
        data: Some(categories),
        position: AxisPosition::Bottom,
        ..Default::default()
    }];

    let chart_series: Vec<SeriesDef> = series
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let label = match &s.labels {
                serde_json::Value::String(name) if !name.is_empty() => name.clone(),
                _ => format!("series-{i}"),
            };
            SeriesDef {
                id: format!("series-{i}"),
                label: Some(label),
                chart_type: Some(ChartType::Line),
                data: Some(s.points.iter().map(|p| p.value).collect()),
                ..Default::default()
            }
        })
        .collect();

    (x_axis, chart_series)
}

/// Builds pie/bar chart series from slice DTOs.
#[must_use]
pub fn chart_from_slices(
    slices: &[SliceDto],
    chart_type: ChartType,
) -> (Vec<AxisDef>, Vec<SeriesDef>) {
    if slices.is_empty() {
        return (Vec::new(), Vec::new());
    }

    let labels: Vec<String> = slices.iter().map(|s| s.label.clone()).collect();
    let values: Vec<f64> = slices.iter().map(|s| s.value).collect();

    let x_axis = vec![AxisDef {
        id: "x".to_string(),
        scale_type: ScaleType::Band,
        data: Some(labels),
        position: AxisPosition::Bottom,
        ..Default::default()
    }];

    let series = vec![SeriesDef {
        id: "slices".to_string(),
        label: Some("Value".to_string()),
        chart_type: Some(chart_type),
        data: Some(values),
        ..Default::default()
    }];

    (x_axis, series)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use spectra_core::MetricPointDto;

    #[test]
    fn chart_from_time_series_maps_points_happy_path() {
        let now = Utc::now();
        let series = vec![TimeSeriesDto {
            labels: serde_json::json!({"name": "cpu"}),
            points: vec![
                MetricPointDto {
                    ts: now,
                    value: 1.0,
                },
                MetricPointDto {
                    ts: now + chrono::Duration::minutes(1),
                    value: 2.0,
                },
            ],
        }];
        let (x_axis, chart_series) = chart_from_time_series(&series);
        assert_eq!(x_axis.len(), 1);
        assert_eq!(x_axis[0].data.as_ref().map(|d| d.len()), Some(2));
        assert_eq!(chart_series.len(), 1);
        assert_eq!(chart_series[0].data.as_ref().map(|d| d.len()), Some(2));
    }

    #[test]
    fn chart_from_slices_maps_labels_happy_path() {
        let slices = vec![
            SliceDto {
                label: "a".into(),
                value: 3.0,
            },
            SliceDto {
                label: "b".into(),
                value: 7.0,
            },
        ];
        let (_, series) = chart_from_slices(&slices, ChartType::Pie);
        assert_eq!(series[0].data.as_ref().map(|d| d.len()), Some(2));
    }
}
