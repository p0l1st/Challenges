FROM rust:1.76-slim-bookworm AS build

ENV TINI_VERSION=v0.19.0
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini /tini
RUN chmod +x /tini
ENTRYPOINT ["/tini", "--"]

RUN apt-get update -y && apt-get install -y pkg-config libudev-dev libssl-dev

COPY ./server /app/server
COPY ./challenge /app/challenge

WORKDIR /app/server

RUN cargo build --release && cp target/release/server /app/run && cargo clean

FROM debian:bookworm-slim

COPY --from=build /app/run /app/run
COPY ./challenge.so /app/challenge.so
WORKDIR /app

CMD ["/app/run"]