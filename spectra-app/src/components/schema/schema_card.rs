use leptos::prelude::*;
use leptos_router::components::A;
use orbital::components::{Card, Title3};
use orbital::primitives::Flex;
use spectra_core::SchemaListItem;

use super::KindBadge;

#[component]
pub fn SchemaCard(
    /// Item to render.
    item: SchemaListItem,
) -> impl IntoView {
    let name = item.table_or_metric.clone();
    let kind = item.logging_kind.clone();
    let href = if kind == "metric" {
        format!("/spectra/metric/{name}/explore")
    } else {
        format!("/spectra/schema/{name}/explore")
    };
    let detail_href = format!("/spectra/schema/{name}");
    let test_id = format!("spectra-schema-card-{}", name);
    view! {
        <div data-testid=test_id>
            <Card>
                <Flex vertical=true>
                    <Title3>{name.clone()}</Title3>
                    <KindBadge kind=kind />
                    <A href=detail_href>"Details"</A>
                    <A href=href>"Explore"</A>
                </Flex>
            </Card>
        </div>
    }
}
