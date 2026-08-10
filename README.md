# Spectra UF App

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[GitHub](https://github.com/deathbreakfast/spectra-uf-app) · `cargo doc -p spectra-backend --open` · distributed via git (not crates.io)

## About

Spectra UF App is the Unified Field **operations UI** for Spectra schemas,
events, and metrics under `/spectra`. Spectra itself has no built-in UI; hosts
mount this crate so operators can browse registered schemas and explore
event/metric data.

- **UI (`spectra-app`)** — pages, Higgs `#[server]` wrappers, `SpectraRoutes`,
  `uf_app!` registration
- **Backend (`spectra-backend`)** — pure schema/query stub helpers (no Leptos);
  preferred Layer 1 CI path

Hosts supply Spectra query backends and auth guard context. Enable `ssr` /
hydrate to match your host. Crate-root rustdoc owns Concern → route → server fn
tables; prefer `cargo doc -p spectra-backend --open` for the mapping contract.
UI rustdoc is pin-dependent on Orbital / host graphs. Explore queries currently
return empty stub payloads until a host injects a live Spectra backend.

## Getting started

```toml
[dependencies]
# Pin tag or rev — do not use branch = "main".
spectra-app = { git = "https://github.com/deathbreakfast/spectra-uf-app", package = "spectra-app", rev = "REPLACE_WITH_PIN", default-features = false }
spectra-backend = { git = "https://github.com/deathbreakfast/spectra-uf-app", package = "spectra-backend", rev = "REPLACE_WITH_PIN" }
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

Wire Spectra query backends + session extractors in host bootstrap, then mount
the routes above. Full Leptos SSR hosts live outside this repository; use the
local teaching host for the auth + schema index contract.

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-spectra-uf-app
cargo test -p spectra-backend
```

## Workspace

| Crate | Role |
|-------|------|
| [`spectra-app`](spectra-app/) | Leptos ops UI + `SpectraRoutes` + app registration |
| [`spectra-backend`](spectra-backend/) | Pure schema catalog + explore-query stub helpers |
| [`protected-spectra-host`](examples/protected-spectra-host/) | Teaching host: deny/allow + schema index |

Top-level `uf-*` directories in this checkout (if present) are unused leftovers.
Real `uf-integrations` / `uf-product-macros` / `uf-ssr` / `uf-app-registry` pins
live in workspace `[workspace.dependencies]` (see `Cargo.toml`).

## Examples

| Host | When to use | Command | Success | Look next |
|------|-------------|---------|---------|-----------|
| [`protected-spectra-host`](examples/protected-spectra-host/) | Auth + `/spectra` schema index | `CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=target-spectra-uf-app cargo run -p protected-spectra-host` | Deny/allow + schema JSON | Mount `SpectraRoutes` |

Copy table + product mount `Cargo.toml`:
[`examples/protected-spectra-host/README.md`](examples/protected-spectra-host/README.md).
Full ladder: [`examples/README.md`](examples/README.md).

| Level | Where |
|-------|--------|
| Highlight | Mount snippet above; crate-root Getting started |
| Mid | `spectra-backend` unit + integ suites (see `docs/VERIFICATION.md`) |
| Detailed | `protected-spectra-host` (session gate + schema index; inventory `spectra` / `/spectra`) |

## Security

Auth-gated `/spectra` routes (`QueryTable` plus per-table
`spectra.query.{table}` for explore) and private vulnerability reporting:
[`SECURITY.md`](SECURITY.md). Report vulnerabilities privately — do not open a
public issue for security-sensitive reports.

## Verify

Local gates (fmt/clippy/CI workflow not claimed here):

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-spectra-uf-app
cargo check -p protected-spectra-host
cargo run -p protected-spectra-host
cargo clippy -p spectra-backend --all-targets -- -D warnings
cargo test -p spectra-backend
RUSTDOCFLAGS="-D rustdoc::broken-intra-doc-links" cargo doc -p spectra-backend --no-deps
```

Prefer `spectra-backend` for contract CI. Teaching host success line:
`protected_spectra_host: OK — /spectra deny/allow + schema index`.
`spectra-app` compile/doc can fail when the path-patched Orbital / host graph is
broken upstream — treat that as host-product debt, not a Spectra mapping gap.
Full command block: [`docs/VERIFICATION.md`](docs/VERIFICATION.md). Contribute:
[`CONTRIBUTING.md`](CONTRIBUTING.md).

## FAQ

**Is this a standalone Spectra server?** No. `spectra-app` mounts under a host
`<Routes>` tree. Schema registry and query storage live in the Spectra core
crates; hosts inject live query backends.

**Why is there a separate `spectra-backend` crate?** So schema catalog and
explore-query stub helpers stay unit-testable without the Leptos/UI dependency
graph. `spectra-app` `#[server]` fns are thin wrappers over those helpers.

**What can operators do from the UI?** Browse registered schemas and explore
events/metrics (table, time series, breakdowns). Explore payloads are empty stubs
until the host wires a live Spectra backend.

**Where does Spectra core fit?** Schema registration and storage live in the
Spectra core repos. This repo maps admin list/get/query APIs into UF ops pages.

## License

MIT. See [LICENSE](LICENSE), [CONTRIBUTING.md](CONTRIBUTING.md),
[SECURITY.md](SECURITY.md), and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
