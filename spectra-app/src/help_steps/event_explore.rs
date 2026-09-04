//! Spotlight steps for event explore (`/spectra/schema/:name/explore`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Centered intro for the event explorer.
#[help_spotlight_step(
    route = "/spectra/schema/:name/explore",
    feature_highlight = "spectra-event-intro",
    title = "Explore an event log",
    order = 10
)]
#[component]
pub fn SpectraEventIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-spectra-event-intro",
        "This page reads one event diary. You pick how far back to look, how to display the rows or charts, and (for charts) how to count and group the values.",
        None,
        &[],
    )
}

/// Time range presets.
#[help_spotlight_step(
    route = "/spectra/schema/:name/explore",
    feature_highlight = "spectra-event-time-range",
    title = "Pick a time window",
    spotlight = "spectra-event-time-range",
    position = "bottom",
    order = 20
)]
#[component]
pub fn SpectraEventTimeRangeHelp() -> impl IntoView {
    help_stack(
        "help-step-spectra-event-time-range",
        "Click a button to reload with that window.",
        None,
        &[
            "1h: last hour",
            "6h: last six hours",
            "24h: last day",
            "7d: last week",
        ],
    )
}

/// View picker.
#[help_spotlight_step(
    route = "/spectra/schema/:name/explore",
    feature_highlight = "spectra-event-view-picker",
    title = "Choose how to look",
    spotlight = "spectra-event-view-picker",
    position = "bottom",
    order = 30
)]
#[component]
pub fn SpectraEventViewPickerHelp() -> impl IntoView {
    help_stack(
        "help-step-spectra-event-view-picker",
        "Click one view; the results area updates.",
        None,
        &[
            "Event log: table of individual rows",
            "Time series: points over time",
            "Line chart: same idea, line style",
            "Bar chart: bars by group",
            "Pie chart: slices by group",
        ],
    )
}

/// Measure select (charts).
#[help_spotlight_step(
    route = "/spectra/schema/:name/explore",
    feature_highlight = "spectra-aggregation-measure",
    title = "Count or Sum",
    spotlight = "spectra-aggregation-measure",
    position = "top",
    order = 40
)]
#[component]
pub fn SpectraAggregationMeasureHelp() -> impl IntoView {
    help_stack(
        "help-step-spectra-aggregation-measure",
        "When you pick a chart view (not Event log), choose Count or Sum for the aggregate.",
        Some("On Event log this control stays empty—switch to a chart to use it."),
        &[],
    )
}

/// Time bucket input.
#[help_spotlight_step(
    route = "/spectra/schema/:name/explore",
    feature_highlight = "spectra-aggregation-bucket",
    title = "Time bucket",
    spotlight = "spectra-aggregation-bucket",
    position = "top",
    order = 50
)]
#[component]
pub fn SpectraAggregationBucketHelp() -> impl IntoView {
    help_stack(
        "help-step-spectra-aggregation-bucket",
        "On Time series or Line chart, set how many seconds each point covers.",
        Some("Leave Event log or pie/bar views alone—this field only applies to series and line."),
        &[],
    )
}

/// Group-by field.
#[help_spotlight_step(
    route = "/spectra/schema/:name/explore",
    feature_highlight = "spectra-aggregation-group-by",
    title = "Group by field",
    spotlight = "spectra-aggregation-group-by",
    position = "top",
    order = 60
)]
#[component]
pub fn SpectraAggregationGroupByHelp() -> impl IntoView {
    help_stack(
        "help-step-spectra-aggregation-group-by",
        "On Pie or Bar charts, type a field name to split slices or bars.",
        Some("Other views leave this empty."),
        &[],
    )
}

/// Results viewport.
#[help_spotlight_step(
    route = "/spectra/schema/:name/explore",
    feature_highlight = "spectra-event-viewport",
    title = "Results",
    spotlight = "spectra-event-explore-viewport",
    position = "top",
    order = 70
)]
#[component]
pub fn SpectraEventViewportHelp() -> impl IntoView {
    help_stack(
        "help-step-spectra-event-viewport",
        "Event log shows a sortable table of rows. Chart views show headline numbers plus the chart itself.",
        Some("Read the grid or series here after you change the window or view."),
        &[],
    )
}

/// Left navigation on event explore.
#[help_spotlight_step(
    route = "/spectra/schema/:name/explore",
    feature_highlight = "spectra-event-nav",
    title = "Finding your way",
    spotlight = "spectra-nav",
    position = "right",
    order = 80
)]
#[component]
pub fn SpectraEventNavHelp() -> impl IntoView {
    help_stack(
        "help-step-spectra-event-nav",
        "Use the left menu to leave explore and open Home or Schemas.",
        Some("Replay anytime: Help → Replay this route."),
        &[],
    )
}
