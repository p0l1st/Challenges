FROM node:18-slim AS bot

ENV PUPPETEER_SKIP_CHROMIUM_DOWNLOAD=true

ENV XDG_CONFIG_HOME=/tmp/.chromium \
    XDG_CACHE_HOME=/tmp/.chromium

RUN apt-get update && apt-get install -y --no-install-recommends \
      gnupg2 wget ca-certificates \
    && wget -qO- https://dl.google.com/linux/linux_signing_key.pub \
         | gpg --dearmor > /usr/share/keyrings/google-linux-signing-keyring.gpg \
    && echo "deb [arch=amd64 signed-by=/usr/share/keyrings/google-linux-signing-keyring.gpg] \
         http://dl.google.com/linux/chrome/deb/ stable main" \
         > /etc/apt/sources.list.d/google-chrome.list \
    && apt-get update \
    && apt-get install -y --no-install-recommends google-chrome-stable \
    && rm -rf /var/lib/apt/lists/*            

WORKDIR /app
COPY package*.json ./
RUN npm install

COPY bot.js .
EXPOSE 8081

CMD ["node", "bot.js"]
