//! Spotlight steps for metric explore (`/spectra/metric/:name/explore`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Centered intro for the metric explorer.
#[help_spotlight_step(
    route = "/spectra/metric/:name/explore",
    feature_highlight = "spectra-metric-intro",
    title = "Explore a metric",
    order = 10
)]
#[component]
pub fn SpectraMetricIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-spectra-metric-intro",
        "Metrics are numbers over time. This page has a simpler toolbar than the event explorer: pick a window, then read the headline cards and chart.",
        None,
        &[],
    )
}

/// Metric time range.
#[help_spotlight_step(
    route = "/spectra/metric/:name/explore",
    feature_highlight = "spectra-metric-time-range",
    title = "Pick a time window",
    spotlight = "spectra-metric-time-range",
    position = "bottom",
    order = 20
)]
#[component]
pub fn SpectraMetricTimeRangeHelp() -> impl IntoView {
    help_stack(
        "help-step-spectra-metric-time-range",
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

/// Headline stats + chart.
#[help_spotlight_step(
    route = "/spectra/metric/:name/explore",
    feature_highlight = "spectra-metric-results",
    title = "Headline and chart",
    spotlight = "spectra-metric-results",
    position = "top",
    order = 30
)]
#[component]
pub fn SpectraMetricResultsHelp() -> impl IntoView {
    help_stack(
        "help-step-spectra-metric-results",
        "Headline cards show summary label and value pairs. The chart plots the series over the window you chose.",
        Some("Use both to spot peaks, drops, and the overall trend."),
        &[],
    )
}

/// Left navigation on metric explore.
#[help_spotlight_step(
    route = "/spectra/metric/:name/explore",
    feature_highlight = "spectra-metric-nav",
    title = "Finding your way",
    spotlight = "spectra-nav",
    position = "right",
    order = 40
)]
#[component]
pub fn SpectraMetricNavHelp() -> impl IntoView {
    help_stack(
        "help-step-spectra-metric-nav",
        "Use the left menu to leave explore and open Home or Schemas.",
        Some("Replay anytime: Help → Replay this route."),
        &[],
    )
}
