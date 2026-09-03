use leptos::prelude::*;
use leptos_router::components::A;
use orbital::components::{Card, Title3};
use orbital::primitives::{Button, ButtonAppearance, Flex};
use spectra_backend::{spectra_metric_explore_path, spectra_schema_explore_path};

#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn QuickActionsCard(
    /// Display name.
    name: String,
    /// Kind or category.
    kind: String,
) -> impl IntoView {
    let href = if kind == "metric" {
        spectra_metric_explore_path(&name)
    } else {
        spectra_schema_explore_path(&name)
    };
    view! {
        <Card>
            <Flex vertical=true>
                <Title3>"Explore"</Title3>
                <A href=href>
                    <span id="spectra-detail-open-explore" data-testid="spectra-detail-open-explore">
                        <Button appearance=ButtonAppearance::Primary>
                            "Open explore"
                        </Button>
                    </span>
                </A>
            </Flex>
        </Card>
    }
}
