# Spectra UF App

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Official Unified Field admin UI for Spectra (Leptos).

```toml
[dependencies]
spectra-app = { git = "https://github.com/deathbreakfast/spectra-uf-app", package = "spectra-app", branch = "main" }
```

Mount Spectra admin routes from your host shell (SSR + hydrate features as required by your Leptos setup).

## Workspace

| Crate | Role |
|-------|------|
| `spectra-app` | Spectra admin UI |
| `uf-*` | Thin shell / registry helpers shared with other uf-app repos |

## Verify

```bash
cargo check --workspace
cargo check -p spectra-app --features ssr
```

## License

MIT. See [LICENSE](LICENSE), [CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md), and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
