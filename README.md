# Spectra UF App

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Official Unified Field admin UI for Spectra (Leptos).

```toml
[dependencies]
spectra-app = { git = "https://github.com/deathbreakfast/spectra-uf-app", package = "spectra-app", branch = "main" }
```

Mount Spectra admin routes from your host shell (SSR + hydrate features as required by your Leptos setup).

## Audience

| Reader | Use this repo for |
|--------|-------------------|
| **Host integrators** | Mounting Spectra admin routes in a Leptos SSR/hydrate shell |
| **UF platform authors** | Shared uf-app registry patterns used across PBCQ admin UIs |
| **Spectra operators** | Admin UI for topics, subscriptions, and Spectra ops views |

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
