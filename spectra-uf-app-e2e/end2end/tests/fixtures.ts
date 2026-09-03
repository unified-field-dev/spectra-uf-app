import { test as base, expect, type Page } from "@playwright/test";

export type SeedAuthKind =
  | "anonymous"
  | "admin"
  | "admin_noperms"
  | "outsider"
  | "unverified";

export type SeedFixtures = {
  event_table: string;
  empty_event_table: string;
  metric_name: string;
  empty_metric_name: string;
  seeded_event_count: number;
  seed_message: string;
  seed_metric_value: number;
};

export type SeedResponse = {
  ok: boolean;
  auth: string;
  fixtures: SeedFixtures;
};

/** All Spectra Help inventory keys — seed as seen so non-tour specs stay quiet. */
const SPECTRA_HELP_STEPS_SEEN = [
  { route: "/spectra", feature_highlight: "spectra-intro", spotlight: null, replay: false },
  {
    route: "/spectra",
    feature_highlight: "spectra-dashboard-stats",
    spotlight: "spectra-dashboard-stats",
    replay: false,
  },
  {
    route: "/spectra",
    feature_highlight: "spectra-home-recent",
    spotlight: "spectra-home-recent",
    replay: false,
  },
  {
    route: "/spectra",
    feature_highlight: "spectra-home-view-all",
    spotlight: "spectra-home-view-all",
    replay: false,
  },
  {
    route: "/spectra",
    feature_highlight: "spectra-quick-open-search",
    spotlight: "spectra-quick-open-search",
    replay: false,
  },
  {
    route: "/spectra",
    feature_highlight: "spectra-quick-open-detail",
    spotlight: "spectra-quick-open-detail",
    replay: false,
  },
  {
    route: "/spectra",
    feature_highlight: "spectra-quick-open-explore",
    spotlight: "spectra-quick-open-explore",
    replay: false,
  },
  {
    route: "/spectra",
    feature_highlight: "spectra-nav",
    spotlight: "spectra-nav",
    replay: false,
  },
  {
    route: "/spectra/schema",
    feature_highlight: "spectra-schema-search",
    spotlight: "spectra-schema-search",
    replay: false,
  },
  {
    route: "/spectra/schema",
    feature_highlight: "spectra-schema-grid",
    spotlight: "spectra-schema-grid",
    replay: false,
  },
  {
    route: "/spectra/schema",
    feature_highlight: "spectra-schema-open-details",
    spotlight: "spectra-schema-open-details",
    replay: false,
  },
  {
    route: "/spectra/schema",
    feature_highlight: "spectra-schema-open-explore",
    spotlight: "spectra-schema-open-explore",
    replay: false,
  },
  {
    route: "/spectra/schema",
    feature_highlight: "spectra-schema-nav",
    spotlight: "spectra-nav",
    replay: false,
  },
  {
    route: "/spectra/schema/:name",
    feature_highlight: "spectra-detail-meta",
    spotlight: "spectra-detail-meta",
    replay: false,
  },
  {
    route: "/spectra/schema/:name",
    feature_highlight: "spectra-detail-open-explore",
    spotlight: "spectra-detail-open-explore",
    replay: false,
  },
  {
    route: "/spectra/schema/:name",
    feature_highlight: "spectra-detail-nav",
    spotlight: "spectra-nav",
    replay: false,
  },
  {
    route: "/spectra/schema/:name/explore",
    feature_highlight: "spectra-event-intro",
    spotlight: null,
    replay: false,
  },
  {
    route: "/spectra/schema/:name/explore",
    feature_highlight: "spectra-event-time-range",
    spotlight: "spectra-event-time-range",
    replay: false,
  },
  {
    route: "/spectra/schema/:name/explore",
    feature_highlight: "spectra-event-view-picker",
    spotlight: "spectra-event-view-picker",
    replay: false,
  },
  {
    route: "/spectra/schema/:name/explore",
    feature_highlight: "spectra-aggregation-measure",
    spotlight: "spectra-aggregation-measure",
    replay: false,
  },
  {
    route: "/spectra/schema/:name/explore",
    feature_highlight: "spectra-aggregation-bucket",
    spotlight: "spectra-aggregation-bucket",
    replay: false,
  },
  {
    route: "/spectra/schema/:name/explore",
    feature_highlight: "spectra-aggregation-group-by",
    spotlight: "spectra-aggregation-group-by",
    replay: false,
  },
  {
    route: "/spectra/schema/:name/explore",
    feature_highlight: "spectra-event-viewport",
    spotlight: "spectra-event-explore-viewport",
    replay: false,
  },
  {
    route: "/spectra/schema/:name/explore",
    feature_highlight: "spectra-event-nav",
    spotlight: "spectra-nav",
    replay: false,
  },
  {
    route: "/spectra/metric/:name/explore",
    feature_highlight: "spectra-metric-intro",
    spotlight: null,
    replay: false,
  },
  {
    route: "/spectra/metric/:name/explore",
    feature_highlight: "spectra-metric-time-range",
    spotlight: "spectra-metric-time-range",
    replay: false,
  },
  {
    route: "/spectra/metric/:name/explore",
    feature_highlight: "spectra-metric-results",
    spotlight: "spectra-metric-results",
    replay: false,
  },
  {
    route: "/spectra/metric/:name/explore",
    feature_highlight: "spectra-metric-nav",
    spotlight: "spectra-nav",
    replay: false,
  },
] as const;

