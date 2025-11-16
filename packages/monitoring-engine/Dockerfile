# Dockerfile for Monitoring Engine
# Multi-stage build to keep final image small

FROM rust:1.88-bookworm as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    libprotobuf-dev \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# Copy ALL workspace files (workspace requires all members)
COPY Cargo.toml Cargo.lock ./
COPY packages ./packages

# Build release binary
RUN cargo build --release --package monitoring-engine

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /build/target/release/monitoring-engine /usr/local/bin/monitoring-engine

# Copy schema for reference
COPY --from=builder /build/packages/monitoring-engine/schema.sql /schema.sql

EXPOSE 8080

ENV RUST_LOG=monitoring_engine=info,sqlx=warn
ENV API_PORT=8080

HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

CMD ["monitoring-engine"]
