use leptos::prelude::*;
use leptos_router::components::A;
use orbital::components::{Card, Title3};
use orbital::primitives::{Button, ButtonAppearance, Flex};

#[component]
pub fn QuickActionsCard(
    /// Display name.
    name: String,
    /// Kind or category.
    kind: String,
) -> impl IntoView {
    let href = if kind == "metric" {
        format!("/spectra/metric/{name}/explore")
    } else {
        format!("/spectra/schema/{name}/explore")
    };
    view! {
        <Card>
            <Flex vertical=true>
                <Title3>"Explore"</Title3>
                <A href=href>
                    <Button appearance=ButtonAppearance::Primary>
                        "Open explore"
                    </Button>
                </A>
            </Flex>
        </Card>
    }
}
