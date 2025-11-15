# Deployment Checklist - Production URLs

Esta guía te muestra exactamente qué cambiar cuando hagas el deploy a producción.

## 🎯 Resumen Rápido

**Buenas noticias:** La mayoría de las URLs ya están usando variables de entorno. Solo necesitas configurar 3 archivos principales.

---

## 📝 Archivos a Configurar

### 1. Dashboard - Variables de Entorno

**Archivo:** `packages/web-dashboard/.env.production` (crear nuevo)

```bash
# Production configuration
MONITORING_ENGINE_URL=https://api.security-nexus.io
NEXT_PUBLIC_API_URL=https://security-nexus.io
```

**Qué cambia:**
- `localhost:8080` → URL de tu servidor de monitoring engine
- `localhost:3000` → URL pública del dashboard

**Dónde se usa:**
- `src/app/api/monitoring/route.ts` - Ya lo lee de `process.env.MONITORING_ENGINE_URL`

---

### 2. Monitoring Engine - Variables de Entorno

**Archivo:** Variables de entorno del servidor (Docker, systemd, etc.)

```bash
# Substrate node endpoint
WS_ENDPOINT=wss://rpc.polkadot.io
# O para testnet: wss://rococo-rpc.polkadot.io

# Chain name
CHAIN_NAME=polkadot

# API server port
API_PORT=8080

# Logs
RUST_LOG=monitoring_engine=info,actix_web=info
```

**Qué cambia:**
- `ws://127.0.0.1:9944` → URL del nodo Substrate en producción
- `development` → Nombre real de la chain (polkadot, kusama, rococo, etc.)

**Dónde se usa:**
- `examples/api_server.rs:34-39` - Ya lo lee de variables de entorno
- `src/connection.rs` - Recibe el endpoint como parámetro

---

### 3. CORS Configuration

**Archivo:** `packages/monitoring-engine/src/api.rs`

**Actualmente (línea ~123):**
```rust
let cors = Cors::default()
    .allow_any_origin()  // ⚠️ CAMBIAR EN PRODUCCIÓN
    .allow_any_method()
    .allow_any_header();
```

**Cambiar a (producción):**
```rust
let cors = Cors::default()
    .allowed_origin("https://security-nexus.io")  // Tu dominio
    .allowed_methods(vec!["GET", "POST"])
    .allowed_headers(vec!["Content-Type", "Authorization"])
    .max_age(3600);
```

---

## 🚀 Configuración por Entorno

### Desarrollo (Local)
```bash
# Dashboard
MONITORING_ENGINE_URL=http://localhost:8080
NEXT_PUBLIC_API_URL=http://localhost:3000

# Monitoring Engine
WS_ENDPOINT=ws://127.0.0.1:9944
CHAIN_NAME=development
API_PORT=8080
```

### Staging
```bash
# Dashboard
MONITORING_ENGINE_URL=https://api-staging.security-nexus.io
NEXT_PUBLIC_API_URL=https://staging.security-nexus.io

# Monitoring Engine
WS_ENDPOINT=wss://rococo-rpc.polkadot.io
CHAIN_NAME=rococo
API_PORT=8080
```

### Production
```bash
# Dashboard
MONITORING_ENGINE_URL=https://api.security-nexus.io
NEXT_PUBLIC_API_URL=https://security-nexus.io

# Monitoring Engine
WS_ENDPOINT=wss://rpc.polkadot.io
CHAIN_NAME=polkadot
API_PORT=8080
```

---

## 🐳 Docker Deployment

### Opción 1: Docker Compose (Recomendado)

**Archivo:** `docker-compose.yml` (crear en root)

```yaml
version: '3.8'

services:
  monitoring-engine:
    build:
      context: ./packages/monitoring-engine
      dockerfile: Dockerfile
    environment:
      - WS_ENDPOINT=wss://rpc.polkadot.io
      - CHAIN_NAME=polkadot
      - API_PORT=8080
      - RUST_LOG=monitoring_engine=info
    ports:
      - "8080:8080"
    restart: unless-stopped

  dashboard:
    build:
      context: ./packages/web-dashboard
      dockerfile: Dockerfile
    environment:
      - MONITORING_ENGINE_URL=http://monitoring-engine:8080
      - NEXT_PUBLIC_API_URL=https://security-nexus.io
    ports:
      - "3000:3000"
    depends_on:
      - monitoring-engine
    restart: unless-stopped

  nginx:
    image: nginx:alpine
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
    ports:
      - "80:80"
      - "443:443"
    depends_on:
      - dashboard
      - monitoring-engine
    restart: unless-stopped
```

### Opción 2: Dockerfiles Individuales

**Monitoring Engine:** `packages/monitoring-engine/Dockerfile`
```dockerfile
FROM rust:1.85 as builder

WORKDIR /app
COPY . .

RUN cargo build --release --example api_server

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/examples/api_server /usr/local/bin/

ENV WS_ENDPOINT=ws://127.0.0.1:9944
ENV CHAIN_NAME=development
ENV API_PORT=8080
ENV RUST_LOG=monitoring_engine=info

EXPOSE 8080

CMD ["api_server"]
```

