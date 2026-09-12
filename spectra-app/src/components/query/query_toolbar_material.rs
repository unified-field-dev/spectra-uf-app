use leptos::prelude::*;
use orbital::components::{Card, CardContent};
use orbital::primitives::Flex;

#[component]
pub fn QueryToolbarMaterial(
    /// Child content rendered inside the component.
    children: Children,
) -> impl IntoView {
    view! {
        <Card>
            <CardContent>
                <Flex>
                    {children()}
                </Flex>
            </CardContent>
        </Card>
    }
}
