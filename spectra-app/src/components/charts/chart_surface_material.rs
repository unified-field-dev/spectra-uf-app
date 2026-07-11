use leptos::prelude::*;
use orbital::components::Card;

#[component]
pub fn ChartSurfaceMaterial(children: Children) -> impl IntoView {
    view! {
        <Card>
            {children()}
        </Card>
    }
}
