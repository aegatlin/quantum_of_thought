import { test, expect } from '@playwright/test';

test('displays Quantum of Thought', async ({ page }) => {
  await page.goto('/');

  await expect(page.getByText('Quantum of Thought')).toBeVisible();
});
