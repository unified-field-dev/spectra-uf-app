import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("pw-spectra-navigation", () => {
  test("pw-spectra-breadcrumbs-explore-happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await page.goto(
      `/spectra/schema/${encodeURIComponent(seeded.fixtures.event_table)}/explore`,
      { waitUntil: "domcontentloaded" },
    );
    await waitForHydrated(page);
    await expect(page.getByTestId("spectra-breadcrumbs")).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.getByTestId("spectra-breadcrumbs")).toContainText(
      seeded.fixtures.event_table,
    );
  });

  test("pw-spectra-direct-explore-url-happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await page.goto(
      `/spectra/schema/${encodeURIComponent(seeded.fixtures.event_table)}/explore`,
      { waitUntil: "domcontentloaded" },
    );
    await waitForHydrated(page);
    await expect(page.getByTestId("spectra-event-explore-panel")).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.getByTestId("spectra-event-data-grid")).toBeVisible({
      timeout: 60_000,
    });
  });
});
