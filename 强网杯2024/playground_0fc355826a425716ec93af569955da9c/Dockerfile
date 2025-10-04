FROM python:3.9-alpine@sha256:2ae855d07a137e4e39f9da8995f2fcd938c5bcddce466f9a8ac437b2de68e1b1 as base

ADD go1.23.2.linux-amd64.tar.gz /

RUN pip install --no-cache-dir flask &&\
    mkdir -p /tmp/go &&\
    mkdir -p /tmp/go/cache &&\
    chmod -R 777 /tmp/go &&\
    apk add --no-cache build-base

ENV PATH="$PATH:/go/bin"
ENV GOPATH="/tmp/go"
ENV GOROOT="/go"
ENV GO111MODULE="on"
ENV GOPROXY="https://goproxy.cn,direct"
ENV GOCACHE="/tmp/go/cache"

FROM base 

COPY ./app /app
COPY ./sandbox /sandbox/sandbox
COPY ./entrypoint.sh /entrypoint.sh

WORKDIR /app

RUN adduser --disabled-password --no-create-home --gecos "" --shell /bin/sh app &&\
    chown -R app:app /app &&\
    chmod -R 700 /app &&\
    chown -R root:root /sandbox &&\
    chmod -R 755 /sandbox &&\
    chmod 700 /entrypoint.sh

CMD ["/entrypoint.sh"]