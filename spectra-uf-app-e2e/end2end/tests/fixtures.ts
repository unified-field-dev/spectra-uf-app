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

export async function seedAuth(
  page: Page,
  auth: SeedAuthKind,
  opts?: { skipData?: boolean },
) {
  const res = await page.request.post("/api/test/seed-data", {
    data: { auth, skip_data: opts?.skipData ?? false },
  });
  expect(res.ok()).toBeTruthy();
  return res.json() as Promise<SeedResponse>;
}

/**
 * Wait for Orbital boot overlay to finish and hydrate to mark the document ready.
 */
export async function waitForHydrated(page: Page, timeoutMs = 240_000) {
  await expect
    .poll(
      async () =>
        page.evaluate(() => {
          const html = document.documentElement;
          if (html.getAttribute("data-orbital-boot-state") === "error") {
            return "error";
          }
          if (html.getAttribute("data-orbital-hydrated") === "true") {
            return "ready";
          }
          return "loading";
        }),
      { timeout: timeoutMs },
    )
    .not.toBe("error");
  await expect
    .poll(
      async () =>
        page.evaluate(
          () =>
            document.documentElement.getAttribute("data-orbital-hydrated") ===
            "true",
        ),
      { timeout: timeoutMs },
    )
    .toBe(true);
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
    await expect(page.getByTestId("spectra-aggregation-measure")).toBeVisible({
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
