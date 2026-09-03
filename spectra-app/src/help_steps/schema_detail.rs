//! Spotlight steps for schema detail (`/spectra/schema/:name`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Metadata card: description and kind.
#[help_spotlight_step(
    route = "/spectra/schema/:name",
    feature_highlight = "spectra-detail-meta",
    title = "About this schema",
    spotlight = "spectra-detail-meta",
    position = "bottom",
    order = 10
)]
#[component]
pub fn SpectraDetailMetaHelp() -> impl IntoView {
    help_stack(
        "help-step-spectra-detail-meta",
        "Read what this drawer holds before you explore the live data.",
        Some("If description is empty, the page says so."),
        &[
            "Description: what this data is about",
            "Kind: Event log or Metric",
        ],
    )
}

/// Open explore CTA.
#[help_spotlight_step(
    route = "/spectra/schema/:name",
    feature_highlight = "spectra-detail-open-explore",
    title = "Explore this data",
    spotlight = "spectra-detail-open-explore",
    position = "top",
    order = 20
)]
#[component]
pub fn SpectraDetailOpenExploreHelp() -> impl IntoView {
    help_stack(
        "help-step-spectra-detail-open-explore",
        "Opens the explorer for this schema. Event kinds open the event explorer; metrics open the metric explorer.",
        None,
        &[],
    )
}

/// Left navigation on schema detail.
#[help_spotlight_step(
    route = "/spectra/schema/:name",
    feature_highlight = "spectra-detail-nav",
    title = "Finding your way",
    spotlight = "spectra-nav",
    position = "right",
    order = 30
)]
#[component]
pub fn SpectraDetailNavHelp() -> impl IntoView {
    help_stack(
        "help-step-spectra-detail-nav",
        "Use the left menu to return to Home or the Schemas list.",
        Some("Replay anytime: Help → Replay this route."),
        &[],
    )
}
