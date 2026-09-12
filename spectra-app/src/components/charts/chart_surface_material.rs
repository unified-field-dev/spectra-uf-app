use leptos::prelude::*;
use orbital::components::{Card, CardContent};

#[component]
pub fn ChartSurfaceMaterial(
    /// Child content rendered inside the component.
    children: Children,
) -> impl IntoView {
    view! {
        <Card>
            <CardContent>
                {children()}
            </CardContent>
        </Card>
    }
}
