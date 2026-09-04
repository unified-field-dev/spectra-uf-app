# Examples

Runnable teaching hosts for this UF app. Each card: when to use · command ·
success · look next.

## Canonical path

### `protected-spectra-host` — auth + `/spectra` schema index

**Teaches:** session auth gate on `/spectra` and the schema catalog
`spectra-backend` exposes for the UI index. Inventory names: `spectra` /
`/spectra` / `RequireAuthenticated` / `QueryTable`.

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-spectra-uf-app
cargo run -p protected-spectra-host
```

**Success:** stdout prints `protected_spectra_host: OK — /spectra deny/allow + schema index`.

**Next step:** Mount `<SpectraRoutes />` in a product host with Spectra query
backends.

Copy table + product mount `Cargo.toml`:
[`protected-spectra-host/README.md`](protected-spectra-host/README.md).

| Host | When to use | Command | Success | Look next |
|------|-------------|---------|---------|-----------|
| [`protected-spectra-host`](protected-spectra-host/) | Auth + `/spectra` schema index | `cargo run -p protected-spectra-host` | Deny/allow + schema JSON | Product host with `SpectraRoutes` |
