import { test, expect, seedAuth, waitForHydrated, expandShellNav } from "./fixtures";

test.describe("pw-spectra-home", () => {
  test("pw-spectra-home-happy-load", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/spectra", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("spectra-home-page")).toBeVisible({ timeout: 60_000 });
    await expandShellNav(page);
    await expect(page.getByTestId("nav-spectra-home")).toBeVisible({ timeout: 30_000 });
    await expect(page.getByTestId("nav-spectra-schemas")).toBeVisible();
  });

  test("pw-spectra-home-nav-schemas-happy", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/spectra", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expandShellNav(page);
    await page.getByTestId("nav-spectra-schemas").click();
    await expect(page.getByTestId("schema-index-page")).toBeVisible({
      timeout: 60_000,
    });
  });
});
