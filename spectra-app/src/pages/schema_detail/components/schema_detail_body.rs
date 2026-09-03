use leptos::prelude::*;
use orbital::components::{Body1, Card, EmptyState};
use orbital::primitives::{Flex, MessageBar, MessageBarIntent};

use crate::server::get_schema_metadata;

use super::quick_actions_card::QuickActionsCard;

#[component]
pub fn SchemaDetailBody(
    /// Reactive signal for the display name.
    #[prop(into)]
    name: Memo<String>,
) -> impl IntoView {
    let detail_res = Resource::new(
        move || name.get(),
        |n| async move { get_schema_metadata(n).await },
    );

    view! {
        <Suspense fallback=|| view! { "Loading…" }>
            {move || match detail_res.get() {
                Some(Ok(Some(d))) => {
                    let table_or_metric = d.table_or_metric.clone();
                    let logging_kind = d.logging_kind.clone();
                    let logging_kind_display = logging_kind.clone();
                    view! {
                        <div id="spectra-detail-meta">
                            <Card>
                                <Flex vertical=true>
                                    <Body1>{d.description.unwrap_or_else(|| "No description".into())}</Body1>
                                    <Body1>{format!("Kind: {logging_kind_display}")}</Body1>
                                </Flex>
                            </Card>
                        </div>
                        <QuickActionsCard name=table_or_metric kind=logging_kind />
                    }.into_any()
                }
                Some(Ok(None)) => view! {
                    <EmptyState
                        message="Schema not found"
                        description="No schema is registered with that name."
                    />
                }.into_any(),
                Some(Err(_)) => view! {
                    <MessageBar intent=MessageBarIntent::Error>"Error loading schema."</MessageBar>
                }.into_any(),
                None => view! { <p>"Loading…"</p> }.into_any(),
            }}
        </Suspense>
    }
}
