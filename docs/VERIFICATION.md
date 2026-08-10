# spectra-uf-app verification

Re-run after code or doc changes. This workspace is the Spectra operations app
(`spectra-app` Leptos UI + `spectra-backend` pure server contracts). Layer 1 unit +
integration tests cover schema catalog and explore-query stub helpers, plus
sibling-source UI surface contracts for `spectra-app`. No Leptos UI e2e, `*-e2e`
crate, or AWS campaign is required for this workspace. Spectra core owns
storage/query IsolatedLab; this repo verifies the UF app mapping layer.

## Environment

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-spectra-uf-app
```

## Teaching host (Pass 3 gate)

Axum oneshot under [`examples/protected-spectra-host`](../examples/protected-spectra-host/).
Copy table + product mount sketches live in that host README.

```bash
cargo check -p protected-spectra-host
cargo run -p protected-spectra-host
```

Success line: `protected_spectra_host: OK — /spectra deny/allow + schema index`.
Hydrate/browser is out of gate for the oneshot (`cargo-leptos` + `wasm32` +
Orbital / `uf-product` belong to a composite product host).

## Layer 1 — Unit + integration (CI)

GitHub Actions (`.github/workflows/ci.yml`) covers this Layer 1 subset plus the
teaching host and spectra-backend rustdoc gate below. It does not build
`spectra-app` (Leptos UI / SSR).

Sibling-source UI contracts (no Orbital / `spectra-app` compile):

```bash
cargo test -p spectra-backend --test workspace_members --test product_surface
```

Backend contracts (preferred path; no UI graph):

```bash
cargo fmt -p spectra-backend -p spectra-app -p protected-spectra-host -- --check
cargo clippy -p spectra-backend --all-targets -- -D warnings
cargo clippy -p protected-spectra-host --all-targets -- -D warnings
cargo test -p spectra-backend
```

`cargo fmt --all` can fail in this monorepo checkout when a path-patched
member sits outside that workspace; package-scoped fmt is the honest local gate.

Full workspace (includes `spectra-app` UI). May fail when the path-patched
`uf-product` / `uf-integrations` UI graph is broken upstream — that is a
pre-existing host-product UI compile issue, not a Spectra backend contract gap.
Surface needles for routes, nav testids, `RequireAuthenticated`, and
`QueryTable` live in `product_surface`.

```bash
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
# Host-aligned SSR surface (when UI graph compiles):
cargo test -p spectra-app --features ssr
```

## Layer 2 — E2E

**Waived.** Schema list/detail and explore-query stub shapes are exercised by Layer 1
integration tests named below. A Leptos/UI browser suite or IsolatedLab `*-e2e` crate
is out of scope for this backend-first remediation; live query backends belong in the
product host / Spectra core workspace.

Covering integ tests for the e2e waiver:

- `schema_metadata_list_returns_valid_items_happy_path` / `schema_metadata_detail_unknown_name_is_none_sad`
- `schema_metadata_detail_matches_list_entry_happy_path`
- `empty_metrics_query_result_shape_happy_path` / `empty_event_query_result_unknown_table_happy_path`
- `empty_event_aggregate_result_timeseries_stub_happy_path`
- `validate_spectra_query_name_accepts_table_happy_path` / `validate_spectra_query_name_rejects_blank_sad`
- `spectra_query_permission_name_formats_table_happy_path`
- `spectra_product_workspace_members_happy_path`
- `spectra_routes_mount_happy_path` / `layout_auth_gate_and_nav_happy_path` / `ops_reads_require_query_table_happy_path`

## Layer 3 — AWS campaigns + performance

**Waived.** This application workspace; no cloud resources or Criterion benches.
Correctness is in-process against the Spectra schema registry and stub query
payloads only.

## Rustdoc policy

Preferred deny gate (no UI graph):

```bash
RUSTDOCFLAGS="-D rustdoc::broken-intra-doc-links" cargo doc -p spectra-backend --no-deps
```

Workspace `rustdoc::broken_intra_doc_links` is `allow` in `Cargo.toml` because
sibling/cfg-gated links often fail under `--no-deps`. Prefer the
`RUSTDOCFLAGS` deny form above for the backend contract crate. `spectra-app`
rustdoc with deny flags is pin-dependent on Orbital / host graphs.
`spectra-app` still uses `#![allow(missing_docs)]` on macro-heavy UI surfaces.

## Notes

- Prefer `cargo test -p spectra-backend` for backend contract CI when the UI
  dependency graph (`uf-product` via `uf-integrations` / `lepton-shell`) fails to
  compile — report that separately from Spectra contract results.
- Tests may `unwrap`/`expect`; production server fns map failures to `ServerFnError`
  (no ordinary-path unwrap).
- Sad-path assertions check message content or `None` — stronger than `is_err()` alone.
- Happy-path tests are named `*_happy_path` so audits detect them.
- `SpectraRoutes` data loaders call the `#[server]` fns; those fns are thin Higgs
  wrappers over `spectra-backend` helpers covered by Layer 1.
