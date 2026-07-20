use leptos::prelude::*;
use orbital::components::Card;
use orbital::primitives::Flex;

#[component]
pub fn QueryToolbarMaterial(
    /// Child content rendered inside the component.
    children: Children,
) -> impl IntoView {
    view! {
        <Card>
            <Flex vertical=true>
                {children()}
            </Flex>
        </Card>
    }
}
