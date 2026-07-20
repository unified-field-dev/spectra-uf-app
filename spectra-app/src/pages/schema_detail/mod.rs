mod components;

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use orbital::components::{ContentContainer, SpacingSize, Title3};
use orbital::primitives::Flex;

use self::components::SchemaDetailBody;

/// Detail view for a single schema: fields, kind, and quick actions to explore its data.
#[component]
pub fn SchemaDetailPage() -> impl IntoView {
    let params = use_params_map();
    let name = Memo::new(move |_| {
        params.with(|p| p.get("name").map(|s| s.to_string()).unwrap_or_default())
    });
    view! {
        <ContentContainer>
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap()>
                <Title3>{move || name.get()}</Title3>
                <SchemaDetailBody name=name />
            </Flex>
        </ContentContainer>
    }
}
