use leptos::prelude::*;
use orbital::components::AutoGrid;
use orbital::components::SpacingSize;

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
                            view! { <p>"No schemas match your search."</p> }.into_any()
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
                    Some(Err(_)) => view! { <p>"Failed to load schemas."</p> }.into_any(),
                    None => view! { <p>"Loading…"</p> }.into_any(),
                }
            }}
        </Suspense>
    }
}
