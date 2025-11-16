# ============================================
# Stage 1: cargo-chef planner
# ============================================
FROM rust:1.88 AS chef
RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    clang \
    libclang-dev \
    cmake \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Set LIBCLANG_PATH for bindgen
ENV LIBCLANG_PATH=/usr/lib/llvm-14/lib

RUN cargo install cargo-chef
WORKDIR /app

# ============================================
# Stage 2: Prepare dependencies recipe
# ============================================
FROM chef AS planner
# Copy workspace structure (lightweight, just metadata)
COPY Cargo.toml Cargo.lock ./
COPY packages packages
COPY pallets pallets
COPY runtime runtime
COPY node node
RUN cargo chef prepare --recipe-path recipe.json --bin monitoring-engine

# ============================================
# Stage 3: Build dependencies (CACHED LAYER)
# ============================================
FROM chef AS builder-deps
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this layer will be cached
RUN cargo chef cook --release --recipe-path recipe.json

# ============================================
# Stage 4: Build application
# ============================================
FROM chef AS builder
# Copy workspace structure
COPY Cargo.toml Cargo.lock ./
COPY packages packages
COPY pallets pallets
COPY runtime runtime
COPY node node

# Copy cached dependencies from previous stage
COPY --from=builder-deps /app/target target
COPY --from=builder-deps /usr/local/cargo /usr/local/cargo

# Build ONLY monitoring-engine package (not the whole workspace)
WORKDIR /app
RUN cargo build --release -p monitoring-engine

# ============================================
# Stage 5: Runtime image (minimal)
# ============================================
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy only the compiled binary
COPY --from=builder /app/target/release/monitoring-engine /app/monitoring-engine

# Create non-root user for security
RUN useradd -m -u 1000 nexus && \
    chown -R nexus:nexus /app
USER nexus

EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/api/health || exit 1

CMD ["./monitoring-engine"]
