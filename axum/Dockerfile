FROM rust:1.71.1-slim-bullseye AS build

WORKDIR /app

COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./migrations ./migrations
COPY ./src ./src
COPY ./.sqlx ./.sqlx

ARG SQLX_OFFLINE=1

RUN --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/local/rustup \set -eux; \
    rustup install stable; \
    apt update; \
    apt install --yes --no-install-recommends pkg-config libssl-dev; \
    cargo build --release; \
    objcopy --compress-debug-sections target/release/axum ./axum


FROM debian:11.3-slim as deploy

RUN set -eux; \
    export DEBIAN_FRONTEND=noninteractive; \
    apt update; \
    apt install --yes --no-install-recommends pkg-config libssl-dev bind9-dnsutils iputils-ping iproute2 curl ca-certificates htop; \
    apt clean autoclean; \
    apt autoremove --yes; \
    rm -rf /var/lib/{apt,dpkg,cache,log}/;

WORKDIR /app

COPY --from=build /app/axum .
COPY --from=build /app/migrations .
COPY --from=build /app/.sqlx .

RUN chmod +x ./axum

CMD ["./axum"]
