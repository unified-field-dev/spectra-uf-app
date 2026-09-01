# Contributing to Spectra UF App

Thank you for improving this project.

## Development setup

1. Clone [unified-field-dev/spectra-uf-app](https://github.com/unified-field-dev/spectra-uf-app) (or your fork).
2. Install Rust **nightly** (`rust-toolchain.toml` pins the workspace toolchain).
3. From the repository root:

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-spectra-uf-app
export RUSTFLAGS="-D warnings"

cargo fmt -p spectra-backend -p spectra-app -p protected-spectra-host -- --check
cargo test -p spectra-backend
cargo test -p spectra-app --features ssr
```

Full verify layers: [`docs/VERIFICATION.md`](docs/VERIFICATION.md). Repository map:
[`docs/DEVELOPMENT.md`](docs/DEVELOPMENT.md).

## Code of conduct

Participation is governed by [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md). Security reports: [`SECURITY.md`](SECURITY.md).

## Pull requests

- Prefer small, focused PRs.
- Update [`README.md`](README.md) when public API or host wiring steps change.
- Run the Verify block in README or VERIFICATION before opening a PR.
