import asyncio
from playwright.async_api import async_playwright
import time

async def run():
    async with async_playwright() as p:
        browser = await p.chromium.launch()

        # Test Light Mode
        context_light = await browser.new_context(color_scheme='light')
        page_light = await context_light.new_page()
        try:
            await page_light.goto("http://localhost:3000")
            # Wait for styles to load
            await page_light.wait_for_load_state('networkidle')
            await page_light.screenshot(path="screenshot_light.png")
            print("Light mode screenshot taken.")
        except Exception as e:
            print(f"Error taking light mode screenshot: {e}")
        finally:
            await context_light.close()

        # Test Dark Mode
        context_dark = await browser.new_context(color_scheme='dark')
        page_dark = await context_dark.new_page()
        try:
            await page_dark.goto("http://localhost:3000")
            # Wait for styles to load
            await page_dark.wait_for_load_state('networkidle')
            await page_dark.screenshot(path="screenshot_dark.png")
            print("Dark mode screenshot taken.")
        except Exception as e:
            print(f"Error taking dark mode screenshot: {e}")
        finally:
            await context_dark.close()

        await browser.close()

if __name__ == "__main__":
    asyncio.run(run())
