FROM debian:stable@sha256:a045f05f78aa1c06a92e4c06cc7c32956c32a51136a724249fd2664e7001ebb8

COPY app/ /app
COPY entrypoint.sh /entrypoint.sh

WORKDIR /app

RUN adduser --disabled-password --no-create-home --gecos "" --shell /bin/sh app &&\
    chown -R app:app /app &&\
    chmod -R 700 /app &&\
    chown -R root:root /app/static/assets &&\
    chown -R root:root /app/templates &&\
    chmod -R 755 /app/static &&\
    chmod -R 755 /app/templates &&\
    chmod 700 /entrypoint.sh

CMD ["/entrypoint.sh"]