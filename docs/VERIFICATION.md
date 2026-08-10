# spectra-uf-app verification

Re-run after code or doc changes. This workspace is the Spectra operations app
(`spectra-app` Leptos UI + `spectra-backend` pure server contracts). Layer 1 unit +
integration tests cover schema catalog and explore-query stub helpers. No Leptos UI
e2e, `*-e2e` crate, or AWS campaign is required for this workspace.

## Environment

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-spectra-uf-app
```

## Layer 1 — Unit + integration (CI)

Backend contracts (preferred path; no UI graph):

```bash
cargo fmt --all --check
cargo clippy -p spectra-backend --all-targets -- -D warnings
cargo test -p spectra-backend
```

Full workspace (includes `spectra-app` UI). May fail when the path-patched
`uf-product` / `uf-integrations` UI graph is broken upstream — that is a
pre-existing host-product UI compile issue, not a Spectra backend contract gap:

```bash
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
# Host-aligned SSR surface (when UI graph compiles):
cargo test -p spectra-app --features ssr
```

### TEST_MAP

| Behavior | Level | Happy | Sad | Notes |
|----------|-------|-------|-----|-------|
| `validate_spectra_query_name` | unit+integ | non-empty / trimmed name | blank / whitespace → `"required"` | gate for explore queries |
| `spectra_query_permission_name` | unit+integ | `spectra.query.{table}` (trimmed) | — | Gauge name for SP-01 |
| `schema_metadata_list` (`list_schema_metadata`) | unit+integ | `Vec` of named event/metric items | — | empty registry OK |
| `schema_metadata_detail` (`get_schema_metadata`) | unit+integ | detail matches list entry when present | unknown name → `None` | |
| `empty_event_query_result` (`query_events` stub) | unit+integ | empty rows + default `ts` column | — | host backend TBD |
| `empty_event_aggregate_result` (`query_event_aggregate` stub) | unit+integ | empty `TimeSeries` | — | aggregation workflow stub |
| `empty_metrics_query_result` (`query_metrics` stub) | unit+integ | empty series/headline | — | host backend TBD |
| `range_from_secs` (explore window helper) | unit | span / zero window | negative secs inverts | in `spectra-app` when UI compiles |
| `require_spectra_query` + QueryTable session | — | — | — | deferred — needs host SSR (SP-01..03) |
| Leptos UI / Playwright / `cargo leptos` e2e | e2e | — | — | **waived** — covering integ named below |
| AWS / soak | AWS | — | — | **waived** — no cloud resources |
| Micro-benchmarks | bench | — | — | **waived** — no hot-path campaign |

## Layer 2 — E2E

**Waived.** Schema list/detail and explore-query stub shapes are exercised by Layer 1
integration tests named below. A Leptos/UI browser suite or IsolatedLab `*-e2e` crate
is out of scope for this backend-first remediation; live host wiring belongs in the
product host workspace.

Covering integ tests for the e2e waiver:

- `schema_metadata_list_returns_valid_items_happy_path` / `schema_metadata_detail_unknown_name_is_none_sad`
- `schema_metadata_detail_matches_list_entry_happy_path`
- `empty_metrics_query_result_shape_happy_path` / `empty_event_query_result_unknown_table_happy_path`
- `empty_event_aggregate_result_timeseries_stub_happy_path`
- `validate_spectra_query_name_accepts_table_happy_path` / `validate_spectra_query_name_rejects_blank_sad`
- `spectra_query_permission_name_formats_table_happy_path`

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
- Sad-path assertions check message content or `None` — (stronger than `is_err()` alone).
- Happy-path tests are named `*_happy_path` so audits detect them.
- `SpectraRoutes` data loaders call the `#[server]` fns; those fns are thin Higgs
  wrappers over the helpers listed in the TEST_MAP.
