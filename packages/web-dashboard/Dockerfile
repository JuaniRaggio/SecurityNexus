# Stage 1: Build SAFT (Rust static analyzer)
FROM rust:1.85 AS saft-builder
WORKDIR /build

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY packages ./packages
COPY pallets ./pallets
COPY runtime ./runtime
COPY node ./node

# Build SAFT binary
WORKDIR /build/packages/saft-enhanced
RUN cargo build --release --bin saft

# Stage 2: Build Next.js app
FROM node:18-alpine AS node-builder
WORKDIR /app
COPY packages/web-dashboard/package.json packages/web-dashboard/pnpm-lock.yaml ./
RUN npm install -g pnpm@8 && pnpm install --frozen-lockfile
COPY packages/web-dashboard .
RUN pnpm build

# Final image
FROM node:18-alpine
WORKDIR /app

# copy next build
COPY --from=node-builder /app/.next ./.next
COPY --from=node-builder /app/public ./public
COPY --from=node-builder /app/package.json ./
COPY --from=node-builder /app/node_modules ./node_modules

# Copy SAFT binary for static analysis
COPY --from=saft-builder /build/target/release/saft /usr/local/bin/saft

ENV NODE_ENV=production
ENV PORT=3000
ENV SAFT_BINARY_PATH=/usr/local/bin/saft

EXPOSE 3000
CMD ["npm", "start"]