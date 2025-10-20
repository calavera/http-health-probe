# syntax=docker/dockerfile:1

FROM --platform=$BUILDPLATFORM rust:alpine AS builder
WORKDIR /app

RUN apk add --no-cache zig musl-dev curl

ARG TARGETPLATFORM
RUN case "$TARGETPLATFORM" in \
    "linux/arm64") echo "aarch64-unknown-linux-musl" > rust_target.txt ;; \
    "linux/amd64") echo "x86_64-unknown-linux-musl" > rust_target.txt ;; \
    *) exit 1 ;; \
    esac;
RUN rustup target add $(cat rust_target.txt)

ARG CARGO_ZIGBUILD_VERSION=0.20.1
RUN curl -sSOL https://github.com/rust-cross/cargo-zigbuild/releases/download/v${CARGO_ZIGBUILD_VERSION}/cargo-zigbuild-v${CARGO_ZIGBUILD_VERSION}.$(cat rust_target.txt).tar.gz && \
    tar -xzf cargo-zigbuild-v${CARGO_ZIGBUILD_VERSION}.$(cat rust_target.txt).tar.gz && \
    mv cargo-zigbuild /usr/local/bin/cargo-zigbuild && \
    rm -rf cargo-zigbuild* && \
    cargo-zigbuild --version

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo zigbuild --target $(cat rust_target.txt) --release
RUN cp /app/target/$(cat rust_target.txt)/release/http-health-probe /http-health-probe

FROM scratch

COPY --from=builder /http-health-probe /http-health-probe

ENTRYPOINT ["/http-health-probe"]
