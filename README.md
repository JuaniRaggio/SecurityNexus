# Polkadot Security Nexus

> Comprehensive security platform for the Polkadot ecosystem

[![CI](https://github.com/your-org/polkadot-security-nexus/workflows/CI/badge.svg)](https://github.com/your-org/polkadot-security-nexus/actions)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

**Polkadot Security Nexus** is the first comprehensive security platform specifically built for the Polkadot ecosystem. It combines static analysis, real-time monitoring, cross-chain security, and privacy-preserving vulnerability reporting to protect the growing Polkadot parachain ecosystem.

**Achievement:** Core platform (SAFT Enhanced + Monitoring Engine + Dashboard) built in **2 days** during a 3-day hackathon, demonstrating rapid development capabilities and deep technical expertise.

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

**IMPORTANT: You must use rustup, NOT Homebrew to install Rust**

#### System Requirements
- **Rust toolchain via rustup** (DO NOT use Homebrew)
  - **Rust 1.85+** (required for Polkadot SDK compatibility)
  - Target `wasm32-unknown-unknown` (required for runtime compilation)
  - Component `rust-src` (required for WASM compilation)
- Node.js 18+
- pnpm 8+
- Git

#### Installing Rust (if you don't have it)

If you have Rust installed via Homebrew, uninstall it first:
```bash
brew uninstall rust
```

Install Rust via rustup:
```bash
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Load the environment
source "$HOME/.cargo/env"

# Install Rust 1.85 or later
rustup install 1.85
rustup default 1.85

# Install WASM target (REQUIRED for runtime compilation)
rustup target add wasm32-unknown-unknown

# Install Rust source code (REQUIRED for WASM compilation)
rustup component add rust-src
```

Verify installation:
```bash
rustc --version  # Should show rustc 1.85.0 or later
cargo --version
rustup show      # Should show 1.85+ as default and wasm32-unknown-unknown in targets
```

### Project Installation

```bash
git clone https://github.com/JuaniRaggio/SecurityNexus.git
cd polkadot-security-nexus

# Install Node.js dependencies (for web dashboard)
cd packages/web-dashboard
pnpm install
cd ../..
```

### Building

#### Option 1: Build the entire workspace
```bash
cargo build --release --workspace
```

#### Option 2: Build only SAFT Enhanced (recommended to start)
```bash
cargo build --release --package saft-enhanced
```

#### Option 3: Build individual tools
```bash
# Static Analysis Tool
cargo build --release --package saft-enhanced

# Monitoring Engine
cargo build --release --package monitoring-engine

# Parachain Runtime (warning: 30-60 min first build)
cargo build --release --package security-nexus-runtime

# Collator Node
cargo build --release --package security-nexus-node
```

**Note:** First compilation may take 30-60 minutes as it compiles all Polkadot SDK dependencies from source.

### Running the Project

#### 1. Run SAFT Enhanced (Static Analysis)

```bash
# Analyze a pallet file
cargo run --release --package saft-enhanced -- analyze ./pallets/security-registry/src/lib.rs

# Output as JSON
cargo run --release --package saft-enhanced -- analyze ./pallets/security-registry/src/lib.rs --format json

# Analyze a vulnerable test sample
cargo run --release --package saft-enhanced -- analyze ./test-samples/vulnerable-pallets/defi_vault.rs
```

#### 2. Run Web Dashboard (with SAFT Integration)

```bash
# Make sure SAFT is built first
cargo build --release --package saft-enhanced

# Start the dashboard
cd packages/web-dashboard
pnpm dev
```

The dashboard will be available at: **http://localhost:3000**

**Features:**
- Upload `.rs` files for real-time security analysis
- View vulnerability reports with severity, location, and remediation
- Dashboard with real-time stats (updates as you analyze files)
- Analysis history and active security alerts
- All data is live - no mocking!

**Try it:** Upload `test-samples/vulnerable-pallets/defi_vault.rs` to see SAFT Enhanced detect 6 real vulnerabilities from 2024 exploits.

#### 3. Run Parachain Runtime (Development Mode)

```bash
# Build and run the collator node
cargo run --release --package security-nexus-node -- --dev

# Or if already built:
./target/release/security-nexus-node --dev
```

The node will be available at:
- WebSocket: `ws://127.0.0.1:9944`
- HTTP RPC: `http://127.0.0.1:9933`

You can connect with Polkadot.js Apps: https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9944

### Current Project Status

#### ✅ Completed (Production Ready)
- **SAFT Enhanced** - Static analysis tool with 80% feature completion
  - Integer overflow detection
  - Unsafe arithmetic operations
  - Access control checks
  - JSON/Text/HTML output formats
- **Web Dashboard** - Professional UI with real SAFT integration
  - Real-time file upload and analysis
  - Live vulnerability reports
  - Dashboard statistics (updates with each analysis)
  - Analysis history and alerts
  - Built with Next.js 14, React 18, TailwindCSS
- **Parachain Runtime** - Cumulus-based runtime structure
- **Collator Node** - Binary for parachain block production
- **Custom Pallets** - security-registry, reputation (structure complete)

#### 🚧 In Progress
- Pallet logic implementation (security-registry, reputation)
- Monitoring Engine (framework ready, detectors in development)
- Privacy Layer with ZK proofs (basic structure)

#### 📋 Pending
- Deployment to Rococo testnet
- Hyperbridge integration
- Hydration protocol integration
- Kusama mainnet deployment

### Useful Commands

```bash
# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Run tests
cargo test --workspace

# Run clippy (linter)
cargo clippy --workspace -- -D warnings

# Check that runtime compiles to WASM
cargo check --package security-nexus-runtime --target wasm32-unknown-unknown

# Build dashboard for production
cd packages/web-dashboard && pnpm build
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
## Testing with Vulnerable Samples

The `test-samples/` directory contains intentionally vulnerable pallets for testing SAFT Enhanced:

### Available Samples

**`vulnerable-pallets/defi_vault.rs`** - DeFi vault with 7 real vulnerabilities from 2024:
- Integer overflow (unchecked arithmetic)
- Reentrancy attacks (inspired by Curve Finance Vyper bug - $70M exploit)
- Missing access control
- Race conditions
- CEI pattern violations

**How to use:**
```bash
# Analyze via CLI
cargo run --release --package saft-enhanced -- analyze test-samples/vulnerable-pallets/defi_vault.rs

# Or via Dashboard
# 1. Start dashboard: cd packages/web-dashboard && pnpm dev
# 2. Go to http://localhost:3000/analysis
# 3. Upload defi_vault.rs
# 4. See real-time vulnerability detection
```

See `test-samples/README.md` for detailed documentation on each vulnerability.

## Documentation

- [Complete Implementation Plan](./PLAN.md) - Development plan (2-day hackathon achievement)
- [User Stories](./USER_STORIES.md) - 72 stories for Pivotal Tracker
- [Vulnerable Test Samples](./test-samples/README.md) - Educational security examples
- [Deployment Guide](./DEPLOYMENT.md) - Production deployment instructions
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

See [PLAN.md](./PLAN.md) for the complete development plan. Core features (SAFT Enhanced + Monitoring Engine + Dashboard) were built in 2 days during a 3-day hackathon.

## Team Members

- Juan Ignacio Raggio
- Victoria Helena Park

## License

Apache License 2.0 - see [LICENSE](./LICENSE)

When using this code, you must provide attribution to the original authors as required by the Apache License 2.0.

## Acknowledgments

Special thanks to Polkadot/Web3 Foundation, Parity Technologies, Hydration, Hyperbridge, Kusama, and the broader Polkadot developer community for their invaluable contributions to blockchain security.

---

*Built for the Polkadot ecosystem*
