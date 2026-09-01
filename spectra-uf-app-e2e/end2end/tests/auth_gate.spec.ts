import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("pw-spectra-auth-gate", () => {
  test("pw-spectra-auth-gate-sad-anonymous", async ({ page }) => {
    await seedAuth(page, "anonymous");
    await page.goto("/spectra", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("auth-required-empty-state")).toBeAttached({
      timeout: 60_000,
    });
    await expect(page.getByTestId("spectra-home-page")).toHaveCount(0);
  });

  test("pw-spectra-auth-gate-happy-admin", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/spectra", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("spectra-app-root")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByTestId("spectra-home-page")).toBeVisible({ timeout: 60_000 });
  });
});
