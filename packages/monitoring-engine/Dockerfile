# Build stage
FROM rust:1.85 AS builder

WORKDIR /app

# Copiamos los archivos raíz del workspace
COPY Cargo.toml Cargo.lock ./

# Copiamos TODAS las carpetas que el workspace necesita
COPY packages ./packages
COPY pallets ./pallets
COPY runtime ./runtime
COPY node ./node

# Entramos al crate específico
WORKDIR /app/packages/monitoring-engine

# Build del binario real
RUN cargo build --release --bin monitoring-engine

# -----------------------------------------------------------
# Runtime image
# -----------------------------------------------------------
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiamos el binario final
COPY --from=builder /app/target/release/monitoring-engine /app/monitoring-engine

EXPOSE 8080
CMD ["./monitoring-engine"]
