# Dockerfile for Web Dashboard
# Multi-stage build

FROM node:20-bookworm-slim as builder

# Install pnpm
RUN npm install -g pnpm@latest

WORKDIR /build

# Copy package files
COPY packages/web-dashboard/package.json packages/web-dashboard/pnpm-lock.yaml ./

# Install dependencies
RUN pnpm install --frozen-lockfile

# Copy source code
COPY packages/web-dashboard ./

# Build the application
RUN pnpm build

# Runtime stage
FROM node:20-bookworm-slim

RUN npm install -g pnpm@latest

WORKDIR /app

# Copy built application
COPY --from=builder /build/.next/standalone ./
COPY --from=builder /build/.next/static ./.next/static
COPY --from=builder /build/public ./public

EXPOSE 3000

ENV NODE_ENV=production
ENV PORT=3000

CMD ["node", "server.js"]
