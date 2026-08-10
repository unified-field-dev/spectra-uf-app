# spectra-app

Leptos operations UI for Spectra: schema browsing and event/metric explore under
`/spectra`.

```toml
# Pin tag or rev — do not use branch = "main".
spectra-app = { git = "https://github.com/unified-field-dev/spectra-uf-app", package = "spectra-app", rev = "REPLACE_WITH_PIN", default-features = false }
```

```rust,ignore
use spectra_app::SpectraRoutes;
use leptos_router::components::Routes;

view! {
    <Routes fallback=|| "not found">
        <SpectraRoutes />
    </Routes>
}
```

Crate-root rustdoc owns Organized-by-task, Owns / does not own, the route table,
and the Examples ladder. Mapping helpers live in `spectra-backend`.

Compose into a host that supplies Spectra query backends and the auth/context
extractors the app expects. Enable `ssr` / hydrate to match your host.
