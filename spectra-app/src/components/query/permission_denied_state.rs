use leptos::prelude::*;
use orbital::components::Body1;

#[component]
pub fn PermissionDeniedState() -> impl IntoView {
    view! {
        <Body1>"You do not have permission to query this table."</Body1>
    }
}
