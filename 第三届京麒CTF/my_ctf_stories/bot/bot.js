import net from 'net';
import puppeteer from 'puppeteer';

const PORT = 8081;
const FLAG = process.env.FLAG || 'flag{this_is_a_fake_flag}';

const FLAG_REGEX = /flag{[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}}/;
if (!FLAG_REGEX.test(FLAG)) {
    console.error('Invalid flag format. The flag should be a guid.');
    process.exit(1);
}


async function visitUrl(url) {
  const browser = await puppeteer.launch({
    headless: true,
    args: ['--no-sandbox', '--disable-setuid-sandbox'],
  });
  const page = await browser.newPage();
  await page.setCookie({
        name: 'flag',
        value: FLAG,
        domain: 'ctf-app',
        // remote environment: domain: 'localhost',
  });
  await page.goto(url, { waitUntil: 'networkidle2' });
  await new Promise(r => setTimeout(r, 15000));
  console.log(`Bot finished visiting: ${url}`);
  await browser.close();
}

const server = net.createServer((socket) => {
  socket.on('data', async (data) => {
    const url = data.toString().trim();
    if (!/^https?:\/\//.test(url)) {
      console.error(`Invalid URL format: ${url}`);
      socket.end();
      return;
    }
    try {
      console.log(`Bot visiting: ${url}`);
      if (socket.writable) {
        socket.write(`Bot visiting: ${url}\n`);
      }
      await visitUrl(url);
      if (socket.writable) {
        socket.write(`Bot finished visiting: ${url}\n`);
      }
    } catch (e) {
      console.error(`Error visiting ${url}:`, e);
    }
    socket.end();
  });
});

server.listen(PORT, () => {
  console.log(`XSS Bot listening on port ${PORT}`);
});
