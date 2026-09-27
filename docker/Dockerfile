# Multi-stage static build producing zero-dependency minimal binary (<20MB)
FROM rust:1.75-alpine AS builder

RUN apk add --no-cache musl-dev perl make pkgconfig

WORKDIR /usr/src/airmesh

# Copy workspace files
COPY Cargo.toml ./
COPY airmesh-core ./airmesh-core
COPY airmesh-daemon ./airmesh-daemon
COPY airmesh-cli ./airmesh-cli
COPY tests ./tests
COPY benches ./benches

# Build optimized release binaries
RUN cargo build --release -p airmesh-daemon -p airmesh-cli
RUN strip target/release/airmesh-daemon target/release/airmesh-cli

# Minimal runtime image
FROM alpine:3.19

RUN apk add --no-cache ca-certificates tzdata

WORKDIR /app
COPY --from=builder /usr/src/airmesh/target/release/airmesh-daemon /usr/local/bin/airmesh-daemon
COPY --from=builder /usr/src/airmesh/target/release/airmesh-cli /usr/local/bin/airmesh

VOLUME ["/shared", "/downloads", "/data"]

EXPOSE 47823/tcp
EXPOSE 47823/udp
EXPOSE 47824/udp

ENV RUST_LOG="info"

ENTRYPOINT ["airmesh-daemon", "--port", "47823", "--discovery-port", "47824", "--shared", "/shared", "--db", "/data/airmesh.db"]
