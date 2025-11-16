# ============================================
# Stage 1: cargo-chef for SAFT dependencies
# ============================================
FROM rust:1.85 AS chef
RUN cargo install cargo-chef
WORKDIR /build

# ============================================
# Stage 2: Prepare Rust dependencies recipe
# ============================================
FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY packages ./packages
COPY pallets ./pallets
COPY runtime ./runtime
COPY node ./node
RUN cargo chef prepare --recipe-path recipe.json

# ============================================
# Stage 3: Build Rust dependencies (CACHED)
# ============================================
FROM chef AS rust-deps
COPY --from=planner /build/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# ============================================
# Stage 4: Build SAFT binary
# ============================================
FROM chef AS saft-builder
COPY Cargo.toml Cargo.lock ./
COPY packages ./packages
COPY pallets ./pallets
COPY runtime ./runtime
COPY node ./node

# Copy cached dependencies
COPY --from=rust-deps /build/target target
COPY --from=rust-deps /usr/local/cargo /usr/local/cargo

# Build only SAFT
WORKDIR /build/packages/saft-enhanced
RUN cargo build --release --bin saft

# ============================================
# Stage 5: Install Node.js dependencies (CACHED)
# ============================================
FROM node:18-alpine AS node-deps
WORKDIR /app

# Install pnpm globally
RUN npm install -g pnpm@8

# Copy only package files first (for better caching)
COPY packages/web-dashboard/package.json packages/web-dashboard/pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile --prod=false

# ============================================
# Stage 6: Build Next.js app
# ============================================
FROM node:18-alpine AS node-builder
WORKDIR /app

# Copy dependencies from previous stage
COPY --from=node-deps /app/node_modules ./node_modules
COPY --from=node-deps /app/package.json ./package.json

# Copy source code
COPY packages/web-dashboard .

# Build Next.js app
RUN npm install -g pnpm@8 && pnpm build

# ============================================
# Stage 7: Production runtime image
# ============================================
FROM node:18-alpine
WORKDIR /app

# Install curl for healthcheck
RUN apk add --no-cache curl

# Copy Next.js build
COPY --from=node-builder /app/.next ./.next
COPY --from=node-builder /app/public ./public
COPY --from=node-builder /app/package.json ./package.json
COPY --from=node-builder /app/node_modules ./node_modules

# Copy SAFT binary for static analysis
COPY --from=saft-builder /build/target/release/saft /usr/local/bin/saft

# Create non-root user for security
RUN addgroup -g 1000 nexus && \
    adduser -D -u 1000 -G nexus nexus && \
    chown -R nexus:nexus /app
USER nexus

ENV NODE_ENV=production
ENV PORT=3000
ENV SAFT_BINARY_PATH=/usr/local/bin/saft

EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:3000/api/health || exit 1

CMD ["npm", "start"]