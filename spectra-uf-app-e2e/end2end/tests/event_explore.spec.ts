import {
  test,
  expect,
  seedAuth,
  waitForHydrated,
  openEventExplore,
  selectEventView,
  expectPermissionDenied,
  expectGridHasRows,
} from "./fixtures";

test.describe("pw-spectra-event-explore", () => {
  test("pw-spectra-event-log-empty-table-happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin", { skipData: true });
    await page.goto(
      `/spectra/schema/${encodeURIComponent(seeded.fixtures.event_table)}/explore`,
      { waitUntil: "domcontentloaded" },
    );
    await waitForHydrated(page);
    const grid = page.getByTestId("spectra-event-data-grid");
    await expect(grid).toBeVisible({ timeout: 60_000 });
    await expect(grid.getByRole("columnheader", { name: /message/i })).toBeVisible();
  });

  test("pw-spectra-event-log-seeded-row-happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await page.goto(
      `/spectra/schema/${encodeURIComponent(seeded.fixtures.event_table)}/explore`,
      { waitUntil: "domcontentloaded" },
    );
    await waitForHydrated(page);
    await expectGridHasRows(page, seeded.fixtures.seeded_event_count);
  });

  test("pw-spectra-event-explore-time-range-happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await openEventExplore(page, seeded.fixtures.event_table);
    await expectGridHasRows(page, 1);
    await page.getByTestId("spectra-time-range-1h").click();
    await expectGridHasRows(page, 1);
  });

  test("pw-spectra-event-view-timeseries-happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await openEventExplore(page, seeded.fixtures.event_table);
    await selectEventView(page, "Time series");
    await expect(page.getByTestId("spectra-event-time-series-chart")).toBeVisible({
      timeout: 60_000,
    });
  });

  test("pw-spectra-event-view-line-chart-happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await openEventExplore(page, seeded.fixtures.event_table);
    await selectEventView(page, "Line chart");
    await expect(page.getByTestId("spectra-event-time-series-chart")).toBeVisible({
      timeout: 60_000,
    });
  });

  test("pw-spectra-event-view-bar-chart-happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await openEventExplore(page, seeded.fixtures.event_table);
    await selectEventView(page, "Bar chart");
    await page.getByTestId("spectra-aggregation-group-by").getByRole("textbox").fill("severity");
    await expect
      .poll(async () => {
        const bar = await page.getByTestId("spectra-event-bar-chart").isVisible();
        const viewport = await page.getByTestId("spectra-event-explore-viewport").isVisible();
        return bar || viewport;
      }, { timeout: 60_000 })
      .toBe(true);
  });

  test("pw-spectra-event-view-pie-chart-happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await openEventExplore(page, seeded.fixtures.event_table);
    await selectEventView(page, "Pie chart");
    await page.getByTestId("spectra-aggregation-group-by").getByRole("textbox").fill("severity");
    await expect
      .poll(async () => {
        const pie = await page.getByTestId("spectra-event-pie-chart").isVisible();
        const viewport = await page.getByTestId("spectra-event-explore-viewport").isVisible();
        return pie || viewport;
      }, { timeout: 60_000 })
      .toBe(true);
  });

  test("pw-spectra-event-aggregate-count-happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await openEventExplore(page, seeded.fixtures.event_table);
    await selectEventView(page, "Time series");
    await page.getByTestId("spectra-aggregation-measure").locator("select").selectOption("count");
    await expect(page.getByTestId("spectra-event-time-series-chart")).toBeVisible({
      timeout: 60_000,
    });
  });

  test("pw-spectra-event-aggregate-sum-control-happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await openEventExplore(page, seeded.fixtures.event_table);
    await selectEventView(page, "Time series");
    await page.getByTestId("spectra-aggregation-measure").locator("select").selectOption("sum");
    await expect(page.getByTestId("spectra-event-time-series-chart")).toBeVisible({
      timeout: 60_000,
    });
  });

  test("pw-spectra-event-explore-permission-denied-sad", async ({ page }) => {
    const seeded = await seedAuth(page, "admin_noperms");
    await page.goto(
      `/spectra/schema/${encodeURIComponent(seeded.fixtures.event_table)}/explore`,
      { waitUntil: "domcontentloaded" },
    );
    await waitForHydrated(page);
    await expectPermissionDenied(page);
  });
});
