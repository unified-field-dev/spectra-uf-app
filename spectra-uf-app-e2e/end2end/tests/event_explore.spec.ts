import {
  test,
  expect,
  seedAuth,
  waitForHydrated,
  openEventExplore,
  selectEventView,
  expectPermissionDenied,
  expectGridHasRows,
  expectEventChartHasData,
  expectEventChartEmpty,
  fillGroupBy,
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
    await expect(page.getByTestId("spectra-event-time-range")).toBeVisible();
    await page.getByTestId("spectra-refresh-data").getByRole("button").click();
    await expectGridHasRows(page, 1);
  });

  test("pw-spectra-event-refresh-happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await openEventExplore(page, seeded.fixtures.event_table);
    await expectGridHasRows(page, 1);
    await page.getByTestId("spectra-refresh-data").getByRole("button").click();
    await expect(page.getByText(/Last refreshed/i)).toBeVisible({ timeout: 60_000 });
    await expect(page).toHaveURL(
      new RegExp(`/spectra/schema/${seeded.fixtures.event_table}/explore`),
    );
  });

  test("pw-spectra-event-view-timeseries-data-happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await openEventExplore(page, seeded.fixtures.event_table);
    await selectEventView(page, "Time series");
    await expectEventChartHasData(page, "time_series", { minTotal: 3 });
  });

  test("pw-spectra-event-view-bar-chart-data-happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await openEventExplore(page, seeded.fixtures.event_table);
    await selectEventView(page, "Bar chart");
    await fillGroupBy(page, "severity");
    await expectEventChartHasData(page, "bar", {
      minTotal: 2,
      labels: ["info", "warn"],
    });
  });

  test("pw-spectra-event-view-pie-chart-data-happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await openEventExplore(page, seeded.fixtures.event_table);
    await selectEventView(page, "Pie chart");
    await fillGroupBy(page, "severity");
    // Pie marks use values, not band tick labels — assert data via headline + SVG marks.
    await expectEventChartHasData(page, "pie", { minTotal: 2 });
  });

  test("pw-spectra-event-aggregate-sum-data-happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await openEventExplore(page, seeded.fixtures.event_table);
    await selectEventView(page, "Time series");
    await page.getByTestId("spectra-aggregation-measure").locator("select").selectOption("sum");
    const field = page.getByTestId("spectra-aggregation-measure-field").locator("select");
    await field.selectOption("value");
    await page.getByTestId("spectra-event-see").getByRole("button").click();
    await expectEventChartHasData(page, "time_series", { minTotal: 18 });
  });

  test("pw-spectra-event-view-pie-need-groupby-sad", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await openEventExplore(page, seeded.fixtures.event_table);
    await selectEventView(page, "Pie chart");
    await expectEventChartEmpty(page, "need_group_by");
  });

  test("pw-spectra-event-view-bar-need-groupby-sad", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await openEventExplore(page, seeded.fixtures.event_table);
    await selectEventView(page, "Bar chart");
    await expectEventChartEmpty(page, "need_group_by");
  });

  test("pw-spectra-event-view-timeseries-empty-table-sad", async ({ page }) => {
    const seeded = await seedAuth(page, "admin", { skipData: true });
    await page.goto(
      `/spectra/schema/${encodeURIComponent(seeded.fixtures.empty_event_table)}/explore`,
      { waitUntil: "domcontentloaded" },
    );
    await waitForHydrated(page);
    await selectEventView(page, "Time series");
    await expectEventChartEmpty(page, "no_series");
  });

  test("pw-spectra-event-view-pie-empty-groupby-sad", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await openEventExplore(page, seeded.fixtures.event_table);
    await selectEventView(page, "Pie chart");
    // Leave group-by on "(select field)" — empty slices / need group-by empty state.
    await expectEventChartEmpty(page, "need_group_by");
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
