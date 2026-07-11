mod components;

use leptos::prelude::*;
use orbital::components::{ContentContainer, SpacingSize};
use orbital::primitives::{Flex, Input, InputAppearance};

use self::components::SchemaIndexSection;

#[component]
pub fn SchemaIndexPage() -> impl IntoView {
    let query = RwSignal::new(String::new());
    view! {
        <ContentContainer data_testid="schema-index-page">
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap()>
                <Input bind=query appearance=InputAppearance::with_placeholder("Search schemas…") />
                <SchemaIndexSection query=query />
            </Flex>
        </ContentContainer>
    }
}
