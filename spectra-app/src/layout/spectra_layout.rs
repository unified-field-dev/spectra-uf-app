//! Spectra app shell inside platform layout.

use leptos::prelude::*;
use leptos_router::components::Outlet;
use orbital::components::{
    Caption1,
    Navigation, NavigationBody, NavigationConfig, NavigationLink, NavigationMaterial,
};
use orbital::primitives::Flex;
use uf_integrations::{
    ShellAppBar, ShellLeftNav, UnifiedFieldAppBar, UnifiedFieldShellLayout,
};

use crate::AppMetadata;

#[component]
pub fn SpectraLayout() -> impl IntoView {
    let app_name = AppMetadata::name().to_string();
    let selected_value = RwSignal::new(None::<String>);
    let open_categories = RwSignal::new(Vec::<String>::new());

    view! {
        <div data-testid="spectra-app-root">
            <UnifiedFieldShellLayout>
                <ShellAppBar slot>
                    <UnifiedFieldAppBar
                        app_name=app_name
                        app_id=AppMetadata::id()
                        homepage_url="/".to_string()
                    />
                </ShellAppBar>
                <ShellLeftNav slot>
                    <Navigation config=NavigationConfig::new().with_selected_value(selected_value).with_open_categories(open_categories)>
                        <NavigationMaterial slot />
                        <NavigationBody slot>
                            <NavigationLink path="/spectra/schema" value="/spectra/schema" icon=icondata::AiDatabaseOutlined test_id="nav-spectra-schemas">"Schemas"</NavigationLink>
                        </NavigationBody>
                    </Navigation>
                </ShellLeftNav>
                <Flex vertical=true>
                    <div data-testid="spectra-breadcrumbs">
                        <Caption1>"Spectra"</Caption1>
                    </div>
                    <Outlet />
                </Flex>
            </UnifiedFieldShellLayout>
        </div>
    }
}
