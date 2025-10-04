FROM rust:1.84-slim@sha256:0ec205a9abb049604cb085f2fdf7630f1a31dad1f7ad4986154a56501fb7ca77 AS builder

RUN mkdir -vp ${CARGO_HOME:-$HOME/.cargo}
RUN cat <<EOF | tee -a ${CARGO_HOME:-$HOME/.cargo}/config.toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"
EOF

WORKDIR /build/app/
COPY ./action-rs/ /build/app/
RUN cargo build --release

FROM rust:1.84-slim@sha256:0ec205a9abb049604cb085f2fdf7630f1a31dad1f7ad4986154a56501fb7ca77

RUN groupadd -r ctf && \
    useradd -m -r -g ctf ctf && \
    mkdir /app/ && \
    chown -R ctf:ctf /app/

COPY --from=builder --chmod=755 /build/app/target/release/action /app/
COPY --from=builder /build/app/conf/ /app/conf/
COPY --chmod=755 ./files/docker-entrypoint.sh /docker-entrypoint.sh

EXPOSE 8000

WORKDIR /app/

ENTRYPOINT [ "/docker-entrypoint.sh" ]
