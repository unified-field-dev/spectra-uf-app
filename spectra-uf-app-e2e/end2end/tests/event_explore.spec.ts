import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("pw-spectra-event-explore", () => {
  test("pw-spectra-event-explore-happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await page.goto(`/spectra/schema/${seeded.fixtures.event_table}/explore`, {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("spectra-event-explore-panel")).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.getByTestId("spectra-app-root")).toBeVisible();
  });
});
