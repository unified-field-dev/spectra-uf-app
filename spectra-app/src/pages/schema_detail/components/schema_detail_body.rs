use leptos::prelude::*;
use orbital::components::{Body1, Card};
use orbital::primitives::Flex;

use crate::server::get_schema_metadata;

use super::quick_actions_card::QuickActionsCard;

#[component]
pub fn SchemaDetailBody(#[prop(into)] name: Memo<String>) -> impl IntoView {
    let detail_res = Resource::new(move || name.get(), |n| async move { get_schema_metadata(n).await });

    view! {
        <Suspense fallback=|| view! { "Loading…" }>
            {move || match detail_res.get() {
                Some(Ok(Some(d))) => {
                    let name = d.table_or_metric.clone();
                    let kind = d.logging_kind.clone();
                    view! {
                        <Card>
                            <Flex vertical=true>
                                <Body1>{d.description.unwrap_or_else(|| "No description".into())}</Body1>
                                <Body1>{format!("Kind: {}", d.logging_kind)}</Body1>
                            </Flex>
                        </Card>
                        <QuickActionsCard name=name kind=kind />
                    }.into_any()
                }
                Some(Ok(None)) => view! { <p>"Schema not found."</p> }.into_any(),
                Some(Err(_)) => view! { <p>"Error loading schema."</p> }.into_any(),
                None => view! { <p>"Loading…"</p> }.into_any(),
            }}
        </Suspense>
    }
}
