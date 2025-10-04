import { Response, Request } from 'express';
import puppeteer from 'puppeteer';
import { sleep } from './util';
import { readFileSync } from 'fs';

const APP_HOST = process.env.APP_HOST || '';

export const botHandler = async (req: Request, res: Response) => {

    let expr = req.query.expr;

    if (!expr || typeof expr !== 'string') {
        res.json({ status: 'error', message: 'Missing expr query parameter' });
        return;
    }

    try {
        let browser = await puppeteer.launch({
            executablePath: '/usr/bin/chromium',
			headless: true,
			args: [
				"--no-sandbox",
				"--disable-setuid-sandbox",
                "--disable-crashpad",
				"--js-flags=--noexpose_wasm,--jitless"
			]
		});
        let page = await browser.newPage();
        await page.goto(`https://${APP_HOST}/`, { waitUntil: 'networkidle2' });
        await sleep(1000);
        await page.focus('input');
        await page.keyboard.type(expr);
        await sleep(100);
        await page.click('button');
        await sleep(2000);
        const result = await page.evaluate(() => {
            const span = document.querySelector('.ant-alert-message');
            if (!span) {
                return false;
            }
            return span.textContent === '114514';
        });
        if (!result) {
            res.json({ status: 'error', message: 'Result not found or invalid' });
            await page.close();
            await browser.close();
            return;
        }
        if (result) {
            await page.close();
            page = await browser.newPage();
            await page.goto(`https://${APP_HOST}/`, { waitUntil: 'networkidle2' });
            await sleep(1000);
            await page.focus('input');
            await page.keyboard.type(`"${readFileSync('/bot/flag', 'utf-8')}"`, { delay: 100 });
            await sleep(100);
            await page.click('button');
            await sleep(1000);
        }

        await page.close();
        await browser.close();
        
        res.json({ status: 'success' });
    } catch (error) {
        console.error(`[-] Bot failed to visit: ${error}`);
        res.json({ status: 'error', message: 'Bot failed to visit' });
    }
};