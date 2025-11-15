# Polkadot Security Nexus

> Comprehensive security platform for the Polkadot ecosystem

[![CI](https://github.com/your-org/polkadot-security-nexus/workflows/CI/badge.svg)](https://github.com/your-org/polkadot-security-nexus/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Polkadot Security Nexus** is the first comprehensive security platform specifically built for the Polkadot ecosystem. It combines static analysis, real-time monitoring, cross-chain security, and privacy-preserving vulnerability reporting to protect the growing Polkadot parachain ecosystem.

## Problem

- **$474M lost in DeFi in 2024** alone
- **Zero specialized tools** for auditing Substrate/FRAME pallets
- **Scarcity of auditors** capable of reviewing Substrate code
- **Lack of real-time monitoring** for parachain security

## Solution

A multi-layer security platform that provides:

1. **Prevention** - Static analysis for FRAME pallets (SAFT Enhanced)
2. **Detection** - Real-time monitoring of mempool and attacks
3. **Response** - Privacy-preserving vulnerability reporting with ZK proofs
4. **Cross-chain** - Unified security monitoring via Hyperbridge
5. **DeFi Security** - Specialized monitoring for Hydration and other DeFi protocols

## Key Features

### 1. SAFT Enhanced
Static analysis for FRAME pallets - Detects vulnerabilities before deployment

### 2. Real-Time Monitoring
Mempool monitoring and attack pattern detection with automated alerts

### 3. Privacy Layer (ZKP)
Zero-knowledge proofs for private vulnerability reporting and bug bounties

### 4. Cross-Chain Security
Multi-chain monitoring via Hyperbridge with state proof verification

### 5. DeFi Security
Specialized monitoring for Hydration Omnipool and lending protocols

### 6. Web Dashboard
Professional interface with real-time updates and comprehensive API

## Quick Start

### Prerequisites

**IMPORTANTE: Debes usar rustup, NO Homebrew para instalar Rust**

#### Requisitos del Sistema
- **Rust toolchain via rustup** (NO usar Homebrew)
  - **Rust 1.81** (REQUERIDO - versiones más nuevas causan errores de compilación)
  - Target `wasm32-unknown-unknown` (requerido para runtime)
  - Componente `rust-src` (requerido para compilación WASM)
- Node.js 18+
- pnpm 8+
- Git

#### Instalación de Rust (si no lo tienes)

Si tienes Rust instalado via Homebrew, primero desinstálalo:
```bash
brew uninstall rust
```

Instala Rust via rustup:
```bash
# Instalar rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Cargar el entorno
source "$HOME/.cargo/env"

# IMPORTANTE: Instalar Rust 1.81 específicamente
rustup install 1.81
rustup default 1.81

# Instalar target WASM (REQUERIDO para compilar runtime)
rustup target add wasm32-unknown-unknown

# Instalar código fuente de Rust (REQUERIDO para compilar runtime a WASM)
rustup component add rust-src
```

Verifica la instalación:
```bash
rustc --version  # Debe mostrar rustc 1.81.0
cargo --version
rustup show      # Debe mostrar 1.81 como default y wasm32-unknown-unknown en targets
```

### Instalación del Proyecto

```bash
git clone https://github.com/JuaniRaggio/SecurityNexus.git
cd polkadot-security-nexus

# Instalar dependencias de Node.js (para web dashboard)
pnpm install
```

### Compilación

#### Opción 1: Compilar todo el workspace
```bash
cargo build --release --workspace
```

#### Opción 2: Compilar solo el runtime (parachain)
```bash
cargo build --release --package security-nexus-runtime
```

#### Opción 3: Compilar solo el node (collator)
```bash
cargo build --release --package security-nexus-node
```

#### Opción 4: Compilar herramientas individuales
```bash
# SAFT Enhanced (análisis estático)
cargo build --release --package saft-enhanced

# Monitoring Engine
cargo build --release --package monitoring-engine
```

**Nota:** La primera compilación puede tardar 30-60 minutos ya que compila todas las dependencias del Polkadot SDK desde el código fuente.

### Ejecución

#### 1. Ejecutar el Runtime en modo Development

```bash
# Compilar y ejecutar el collator node
cargo run --release --package security-nexus-node -- --dev

# O si ya compilaste:
./target/release/security-nexus-node --dev
```

El nodo estará disponible en:
- WebSocket: `ws://127.0.0.1:9944`
- HTTP RPC: `http://127.0.0.1:9933`

Puedes conectarte con Polkadot.js Apps: https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9944

#### 2. Ejecutar SAFT Enhanced (Análisis Estático)

```bash
# Analizar un pallet
cargo run --release --package saft-enhanced -- analyze ./pallets/security-registry

# Con output en JSON
cargo run --release --package saft-enhanced -- analyze ./pallets/security-registry --format json

# Con output en HTML
cargo run --release --package saft-enhanced -- analyze ./pallets/security-registry --format html -o report.html
```

#### 3. Ejecutar Web Dashboard

```bash
cd packages/web-dashboard
pnpm dev
```

El dashboard estará disponible en http://localhost:3000

### Estado Actual del Proyecto

#### Completado (Listo para usar)
- Runtime de parachain con Cumulus
- Collator node binary
- Estructura completa de pallets (security-registry, reputation)
- SAFT Enhanced - Análisis estático funcional
- Web Dashboard UI (con datos mock)
- Integración XCM básica

#### En Progreso
- Implementación de lógica en pallets personalizados
- Monitoring Engine (framework listo, detectores en desarrollo)
- Privacy Layer con ZK proofs (estructura básica)

#### Pendiente
- Deployment a Rococo testnet
- Integración con Hyperbridge
- Integración con Hydration
- Deployment a Kusama mainnet

### Comandos Útiles

```bash
# Limpiar build artifacts
cargo clean

# Actualizar dependencias
cargo update

# Ejecutar tests
cargo test --workspace

# Ejecutar clippy (linter)
cargo clippy --workspace -- -D warnings

# Verificar que el runtime compila para WASM
cargo check --package security-nexus-runtime --target wasm32-unknown-unknown
```

<!-- ## Project Structure -->
<!--  -->
<!-- ``` -->
<!-- polkadot-security-nexus/ -->
<!-- ├── packages/ -->
<!-- │   ├── saft-enhanced/              # Rust - Static analysis -->
<!-- │   │   ├── src/ -->
<!-- │   │   │   ├── parser/            # FRAME pallet parser -->
<!-- │   │   │   ├── analyzers/         # Vulnerability detectors -->
<!-- │   │   │   ├── rules/             # Security rules -->
<!-- │   │   │   └── cli/               # CLI tool -->
<!-- │   │   ├── tests/ -->
<!-- │   │   └── Cargo.toml -->
<!-- │   │ -->
<!-- │   ├── monitoring-engine/          # Rust - Real-time monitoring -->
<!-- │   │   ├── src/ -->
<!-- │   │   │   ├── mempool/           # Mempool monitoring -->
<!-- │   │   │   ├── detectors/         # Pattern matching -->
<!-- │   │   │   ├── alerts/            # Alert system -->
<!-- │   │   │   └── api/               # REST API -->
<!-- │   │   ├── tests/ -->
<!-- │   │   └── Cargo.toml -->
<!-- │   │ -->
<!-- │   ├── privacy-layer/              # Rust + ink! - ZKP -->
<!-- │   │   ├── circuits/              # ZK circuits (arkworks) -->
<!-- │   │   ├── contracts/             # ink! smart contracts -->
<!-- │   │   ├── src/ -->
<!-- │   │   │   ├── zkp/               # ZK proof generation/verification -->
<!-- │   │   │   ├── credentials/       # Verifiable credentials -->
<!-- │   │   │   └── marketplace/       # Bug bounty logic -->
<!-- │   │   ├── tests/ -->
<!-- │   │   └── Cargo.toml -->
<!-- │   │ -->
<!-- │   ├── hyperbridge-integration/    # Rust - Cross-chain -->
<!-- │   │   ├── src/ -->
<!-- │   │   │   ├── ismp/              # ISMP protocol integration -->
<!-- │   │   │   ├── state_proofs/      # State proof verification -->
<!-- │   │   │   └── multi_chain/       # Multi-chain monitoring -->
<!-- │   │   ├── tests/ -->
<!-- │   │   └── Cargo.toml -->
<!-- │   │ -->
<!-- │   ├── hydration-module/           # Rust + TypeScript -->
<!-- │   │   ├── rust/ -->
<!-- │   │   │   ├── src/ -->
<!-- │   │   │   │   ├── omnipool/      # Omnipool monitoring -->
<!-- │   │   │   │   ├── lending/       # Lending protocol analysis -->
<!-- │   │   │   │   └── circuit_breaker/ -->
<!-- │   │   │   └── Cargo.toml -->
<!-- │   │   └── ts/ -->
<!-- │   │       └── integration/       # Hydration API integration -->
<!-- │   │ -->
<!-- │   ├── web-dashboard/              # TypeScript/React -->
<!-- │   │   ├── src/ -->
<!-- │   │   │   ├── components/ -->
<!-- │   │   │   ├── pages/ -->
<!-- │   │   │   ├── hooks/ -->
<!-- │   │   │   ├── api/               # API client -->
<!-- │   │   │   └── utils/ -->
<!-- │   │   ├── public/ -->
<!-- │   │   ├── tests/ -->
<!-- │   │   ├── package.json -->
<!-- │   │   └── tsconfig.json -->
<!-- │   │ -->
<!-- │   └── api-server/                 # Node.js/TypeScript -->
<!-- │       ├── src/ -->
<!-- │       │   ├── routes/ -->
<!-- │       │   ├── controllers/ -->
<!-- │       │   ├── services/ -->
<!-- │       │   ├── middleware/ -->
<!-- │       │   └── db/                # PostgreSQL schemas -->
<!-- │       ├── tests/ -->
<!-- │       ├── package.json -->
<!-- │       └── tsconfig.json -->
<!-- │ -->
<!-- ├── pallets/                        # Custom Substrate pallets -->
<!-- │   ├── security-registry/         # On-chain audit registry -->
<!-- │   └── reputation/                # Reputation system -->
<!-- │ -->
<!-- ├── scripts/ -->
<!-- │   ├── deploy/                    # Deployment scripts -->
<!-- │   ├── seed/                      # Data seeding for testing -->
<!-- │   └── benchmarks/                # Performance benchmarks -->
<!-- │ -->
<!-- ├── docs/ -->
<!-- │   ├── architecture/              # Architecture diagrams -->
<!-- │   ├── api/                       # API documentation -->
<!-- │   ├── user-guide/                # User documentation -->
<!-- │   └── integration/               # Integration guides -->
<!-- │ -->
<!-- ├── docker/ -->
<!-- │   ├── docker-compose.yml -->
<!-- │   ├── Dockerfile.rust -->
<!-- │   ├── Dockerfile.node -->
<!-- │   └── Dockerfile.dashboard -->
<!-- │ -->
<!-- ├── .github/ -->
<!-- │   └── workflows/ -->
<!-- │       ├── ci.yml -->
<!-- │       ├── deploy-kusama.yml -->
<!-- │       └── security-audit.yml -->
<!-- │ -->
<!-- └── LICENSE -->
<!-- ``` -->
<!--  -->
## Documentation

- [Complete Implementation Plan](./PLAN.md) - Detailed 12-week roadmap
- [User Stories](./USER_STORIES.md) - 72 stories for Pivotal Tracker
- [Architecture Docs](./docs/architecture/)
- [API Documentation](./docs/api/)
- [User Guide](./docs/user-guide/)

## Sponsor Integration

- **Parity Technologies** - Substrate/FRAME/ink!
- **Kusama** - Canary deployment and testing
- **Hydration** - Omnipool monitoring + HOLLAR integration
- **Hyperbridge** - Cross-chain security via ISMP

## Development

```bash
# Run tests
cargo test --workspace
pnpm test

# Linting
cargo clippy --workspace -- -D warnings
pnpm lint

# Build for production
cargo build --release --workspace
pnpm build
```

## Technology Stack

**Backend (Rust):** Substrate/FRAME, ink!, arkworks (ZKP), tokio, actix-web

**Frontend (TypeScript):** React 18, Next.js 14, TailwindCSS, Polkadot.js

**Infrastructure:** PostgreSQL, Redis, Docker, GitHub Actions

## Roadmap

See [PLAN.md](./PLAN.md) for the complete 12-week implementation plan with detailed phases, user stories, and success metrics.

## Team Members

- Juan Ignacio Raggio
- Victoria Helena Park

## License

MIT License - see [LICENSE](./LICENSE)

## Acknowledgments

Special thanks to Polkadot/Web3 Foundation, Parity Technologies, Hydration, Hyperbridge, Kusama, and the broader Polkadot developer community for their invaluable contributions to blockchain security.

---

*Built for the Polkadot ecosystem*
