use leptos::prelude::*;
use leptos_router::components::A;
use orbital::components::{Card, SpacingSize, Title3};
use orbital::primitives::Flex;
use spectra_backend::{
    spectra_metric_explore_path, spectra_schema_explore_path, spectra_schema_path,
};
use spectra_core::SchemaListItem;

use super::KindBadge;

#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn SchemaCard(
    /// Item to render.
    item: SchemaListItem,
) -> impl IntoView {
    let name = item.table_or_metric.clone();
    let kind = item.logging_kind;
    let href = if kind == "metric" {
        spectra_metric_explore_path(&name)
    } else {
        spectra_schema_explore_path(&name)
    };
    let detail_href = spectra_schema_path(&name);
    let test_id = format!("spectra-schema-card-{name}");
    view! {
        <div data-testid=test_id>
            <Card>
                <Flex vertical=true gap=SpacingSize::Size80.flex_gap()>
                    <Title3>{name.clone()}</Title3>
                    <KindBadge kind=kind />
                    <A href=detail_href>"Details"</A>
                    <A href=href>"Explore"</A>
                </Flex>
            </Card>
        </div>
    }
}
