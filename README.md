# Spectra UF App

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Leptos admin UI for Spectra schemas, events, and metrics — mounted under `/spectra`.

```toml
[dependencies]
spectra-app = { git = "https://github.com/deathbreakfast/spectra-uf-app", package = "spectra-app", branch = "main" }
```

```rust
use spectra_app::SpectraRoutes;
use leptos_router::components::Routes;

view! {
    <Routes fallback=|| "not found">
        <SpectraRoutes />
    </Routes>
}
```

## About

- Schema index and detail for registered event/metric schemas
- Event explore (table, time series, breakdowns)
- Metric explore for a single series over time

Host must supply Spectra query backends and auth guard context. Enable `ssr` / hydrate features to match your host. See the `spectra-app` crate rustdocs for the full Concern → route → server fn table.

## Examples

| Host | When to use | Command | Success | Look next |
|------|-------------|---------|---------|-----------|
| [`protected-spectra-host`](examples/protected-spectra-host/) | Auth + `/spectra` schema index | `CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=target-spectra-uf-app cargo run -p protected-spectra-host` | Deny/allow + schema JSON | Product host with `SpectraRoutes` |

Full ladder: [`examples/README.md`](examples/README.md).

## Workspace

| Crate | Role |
|-------|------|
| `spectra-app` | Spectra admin UI |
| `spectra-backend` | Pure schema/query contracts for server fns (no UI deps) |
| `uf-*` (top-level `uf-app-registry`, `uf-integrations`, `uf-product-macros`, `uf-ssr`) | Not workspace members and not depended on — the workspace's real `uf-*` crates come from `L3-products-zones-hosts` (see `[workspace.dependencies]` in `Cargo.toml`). These local trees are unused leftovers; do not treat them as source of truth. |

## Verify

See [docs/VERIFICATION.md](docs/VERIFICATION.md) for the TEST_MAP and Layer 1–3 gates.

```bash
export CARGO_BUILD_JOBS=1
cargo test -p spectra-backend
cargo clippy -p spectra-backend --all-targets -- -D warnings
# Full UI surface (requires a compiling uf-product graph):
cargo check -p spectra-app --features ssr
cargo test -p spectra-app --features ssr
```

## License

MIT. See [LICENSE](LICENSE), [CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md), and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
