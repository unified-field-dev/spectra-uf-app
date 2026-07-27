# protected-spectra-host

Axum oneshot host under **`/spectra`**: deny without session, allow with `X-Demo-User`, return the schema catalog `spectra-backend` exposes for the UI index screen.

Production Leptos hosts mount `<SpectraRoutes />` (auth-gated). This example proves the same path + auth + schema-index contract without the full SSR/WASM graph.

| | |
|---|---|
| **When to use** | First smoke of Spectra UF app host wiring (auth gate + schema index API) |
| **Command** | `CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=target-spectra-uf-app cargo run -p protected-spectra-host` |
| **Success** | Stdout: `protected_spectra_host: OK — /spectra deny/allow + schema index` |
| **Look next** | Mount [`SpectraRoutes`](../../spectra-app/) in a product host; wire Spectra query backends |

**Open first:** [`src/main.rs`](src/main.rs)

Compile-check:

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-spectra-uf-app
cargo check -p protected-spectra-host
```
