FROM node:lts-alpine@sha256:f265794478aa0b1a23d85a492c8311ed795bc527c3fe7e43453b3c872dcd71a3

COPY bot/ /bot

COPY entrypoint.sh /entrypoint.sh

WORKDIR /bot

ENV NODE_ENV=production
ENV PUPPETEER_SKIP_CHROMIUM_DOWNLOAD=true

RUN sed -i 's#https\?://dl-cdn.alpinelinux.org/alpine#https://mirrors.tuna.tsinghua.edu.cn/alpine#g' /etc/apk/repositories&&\
    apk update &&\
    apk add chromium &&\
    yarn config set registry https://registry.npmmirror.com/ &&\
    yarn &&\
    yarn build &&\
    adduser --disabled-password --gecos "" --shell /bin/sh bot &&\
    chown -R bot:bot /bot &&\
    chmod -R 700 /bot &&\
    chown root:root /bot/flag &&\
    chmod 755 /bot/flag &&\
    chmod 700 /entrypoint.sh

CMD ["/entrypoint.sh"]