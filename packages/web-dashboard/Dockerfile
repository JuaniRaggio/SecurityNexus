# Stage: build saft (Rust)
FROM rust:1.85 as saft-builder
WORKDIR /saft
COPY ../saft-enhanced ./saft-enhanced
WORKDIR /saft/saft-enhanced
RUN cargo build --release --bin saft || true
# If binary name differs adjust path

# Stage: build next app
FROM node:18-alpine AS node-builder
WORKDIR /app
COPY package.json pnpm-lock.yaml ./
RUN npm install -g pnpm && pnpm install --frozen-lockfile
COPY . .
RUN pnpm build

# Final image
FROM node:18-alpine
WORKDIR /app

# copy next build
COPY --from=node-builder /app/.next ./.next
COPY --from=node-builder /app/public ./public
COPY --from=node-builder /app/package.json ./
COPY --from=node-builder /app/node_modules ./node_modules

# copy saft binary
COPY --from=saft-builder /saft/saft-enhanced/target/release/saft /usr/local/bin/saft

ENV NODE_ENV=production
ENV PORT=3000
ENV SAFT_BINARY_PATH=/usr/local/bin/saft

EXPOSE 3000
CMD ["npm", "start"]