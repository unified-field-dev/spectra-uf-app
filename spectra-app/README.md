# spectra-app

Leptos admin UI for Spectra: schema browsing and event/metric explore under `/spectra`.

Wire routes through your host shell. Query server functions gate on a non-empty table/metric
name; live Spectra backend wiring is composed by the host deployment.

## Feature checks

```bash
cargo check -p spectra-app
cargo check -p spectra-app --features hydrate
cargo check -p spectra-app --features ssr
```
