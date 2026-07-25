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

Host must supply Spectra query backends and auth guard context. Enable `ssr` / hydrate features to match your host.

## Workspace

| Crate | Role |
|-------|------|
| `spectra-app` | Spectra admin UI |
| `uf-*` | Thin shell / registry helpers shared with other uf-app repos |

## Verify

```bash
export CARGO_BUILD_JOBS=1
cargo check --workspace
cargo check -p spectra-app --features ssr
cargo test -p spectra-app --features ssr
```

## License

MIT. See [LICENSE](LICENSE), [CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md), and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