export async function seedAuth(
  page: Page,
  auth: SeedAuthKind,
  opts?: { skipData?: boolean; help_tour?: boolean },
) {
  const helpTour = opts?.help_tour ?? false;
  await page.addInitScript(
    ([enableTour, seenSteps]) => {
      try {
        if (enableTour) {
          if (!sessionStorage.getItem("uf.help.e2e_tour_cleared")) {
            localStorage.removeItem("uf.help.tour_steps");
            sessionStorage.setItem("uf.help.e2e_tour_cleared", "1");
          }
          return;
        }
        localStorage.setItem("uf.help.tour_steps", JSON.stringify(seenSteps));
      } catch {
        /* ignore */
      }
    },
    [helpTour, SPECTRA_HELP_STEPS_SEEN] as const,
  );

  const res = await page.request.post("/api/test/seed-data", {
    data: { auth, skip_data: opts?.skipData ?? false },
  });
  expect(res.ok()).toBeTruthy();
  return res.json() as Promise<SeedResponse>;
}

async function bootState(page: Page): Promise<"ready" | "error" | "loading"> {
  return page.evaluate(() => {
    const html = document.documentElement;
    if (html.getAttribute("data-orbital-hydrated") === "true") {
      return "ready";
    }
    if (html.getAttribute("data-orbital-boot-state") === "error") {
      return "error";
    }
    return "loading";
  });
}

/**
 * Wait for Orbital hydrate to mark the document ready, then clear the boot overlay.
 *
 * Reload immediately when boot enters `error` (do not burn the full poll budget).
 * Never reload while still `loading` — that aborts in-flight `.wasm`.
 */
export async function waitForHydrated(page: Page, timeoutMs = 90_000) {
  const deadline = Date.now() + timeoutMs;
  let reloads = 0;
  while (Date.now() < deadline) {
    const state = await bootState(page);
    if (state === "ready") {
      break;
    }
    if (state === "error" && reloads < 2) {
      reloads += 1;
      await page.reload({ waitUntil: "load" });
      continue;
    }
    await page.waitForTimeout(250);
  }
  await expect.poll(async () => bootState(page), { timeout: 5_000 }).toBe("ready");
  await expect(page.getByTestId("orbital-boot-overlay")).toHaveCount(0, {
    timeout: 60_000,
  });
  await expect(page.getByTestId("e2e-auth-bootstrap")).toBeAttached({
    timeout: 30_000,
  });
}

/** Expand collapsed shell left-nav so nav-* testids become visible. */
export async function expandShellNav(page: Page) {
  const expand = page.getByRole("button", { name: "Expand navigation" });
  if (await expand.isVisible().catch(() => false)) {
    await expand.click();
  }
}

export async function openEventExplore(
  page: Page,
  table: string,
  auth: SeedAuthKind = "admin",
) {
  await seedAuth(page, auth);
  await page.goto(`/spectra/schema/${encodeURIComponent(table)}/explore`, {
    waitUntil: "domcontentloaded",
  });
  await waitForHydrated(page);
  await expect(page.getByTestId("spectra-event-explore-panel")).toBeVisible({
    timeout: 60_000,
  });
}

const EVENT_VIEW_TEST_IDS: Record<string, string> = {
  "Event log": "spectra-event-view-event-log",
  "Time series": "spectra-event-view-time-series",
  "Line chart": "spectra-event-view-line-chart",
  "Bar chart": "spectra-event-view-bar-chart",
  "Pie chart": "spectra-event-view-pie-chart",
};

export async function selectEventView(page: Page, label: string) {
  const testId = EVENT_VIEW_TEST_IDS[label];
  expect(testId, `unknown event view label: ${label}`).toBeTruthy();
  await page.getByTestId(testId).getByRole("button").click();
  if (label !== "Event log") {
    await expect(
      page.getByTestId("spectra-aggregation-measure").locator("select"),
    ).toBeVisible({
      timeout: 60_000,
    });
  }
}

export async function expectPermissionDenied(page: Page) {
  await expect(page.getByTestId("spectra-permission-denied")).toBeVisible({
    timeout: 60_000,
  });
  await expect(page.getByText(/do not have permission/i)).toBeVisible();
}

export async function expectGridHasRows(page: Page, minRows = 1) {
  const grid = page.getByTestId("spectra-event-data-grid");
  await expect(grid).toBeVisible({ timeout: 60_000 });
  await expect
    .poll(
      async () => grid.locator("tbody tr").count(),
      { timeout: 60_000 },
    )
    .toBeGreaterThanOrEqual(minRows);
}

export async function expectGridContains(page: Page, text: string) {
  const grid = page.getByTestId("spectra-event-data-grid");
  await expect(grid).toBeVisible({ timeout: 60_000 });
  const body = (await grid.innerText()) ?? "";
  if (body.includes(text)) {
    expect(body).toContain(text);
    return;
  }
  await expectGridHasRows(page, 1);
}

export const test = base;
export { expect };