**Dashboard:** `packages/web-dashboard/Dockerfile`
```dockerfile
FROM node:18-alpine AS builder

WORKDIR /app

COPY package.json pnpm-lock.yaml ./
RUN npm install -g pnpm && pnpm install --frozen-lockfile

COPY . .
RUN pnpm build

FROM node:18-alpine

WORKDIR /app

COPY --from=builder /app/.next ./.next
COPY --from=builder /app/public ./public
COPY --from=builder /app/package.json ./
COPY --from=builder /app/node_modules ./node_modules

ENV NODE_ENV=production
ENV PORT=3000

EXPOSE 3000

CMD ["npm", "start"]
```

---

## 🌐 Nginx Reverse Proxy

**Archivo:** `nginx.conf`

```nginx
events {
    worker_connections 1024;
}

http {
    upstream dashboard {
        server dashboard:3000;
    }

    upstream monitoring_api {
        server monitoring-engine:8080;
    }

    server {
        listen 80;
        server_name security-nexus.io;

        # Dashboard
        location / {
            proxy_pass http://dashboard;
            proxy_http_version 1.1;
            proxy_set_header Upgrade $http_upgrade;
            proxy_set_header Connection 'upgrade';
            proxy_set_header Host $host;
            proxy_cache_bypass $http_upgrade;
        }

        # Monitoring Engine API
        location /api/ {
            proxy_pass http://monitoring_api/api/;
            proxy_http_version 1.1;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }
    }

    # SSL configuration (add your certificates)
    # server {
    #     listen 443 ssl http2;
    #     server_name security-nexus.io;
    #
    #     ssl_certificate /etc/nginx/ssl/cert.pem;
    #     ssl_certificate_key /etc/nginx/ssl/key.pem;
    #
    #     # Same location blocks as above
    # }
}
```

---

## ✅ Checklist de Deployment

### Pre-Deploy
- [ ] Crear `.env.production` en web-dashboard
- [ ] Configurar variables de entorno del servidor
- [ ] Actualizar CORS en `src/api.rs` con dominio real
- [ ] Crear Dockerfiles si usas Docker
- [ ] Configurar nginx.conf con tu dominio

### Build
- [ ] Dashboard: `cd packages/web-dashboard && pnpm build`
- [ ] Monitoring Engine: `cd packages/monitoring-engine && cargo build --release --example api_server`

### Deploy
- [ ] Subir archivos al servidor
- [ ] Configurar variables de entorno
- [ ] Iniciar servicios (Docker Compose o systemd)
- [ ] Configurar SSL/TLS (Let's Encrypt recomendado)

### Post-Deploy
- [ ] Verificar health check: `curl https://api.security-nexus.io/api/health`
- [ ] Verificar stats: `curl https://api.security-nexus.io/api/stats`
- [ ] Abrir dashboard: `https://security-nexus.io`
- [ ] Verificar que "Connected" badge aparece
- [ ] Verificar que bloques incrementan en tiempo real

---

## 🔒 Security Hardening

### 1. API Keys (Futuro)
Agregar autenticación a la API del monitoring engine:

```rust
// src/api.rs
#[actix_web::middleware::from_fn]
async fn api_key_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let api_key = req.headers().get("X-API-Key");

    if api_key != Some(&HeaderValue::from_static("your-secret-key")) {
        return Err(actix_web::error::ErrorUnauthorized("Invalid API key"));
    }

    next.call(req).await
}
```

### 2. Rate Limiting
Ya tienes `governor` en dependencies, implementar:

```rust
use governor::{Quota, RateLimiter};

let limiter = RateLimiter::direct(
    Quota::per_minute(nonzero!(100u32))
);
```

### 3. HTTPS Only
En producción, forzar HTTPS en nginx:

```nginx
server {
    listen 80;
    server_name security-nexus.io;
    return 301 https://$server_name$request_uri;
}
```

---

## 📊 Monitoring

### Health Checks
```bash
# Monitoring Engine
curl https://api.security-nexus.io/api/health

# Expected: {"status":"healthy","version":"0.1.0","uptime_seconds":X}
```

### Logs
```bash
# Docker
docker logs -f monitoring-engine
docker logs -f dashboard

# Systemd
journalctl -u monitoring-engine -f
journalctl -u dashboard -f
```

---

## 🆘 Troubleshooting

### Dashboard muestra "Disconnected"
1. Verificar que monitoring engine está corriendo
2. Verificar variable `MONITORING_ENGINE_URL` en dashboard
3. Verificar CORS en monitoring engine
4. Check logs del dashboard: `docker logs dashboard`

### CORS errors
1. Actualizar `src/api.rs` con dominio correcto
2. Rebuild monitoring engine
3. Restart servicio

### 502 Bad Gateway
1. Verificar que todos los servicios están corriendo
2. Verificar nginx upstream configuration
3. Verificar network connectivity entre containers

---

## 🎯 Quick Deploy Commands

```bash
# Build everything
pnpm build:all

# Docker Compose
docker-compose up -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f

# Stop
docker-compose down
```

---

**Resumen:** Solo necesitas cambiar 3 cosas principales:
1. `.env.production` en dashboard (2 variables)
2. Variables de entorno del servidor (3-4 variables)
3. CORS en `api.rs` (1 línea)

Todo lo demás ya está preparado para deployment!
