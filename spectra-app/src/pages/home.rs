use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::{hooks::use_navigate, NavigateOptions};
use orbital::components::{Caption1, ContentContainer, EmptyState, SpacingSize, StatCard, Title3};
use orbital::primitives::{
    Body1, Button, ButtonAppearance, Card, CardContent, CardHeader, Flex, FlexWrap, SearchBox,
    SearchBoxAppearance, SearchBoxBind,
};

use crate::components::schema::SchemaCard;
use crate::server::{get_spectra_dashboard_summary, SpectraDashboardSummary};
use spectra_backend::{
    spectra_schema_explore_path, spectra_schema_path, validate_spectra_query_name,
};

/// Spectra home dashboard: catalog stats, recent schemas, and quick open.
#[component]
pub fn SpectraHomePage() -> impl IntoView {
    let summary_res = Resource::new(|| (), |_| get_spectra_dashboard_summary());

    view! {
        <ContentContainer data_testid="spectra-home-page">
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap()>
                <Title3>"Spectra"</Title3>
                <Caption1>"Inspect schemas, event logs, and metrics"</Caption1>
                <Suspense fallback=|| view! { <Body1>"Loading dashboard…"</Body1> }>
                    {move || match summary_res.get() {
                        Some(Ok(summary)) if summary.schema_count == 0 => view! {
                            <EmptyState
                                message="No schemas registered"
                                description="Register Spectra schemas in the host to browse them here."
                            />
                        }.into_any(),
                        Some(Ok(summary)) => view! { <DashboardBody summary=summary /> }.into_any(),
                        Some(Err(e)) => view! {
                            <Body1>{format!("Failed to load dashboard: {e}")}</Body1>
                        }.into_any(),
                        None => view! { <Body1>"Loading…"</Body1> }.into_any(),
                    }}
                </Suspense>
            </Flex>
        </ContentContainer>
    }
}

#[component]
fn DashboardBody(
    /// Dashboard summary payload.
    summary: SpectraDashboardSummary,
) -> impl IntoView {
    let activity = summary.activity_24h_event_rows;
    view! {
        <Flex gap=SpacingSize::Size160.flex_gap() wrap=FlexWrap::Wrap>
            <StatCard label="Schemas" value=Signal::derive(move || summary.schema_count.to_string()) />
            <StatCard label="Event tables" value=Signal::derive(move || summary.event_table_count.to_string()) />
            <StatCard label="Metrics" value=Signal::derive(move || summary.metric_count.to_string()) />
            {activity.map(|n| {
                let value = Signal::derive(move || n.to_string());
                view! { <StatCard label="24h event rows" value=value /> }
            })}
        </Flex>
        <Flex vertical=true gap=SpacingSize::Size240.flex_gap()>
            <Card>
                <CardHeader>
                    <Title3>"Recent schemas"</Title3>
                </CardHeader>
                <CardContent>
                    <Flex vertical=true gap=SpacingSize::Size160.flex_gap()>
                        {summary.recent_schemas.into_iter().map(|item| view! {
                            <SchemaCard item=item />
                        }).collect_view()}
                        <A href="/spectra/schema">"View all schemas →"</A>
                    </Flex>
                </CardContent>
            </Card>
            <QuickOpenCard />
        </Flex>
    }
}

#[component]
fn QuickOpenCard() -> impl IntoView {
    let query = RwSignal::new(String::new());
    let navigate = use_navigate();
    let open_detail = {
        let navigate = navigate.clone();
        move |_| {
            let name = query.get().trim().to_string();
            if validate_spectra_query_name(&name).is_ok() {
                let path = spectra_schema_path(&name);
                navigate(&path, NavigateOptions::default());
            }
        }
    };
    let open_explore = move |_| {
        let name = query.get().trim().to_string();
        if validate_spectra_query_name(&name).is_ok() {
            let path = spectra_schema_explore_path(&name);
            navigate(&path, NavigateOptions::default());
        }
    };
    view! {
        <div data-testid="spectra-home-quick-open">
            <Card>
            <CardHeader>
                <Title3>"Quick open"</Title3>
            </CardHeader>
            <CardContent>
                <Flex vertical=true gap=SpacingSize::Size160.flex_gap()>
                    <SearchBox
                        bind=SearchBoxBind::from(query)
                        appearance=SearchBoxAppearance::with_placeholder("Search schema name…")
                    />
                    <Flex gap=SpacingSize::Size80.flex_gap()>
                        <Button appearance=ButtonAppearance::Secondary on:click=open_detail>"Open detail"</Button>
                        <Button appearance=ButtonAppearance::Primary on:click=open_explore>"Open explore"</Button>
                    </Flex>
                </Flex>
            </CardContent>
            </Card>
        </div>
    }
}
