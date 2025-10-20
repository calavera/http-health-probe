# syntax=docker/dockerfile:1

FROM --platform=$BUILDPLATFORM rust:alpine AS builder

RUN apk add --no-cache zig musl-dev

RUN cargo install --locked cargo-zigbuild
WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

# Determine the target based on BUILDPLATFORM
ARG TARGETPLATFORM
RUN case $TARGETPLATFORM in \
    "linux/amd64") TARGET="x86_64-unknown-linux-musl" ;; \
    "linux/arm64") TARGET="aarch64-unknown-linux-musl" ;; \
    *) echo "Unsupported platform: $TARGETPLATFORM" && exit 1 ;; \
    esac && \
    echo "Building for target: $TARGET" && \
    cargo zigbuild --target $TARGET --release

# Copy the built binary to a known location
RUN case $TARGETPLATFORM in \
    "linux/amd64") cp /app/target/x86_64-unknown-linux-musl/release/http-health-probe /http-healt-probe ;; \
    "linux/arm64") cp /app/target/aarch64-unknown-linux-musl/release/http-health-probe /http-healt-probe ;; \
    esac

FROM scratch

COPY --from=builder /http-healt-probe /http-healt-probe

ENTRYPOINT ["/http-healt-probe"]
