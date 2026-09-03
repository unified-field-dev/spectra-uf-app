# protected-spectra-host

Axum oneshot host under **`/spectra`**: deny without session, allow with
`X-Demo-User`, return the schema catalog `spectra-backend` exposes for the UI
index screen.

Production Leptos hosts mount `SpectraRoutes` at **`/spectra`** and gate ops
reads with `QueryTable`. This example proves the same path + auth + schema-index
contract without the SSR/WASM / Orbital graph. The oneshot path `/spectra`
matches the Orbital app id/path (`spectra` / `/spectra`).

| | |
|---|---|
| **When to use** | First smoke of Spectra UF app host wiring (auth gate + schema index API) |
| **Command** | `CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=target-spectra-uf-app cargo run -p protected-spectra-host` |
| **Success** | Stdout: `protected_spectra_host: OK — /spectra deny/allow + schema index` |
| **Look next** | Mount [`SpectraRoutes`](../../spectra-app/) ; wire Spectra query backends |

**Open first:** [`src/main.rs`](src/main.rs)

## Copy into your host

| File | What to take |
|------|----------------|
| This [`Cargo.toml`](Cargo.toml) | Axum oneshot shape + `spectra-backend` (schema catalog smoke) |
| Product mount `Cargo.toml` (below) | `spectra-app` + `spectra-backend` with `ssr` / `hydrate` features |
| [`src/main.rs`](src/main.rs) | Session gate on `/spectra`, schema JSON, inventory contract names |
| Leptos sketch (below) | `<SpectraRoutes />` under `/spectra` |

### Product mount dependencies

```toml
[dependencies]
spectra-app = { git = "https://github.com/unified-field-dev/spectra-uf-app", package = "spectra-app", rev = "REPLACE_WITH_PIN", default-features = false }
spectra-backend = { git = "https://github.com/unified-field-dev/spectra-uf-app", package = "spectra-backend", rev = "REPLACE_WITH_PIN" }
uf-product = { /* your pin */, default-features = false }
uf-integrations = { /* your pin */, default-features = false }

[features]
ssr = [
    "spectra-app/ssr",
    "uf-product/ssr",
    "uf-integrations/ssr",
]
hydrate = [
    "spectra-app/hydrate",
    "uf-product/hydrate",
    "uf-integrations/hydrate",
]
```

### Leptos mount sketch

```rust,ignore
use spectra_app::SpectraRoutes;
use leptos_router::components::Routes;

view! {
    <Routes fallback=|| "not found">
        <SpectraRoutes />
    </Routes>
}
```

Catalog helpers (Leptos-free):

```rust,ignore
use spectra_backend::schema_metadata_list;

let schemas = schema_metadata_list();
```

Inventory names match `spectra` / `/spectra`. Layout uses `RequireAuthenticated`;
ops `#[server]` fns carry `QueryTable` (manifest
`permissions::SpectraPermission`). Explore queries also check Gauge
`spectra.query.{table}` via `require_spectra_query`. Wire Spectra query backends
+ session extractors in host bootstrap before mounting the routes.

For shell chrome (layout, fonts, Axum + Leptos boot), copy
[`shell-chrome-host`](https://github.com/unified-field-dev/unified-field-product/tree/main/examples/shell-chrome-host)
from unified-field-product, then mount `SpectraRoutes`.

## Run (documented gate)

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-spectra-uf-app
cargo check -p protected-spectra-host
cargo run -p protected-spectra-host
```

**Success:** stdout prints `protected_spectra_host: OK — /spectra deny/allow + schema index`.

## Hydrate / browser

Out of gate for this host. Full ops UI needs a product binary with
`cargo-leptos`, `wasm32`, session chrome, Spectra query backends, and a working
Orbital / `uf-product` graph. Prefer the oneshot above.
