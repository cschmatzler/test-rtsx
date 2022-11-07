import { test, expect } from "@playwright/test";

test("has hello world", async ({ page }) => {
	await page.goto("http://localhost:3000");

	const heading = page.locator("h1");
	expect(await heading.textContent()).toBe("Hello, World!");
});
