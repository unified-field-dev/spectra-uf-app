use leptos::prelude::*;
use orbital::components::{ContentContainer, SpacingSize, Title3};
use orbital::primitives::{Flex, SearchBox, SearchBoxAppearance, SearchBoxBind};

use self::components::SchemaIndexSection;

mod components;

/// Schema index: browsable list of all registered event/metric schemas.
#[component]
pub fn SchemaIndexPage() -> impl IntoView {
    let query = RwSignal::new(String::new());
    view! {
        <ContentContainer data_testid="schema-index-page">
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap()>
                <Title3>"Schemas"</Title3>
                <SearchBox
                    bind=SearchBoxBind::from(query)
                    appearance=SearchBoxAppearance::with_placeholder("Search schemas…")
                />
                <SchemaIndexSection query=query />
            </Flex>
        </ContentContainer>
    }
}
