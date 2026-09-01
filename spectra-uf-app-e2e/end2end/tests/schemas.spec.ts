import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("pw-spectra-schemas", () => {
  test("pw-spectra-schema-index-happy", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/spectra/schema", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("schema-index-page")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByTestId("spectra-app-root")).toBeVisible();
  });

  test("pw-spectra-schema-detail-happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await page.goto(`/spectra/schema/${seeded.fixtures.event_table}`, {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("spectra-schema-detail-page")).toBeVisible({
      timeout: 60_000,
    });
  });
});
