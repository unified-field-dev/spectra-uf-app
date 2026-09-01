# spectra-uf-app-e2e

Playwright IsolatedLab host for [`spectra-app`](../spectra-app/) on `127.0.0.1:3200`.

## Run

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-spectra-uf-app
cd L4-composers/spectra-uf-app
cargo leptos end-to-end --project spectra-uf-app-e2e
```

Do not interrupt — the command exits when Playwright finishes.

Headed debug:

```bash
cargo leptos watch --project spectra-uf-app-e2e
cd spectra-uf-app-e2e/end2end && npm ci && npx playwright install chromium
npm run test:headed
```

## Scenario catalog

| ID | Spec file | Asserts |
|----|-----------|---------|
| `pw-spectra-auth-gate-sad-anonymous` | `auth_gate.spec.ts` | Anonymous → `auth-required-empty-state`; home absent |
| `pw-spectra-auth-gate-happy-admin` | `auth_gate.spec.ts` | Admin → home visible |
| `pw-spectra-auth-outsider-schema-index-sad` | `auth_gate.spec.ts` | Outsider → schema load error |
| `pw-spectra-auth-unverified-schema-index-sad` | `auth_gate.spec.ts` | Unverified → schema load error |
| `pw-spectra-guest-schema-index-sad` | `auth_gate.spec.ts` | Guest `/spectra/schema` → auth empty state |
| `pw-spectra-guest-event-explore-sad` | `auth_gate.spec.ts` | Guest event explore → auth empty state |
| `pw-spectra-guest-metric-explore-sad` | `auth_gate.spec.ts` | Guest metric explore → auth empty state |
| `pw-spectra-explore-admin-noperms-sad` | `auth_gate.spec.ts` | Admin without table perm → permission denied |
| `pw-spectra-home-happy-load` | `home.spec.ts` | Home + nav testids |
| `pw-spectra-home-stat-cards-happy` | `home.spec.ts` | Dashboard stat cards visible |
| `pw-spectra-home-recent-schema-happy` | `home.spec.ts` | Seeded schema card on home |
| `pw-spectra-home-nav-schemas-happy` | `home.spec.ts` | Nav → schema index |
| `pw-spectra-home-quick-open-detail-happy` | `home.spec.ts` | Quick open → detail |
| `pw-spectra-home-quick-open-explore-happy` | `home.spec.ts` | Quick open → explore |
| `pw-spectra-home-quick-open-blank-sad` | `home.spec.ts` | Blank quick open does not navigate |
| `pw-spectra-schema-index-happy` | `schemas.spec.ts` | Index + seeded card |
| `pw-spectra-schema-search-happy` | `schemas.spec.ts` | Search filters to seeded table |
| `pw-spectra-schema-search-no-match-sad` | `schemas.spec.ts` | Search empty state |
| `pw-spectra-schema-detail-happy` | `schemas.spec.ts` | Detail page metadata |
| `pw-spectra-schema-detail-explore-cta-happy` | `schemas.spec.ts` | Detail explore CTA |
| `pw-spectra-schema-card-details-link-happy` | `schemas.spec.ts` | Card Details link |
| `pw-spectra-schema-card-explore-link-happy` | `schemas.spec.ts` | Card Explore link |
| `pw-spectra-schema-unknown-sad` | `schemas.spec.ts` | Unknown schema empty state |
| `pw-spectra-event-log-seeded-row-happy` | `event_explore.spec.ts` | Grid contains seed message |
| `pw-spectra-event-log-empty-table-happy` | `event_explore.spec.ts` | Empty table grid mounts |
| `pw-spectra-event-explore-time-range-happy` | `event_explore.spec.ts` | Time range reload |
| `pw-spectra-event-view-timeseries-happy` | `event_explore.spec.ts` | Time series chart |
| `pw-spectra-event-view-line-chart-happy` | `event_explore.spec.ts` | Line chart |
| `pw-spectra-event-view-bar-chart-happy` | `event_explore.spec.ts` | Bar chart + group by |
| `pw-spectra-event-view-pie-chart-happy` | `event_explore.spec.ts` | Pie chart + group by |
| `pw-spectra-event-aggregate-count-happy` | `event_explore.spec.ts` | Count aggregation control |
| `pw-spectra-event-aggregate-sum-control-happy` | `event_explore.spec.ts` | Sum aggregation control |
| `pw-spectra-event-explore-permission-denied-sad` | `event_explore.spec.ts` | Permission denied |
| `pw-spectra-metric-chart-happy` | `metric_explore.spec.ts` | Metric chart visible |
| `pw-spectra-metric-headline-happy` | `metric_explore.spec.ts` | Headline shows seeded 42 |
| `pw-spectra-metric-time-range-happy` | `metric_explore.spec.ts` | Metric time range |
| `pw-spectra-metric-empty-happy` | `metric_explore.spec.ts` | Empty metric chart mounts |
| `pw-spectra-metric-permission-denied-sad` | `metric_explore.spec.ts` | Permission denied |
| `pw-spectra-breadcrumbs-explore-happy` | `navigation.spec.ts` | Breadcrumbs show table name |
| `pw-spectra-direct-explore-url-happy` | `navigation.spec.ts` | Direct URL loads grid |

Seed profiles via `POST /api/test/seed-data`: `admin`, `admin_noperms`, `outsider`, `unverified`, `anonymous`; optional `skip_data: true`.
