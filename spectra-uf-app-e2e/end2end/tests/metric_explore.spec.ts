import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("pw-spectra-metric-explore", () => {
  test("pw-spectra-metric-explore-happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await page.goto(`/spectra/metric/${seeded.fixtures.metric_name}/explore`, {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("spectra-metric-explore-panel")).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.getByTestId("spectra-app-root")).toBeVisible();
  });
});
