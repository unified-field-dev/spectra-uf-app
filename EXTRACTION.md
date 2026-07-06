# spectra-uf-app extraction

        Upstream playbook for [deathbreakfast/spectra-uf-app](https://github.com/deathbreakfast/spectra-uf-app).

        ## Workspace crates

        | Phase | Work | Status |
        |-------|------|--------|
        | 0 | skeleton | shipped — workspace + stub crates |
| 1 | core import | NOT_STARTED |
| 2 | git deps + verify | NOT_STARTED |## Dependencies (git)

Platform libraries are pinned in the root `Cargo.toml` `[workspace.dependencies]`.
Use release tags when available; `branch = "main"` during initial skeleton phase.

## Gating (before tag)

- No zone vocabulary in public docs
- Hero README with quick-start git dependency
- `cargo check --workspace` green on skeleton
