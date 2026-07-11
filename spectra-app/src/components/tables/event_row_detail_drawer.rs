use leptos::prelude::*;
use orbital::components::{Body1, Title3};
use orbital::primitives::Button;
use spectra_core::EventGridRow;

#[component]
pub fn EventRowDetailDrawer(
    row: EventGridRow,
    #[prop(into)] open: Signal<bool>,
    on_close: Callback<()>,
) -> impl IntoView {
    let json = StoredValue::new(serde_json::to_string_pretty(&row.fields).unwrap_or_default());
    view! {
        <Show when=move || open.get()>
            <div class="spectra-row-drawer">
                <Title3>"Row detail"</Title3>
                <Body1>{json.get_value()}</Body1>
                <Button on:click=move |_| on_close.run(())>"Close"</Button>
            </div>
        </Show>
    }
}
