# SecurityNexus - Quick Start Guide

Get the complete system up and running in 5 minutes.

## 1. Prerequisites

- Docker and Docker Compose installed
- 4GB RAM available
- Free ports: 80, 443, 3000, 5432, 8080

## 2. Start the System (1 Command)

```bash
# From project root
docker-compose up -d
```

This will start:
- TimescaleDB (database)
- Monitoring Engine (Rust backend)
- Web Dashboard (Next.js frontend)
- Nginx (reverse proxy)

## 3. Verify All Services Are Running

```bash
docker-compose ps
```

You should see all services with "Up (healthy)" status:

```
NAME                               STATUS          PORTS
security-nexus-timescaledb        Up (healthy)    0.0.0.0:5432->5432/tcp
security-nexus-monitoring-engine  Up (healthy)    0.0.0.0:8080->8080/tcp
security-nexus-dashboard          Up (healthy)    0.0.0.0:3000->3000/tcp
security-nexus-nginx              Up (healthy)    0.0.0.0:80->80/tcp
```

## 4. Access the Dashboard

Open your browser at:

```
http://localhost:3000
```

Or through Nginx:

```
http://localhost
```

## 5. Test API Endpoints

```bash
# Health check
curl http://localhost:8080/api/health | jq .

# System statistics
curl http://localhost:8080/api/stats | jq .

# Active detectors
curl http://localhost:8080/api/detectors | jq .

# Alerts
curl http://localhost:8080/api/alerts | jq .

# Analytics - Attack Trends (last 24 hours)
curl "http://localhost:8080/api/analytics/attack-trends?hours=24" | jq .

# Analytics - Detector Stats
curl "http://localhost:8080/api/analytics/detector-stats?hours=24" | jq .

# ML Features
curl "http://localhost:8080/api/analytics/ml-features?limit=20" | jq .
```

## 6. Navigate the Dashboard

The dashboard includes the following pages:

1. **Dashboard** (`/`) - System overview
2. **Static Analysis** (`/analysis`) - Static analysis with SAFT
3. **Monitoring** (`/monitoring`) - Real-time monitoring
4. **Alerts** (`/alerts`) - Alerts and detections
5. **Hyperbridge** (`/hyperbridge`) - Cross-chain monitoring
6. **Hydration** (`/hydration`) - DeFi monitoring
7. **Analytics** (`/analytics`) - Analytics and ML insights
8. **Privacy** (`/privacy`) - Privacy layer (coming soon)
9. **Data** (`/data`) - Data and exports
10. **Settings** (`/settings`) - Configuration

## 7. View Real-Time Logs

```bash
# Logs from all services
docker-compose logs -f

# Monitoring engine logs
docker-compose logs -f monitoring-engine

# Dashboard logs
docker-compose logs -f dashboard

# Database logs
docker-compose logs -f timescaledb
```

## 8. Query Test Data

```bash
# Connect to database
docker exec -it security-nexus-timescaledb psql -U nexus -d security_nexus
```

```sql
-- View recent transactions
SELECT * FROM transactions ORDER BY timestamp DESC LIMIT 10;

-- View recent detections
SELECT * FROM detections ORDER BY timestamp DESC LIMIT 10;

-- View detector statistics
SELECT * FROM detector_stats_hourly ORDER BY hour DESC LIMIT 20;

-- View ML features
SELECT * FROM ml_features ORDER BY timestamp DESC LIMIT 10;
```

## 9. Export Data

```bash
# Export as JSON
curl "http://localhost:8080/api/export/json?hours=24" -o detections.json

# Export as CSV
curl "http://localhost:8080/api/export/csv?hours=24" -o detections.csv
```

## 10. Switch Chains

By default, the system monitors Westend. To change:

```bash
# Edit .env
nano .env

# Change these lines:
WS_ENDPOINT=wss://polkadot.api.onfinality.io/public-ws
CHAIN_NAME=Polkadot

# Restart services
docker-compose restart monitoring-engine
```

Or use the API:

```bash
curl -X POST http://localhost:8080/api/chains/switch \
  -H "Content-Type: application/json" \
  -d '{"chain_name": "polkadot"}'

# Then restart
docker-compose restart monitoring-engine
```

## 11. Stop the System

```bash
# Stop all services (keeps data)
docker-compose stop

# Stop and remove containers (keeps volumes/data)
docker-compose down

# Stop and remove EVERYTHING (including data)
docker-compose down -v
```

## 12. Restart the System

```bash
# Restart all services
docker-compose restart

# Restart a specific service
docker-compose restart monitoring-engine
```

## 13. View Database Status

```bash
# Connect to TimescaleDB
docker exec -it security-nexus-timescaledb psql -U nexus -d security_nexus

# List tables
\dt

# View hypertables
SELECT * FROM timescaledb_information.hypertables;

# View continuous aggregates
SELECT * FROM timescaledb_information.continuous_aggregates;

# Exit
\q
```

## 14. Troubleshooting

### Service is not healthy

```bash
# View logs for problematic service
docker-compose logs monitoring-engine

# Restart the service
docker-compose restart monitoring-engine
```

### Port already in use

```bash
# Edit .env and change ports
API_PORT=8081

# Rebuild
docker-compose up -d
```

### Database won't connect

```bash
# Verify TimescaleDB is healthy
docker-compose ps timescaledb

# View logs
docker-compose logs timescaledb

# Restart
docker-compose restart timescaledb
```

### Dashboard won't load

```bash
# Check logs
docker-compose logs dashboard

# Rebuild
docker-compose up -d --build dashboard
```

## 15. Local Development (without Docker)

If you prefer to run without Docker:

### Backend (Monitoring Engine)

```bash
cd packages/monitoring-engine

# Create .env with DATABASE_URL
echo "DATABASE_URL=postgresql://nexus:nexus_dev_password@localhost:5432/security_nexus" > .env

# Run
cargo run --release
```

### Frontend (Dashboard)

```bash
cd packages/web-dashboard

# Install dependencies
pnpm install

# Run in development
pnpm dev
```

### Database

```bash
# Only start TimescaleDB
docker-compose up -d timescaledb
```

---

## Quick Command Reference

```bash
# Start everything
docker-compose up -d

# View status
docker-compose ps

# View logs
docker-compose logs -f

# Stop everything
docker-compose down

# Restart
docker-compose restart

# Rebuild
docker-compose up -d --build

# Clean everything
docker-compose down -v
```

---

For more details see:
- `TESTING.md` - Complete testing guide
- `IMPLEMENTATION.md` - Complete technical documentation
- `README.md` - General project documentation
