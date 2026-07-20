use leptos::prelude::*;
use orbital::components::Card;

#[component]
pub fn ChartSurfaceMaterial(
    /// Child content rendered inside the component.
    children: Children,
) -> impl IntoView {
    view! {
        <Card>
            {children()}
        </Card>
    }
}
