use leptos::prelude::*;
use orbital::components::{AutoGrid, EmptyState, SpacingSize};
use orbital::primitives::{MessageBar, MessageBarIntent};

use crate::components::schema::SchemaCard;
use crate::server::list_schema_metadata;

#[component]
pub fn SchemaIndexSection(
    /// Reactive signal for the search query text.
    #[prop(into)]
    query: Signal<String>,
) -> impl IntoView {
    let schemas_res = Resource::new(|| (), |()| async move { list_schema_metadata().await });

    view! {
        <Suspense fallback=|| view! { "Loading schemas…" }>
            {move || {
                let q = query.get().to_lowercase();
                match schemas_res.get() {
                    Some(Ok(list)) => {
                        let filtered: Vec<_> = list
                            .into_iter()
                            .filter(|s| {
                                q.is_empty() || s.table_or_metric.to_lowercase().contains(&q)
                            })
                            .collect();
                        if filtered.is_empty() {
                            if q.is_empty() {
                                view! {
                                    <EmptyState message="No schemas registered" />
                                }
                                .into_any()
                            } else {
                                view! {
                                    <EmptyState
                                        message="No schemas match your search"
                                        description="Try a different search term."
                                    />
                                }
                                .into_any()
                            }
                        } else {
                            view! {
                                <AutoGrid min=Signal::derive(|| "270px".to_string())>
                                    {filtered.into_iter().map(|item| view! {
                                        <SchemaCard item=item />
                                    }).collect_view()}
                                </AutoGrid>
                            }
                            .into_any()
                        }
                    }
                    Some(Err(_)) => view! {
                        <MessageBar intent=MessageBarIntent::Error>
                            "Failed to load schemas."
                        </MessageBar>
                    }
                    .into_any(),
                    None => view! { <p>"Loading…"</p> }.into_any(),
                }
            }}
        </Suspense>
    }
}
