use leptos::prelude::*;
use orbital::primitives::{MessageBar, MessageBarIntent};

#[component]
pub fn PermissionDeniedState() -> impl IntoView {
    view! {
        <div data-testid="spectra-permission-denied">
            <MessageBar intent=MessageBarIntent::Warning>
                "You do not have permission to query this table."
            </MessageBar>
        </div>
    }
}
