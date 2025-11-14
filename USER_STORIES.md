# User Stories para Pivotal Tracker
## Polkadot Security Nexus

Este documento contiene todas las user stories para el desarrollo del proyecto, organizadas por épicas y features. Cada story sigue el formato estándar de Pivotal Tracker.

---

## EPIC 1: Infraestructura y Setup

### Story 1.1: Configurar Monorepo
**Story Type:** Feature
**Points:** 2

**As a** developer,
**I want** un monorepo configurado con Turborepo y Cargo workspace,
**So that** puedo gestionar múltiples paquetes Rust y TypeScript eficientemente.

**Acceptance Criteria:**
- Turbo.json configurado con tasks de build, test, lint
- Cargo.toml workspace con todos los paquetes Rust
- package.json root con scripts para gestión del monorepo
- .gitignore configurado para node_modules, target, etc.
- README.md con instrucciones de setup

**Tasks:**
- Crear Cargo.toml workspace
- Configurar turbo.json
- Setup package.json root
- Configurar .gitignore
- Escribir README de setup

---

### Story 1.2: Crear Estructura de Carpetas
**Story Type:** Feature
**Points:** 1

**As a** developer,
**I want** la estructura completa de carpetas del proyecto,
**So that** puedo organizar el código de manera consistente.

**Acceptance Criteria:**
- Todas las carpetas principales creadas (packages/, pallets/, scripts/, docs/, docker/)
- Cada paquete tiene su propia estructura interna
- .gitkeep en carpetas vacías para mantener en git

**Tasks:**
- Crear packages/ con subdirectorios
- Crear pallets/ para Substrate pallets
- Crear scripts/, docs/, docker/
- Añadir .gitkeep donde sea necesario

---

### Story 1.3: Configurar CI/CD Pipeline
**Story Type:** Feature
**Points:** 3

**As a** developer,
**I want** un pipeline de CI/CD automatizado,
**So that** el código se testea y valida automáticamente en cada commit.

**Acceptance Criteria:**
- GitHub Actions workflow configurado
- Tests de Rust ejecutándose (cargo test)
- Tests de TypeScript ejecutándose (jest)
- Linting (clippy, eslint, rustfmt)
- Build exitoso de todos los paquetes
- Notificaciones de fallos

**Tasks:**
- Crear .github/workflows/ci.yml
- Configurar jobs para Rust
- Configurar jobs para TypeScript
- Setup de caching para dependencies
- Configurar badges en README

---

### Story 1.4: Setup Docker para Development
**Story Type:** Feature
**Points:** 2

**As a** developer,
**I want** containers Docker para desarrollo local,
**So that** puedo ejecutar todos los servicios localmente de manera consistente.

**Acceptance Criteria:**
- docker-compose.yml con todos los servicios
- PostgreSQL container configurado
- Redis container configurado
- Substrate node container
- Todos los servicios inician con un comando
- Volúmenes persistentes configurados

**Tasks:**
- Crear docker-compose.yml
- Dockerfile para cada servicio
- Configurar networking entre containers
- Scripts de inicialización de DB
- Documentar cómo usar Docker

---

## EPIC 2: SAFT Enhanced (Static Analysis)

### Story 2.1: Parser de FRAME Pallets
**Story Type:** Feature
**Points:** 5

**As a** security auditor,
**I want** un parser que analice código de FRAME pallets,
**So that** puedo extraer el AST para análisis posterior.

**Acceptance Criteria:**
- Parser usando librería `syn` funcional
- Extracción de AST de pallets FRAME
- Identificación de macros FRAME (#[pallet], #[extrinsic], etc.)
- Visitor pattern para traversar AST
- Tests con 3+ pallets de ejemplo
- Errores claros para código inválido

**Tasks:**
- Setup de paquete saft-enhanced
- Implementar parser usando syn
- Crear AST definitions
- Implementar visitor pattern
- Escribir tests unitarios
- Documentar API del parser

---

### Story 2.2: Detector de Overflow/Underflow
**Story Type:** Feature
**Points:** 3

**As a** security auditor,
**I want** detectar operaciones aritméticas sin checked operations,
**So that** puedo prevenir overflow/underflow bugs.

**Acceptance Criteria:**
- Detección de operaciones +, -, *, / sin checked_*
- Ignorar operaciones con SafeMath o saturating_*
- Severidad: High
- Reporte incluye línea de código y recomendación
- Tests con casos positivos y negativos
- False positive rate < 10%

**Tasks:**
- Implementar overflow analyzer
- Crear reglas de detección
- Implementar reporter
- Escribir tests
- Documentar detector

---

### Story 2.3: Detector de Authorization Issues
**Story Type:** Feature
**Points:** 3

**As a** security auditor,
**I want** detectar problemas de validación de autorización,
**So that** puedo prevenir accesos no autorizados.

**Acceptance Criteria:**
- Detección de extrinsics sin ensure_signed o ensure_root
- Detección de acceso a storage sin validación de origin
- Severidad: Critical
- Reporte detallado con línea y función
- Tests con múltiples patrones de vulnerabilidad
- Documentación de best practices

**Tasks:**
- Implementar authorization analyzer
- Detectar patrones de authorization
- Crear reglas de severidad
- Escribir tests exhaustivos
- Documentar patterns detectables

---

### Story 2.4: Detector de Ownership Problems
**Story Type:** Feature
**Points:** 3

**As a** security auditor,
**I want** detectar problemas de ownership en transferencias,
**So that** puedo prevenir robos de assets.

**Acceptance Criteria:**
- Detección de transfers sin verificación de ownership
- Detección de cambios de owner sin validación
- Severidad: Critical
- Sugerencias de fix incluidas
- Tests con casos edge
- Integración con otros detectores

**Tasks:**
- Implementar ownership analyzer
- Crear reglas de ownership
- Implementar fix suggestions
- Escribir tests
- Documentar vulnerabilidades comunes

---

### Story 2.5: Detector de Decimal Precision en XCM
**Story Type:** Feature
**Points:** 4

**As a** security auditor,
**I want** detectar problemas de precisión decimal en transfers XCM,
**So that** puedo prevenir pérdidas en cross-chain transfers.

**Acceptance Criteria:**
- Detección de conversiones de decimales sin validación
- Detección de rounding errors en XCM
- Severidad: High
- Análisis de tipos de decimals
- Tests con diferentes configuraciones de decimals
- Documentación de XCM best practices

**Tasks:**
- Implementar decimal analyzer
- Detectar conversiones XCM
- Analizar precision loss
- Escribir tests específicos de XCM
- Documentar problemas comunes

---

### Story 2.6: CLI Tool para SAFT
**Story Type:** Feature
**Points:** 3

**As a** developer,
**I want** una CLI tool fácil de usar para SAFT,
**So that** puedo analizar pallets desde la terminal.

**Acceptance Criteria:**
- Comando `saft analyze <path>` funcional
- Output en formato JSON, HTML, y texto
- Flags para configurar severidad mínima
- Progress bar durante análisis
- Colorized output
- Exit codes apropiados (0 = no issues, 1 = issues found)

**Tasks:**
- Implementar CLI usando clap
- Añadir output formatters
- Implementar progress indicators
- Añadir colored output
- Escribir help text
- Documentar CLI usage

---

### Story 2.7: Integración CI/CD para SAFT
**Story Type:** Feature
**Points:** 2

**As a** developer,
**I want** integrar SAFT en mi CI/CD,
**So that** cada commit sea analizado automáticamente.

**Acceptance Criteria:**
- GitHub Action para SAFT disponible
- GitLab CI template disponible
- Configuración como code (saft.yaml)
- Comentarios en PRs con resultados
- Fail builds si hay critical issues
- Documentación de integración

**Tasks:**
- Crear GitHub Action
- Crear GitLab CI template
- Implementar saft.yaml config
- Crear PR comment bot
- Documentar integraciones
- Ejemplos de configuración

---

## EPIC 3: Real-Time Monitoring Engine

### Story 3.1: Conexión a Parachain Node
**Story Type:** Feature
**Points:** 3

**As a** security monitor,
**I want** conectar al node de una parachain,
**So that** puedo monitorear transacciones en tiempo real.

**Acceptance Criteria:**
- Conexión vía WebSocket a Substrate node
- Subscription a new blocks
- Subscription a pending transactions
- Reconnection automática si se pierde conexión
- Logging de eventos de conexión
- Soporte para múltiples chains simultáneas

**Tasks:**
- Setup de paquete monitoring-engine
- Implementar WebSocket client usando Polkadot.js
- Event subscription system
- Reconnection logic
- Multi-chain support
- Tests de conexión

---

### Story 3.2: Mempool Monitoring
**Story Type:** Feature
**Points:** 4

**As a** security monitor,
**I want** monitorear el mempool de transacciones,
**So that** puedo detectar ataques antes de que se ejecuten.

**Acceptance Criteria:**
- Monitoreo de pending transactions
- Parsing de transaction data
- Extracción de calls y parámetros
- Indexing en DB para análisis
- Performance: procesamiento de 100+ tx/segundo
- Logging detallado

**Tasks:**
- Implementar mempool subscriber
- Transaction parser
- DB schema para transactions
- Indexing optimizado
- Performance optimization
- Tests de throughput

---

### Story 3.3: Flash Loan Attack Detector
**Story Type:** Feature
**Points:** 5

**As a** DeFi protocol,
**I want** detectar flash loan attacks en tiempo real,
**So that** puedo activar circuit breakers antes de pérdidas.

**Acceptance Criteria:**
- Detección de patrón: borrow + manipulación + repay en mismo bloque
- Análisis de balance changes anormales (>50% en un tx)
- Alert con severidad Critical
- Latencia de detección < 3 segundos
- False positive rate < 5%
- Historical data de ataques conocidos

**Tasks:**
- Implementar flash loan detector
- Pattern matching logic
- Balance change analysis
- Alert generation
- Tests con ataques simulados
- Documentar patrones detectables

---

### Story 3.4: Oracle Manipulation Detector
**Story Type:** Feature
**Points:** 5

**As a** DeFi protocol,
**I want** detectar manipulación de oráculos de precio,
**So that** puedo proteger liquidaciones y lending.

**Acceptance Criteria:**
- Monitoreo de price feeds de oráculos
- Detección de desviaciones > threshold (ej: 10% en 1 bloque)
- Correlación con volumen anormal
- Alert con severidad Critical
- Integración con múltiples oráculos (Chainlink, Band, etc.)
- Historical trending

**Tasks:**
- Implementar oracle monitor
- Price deviation detection
- Volume analysis
- Multi-oracle support
- Alert correlation
- Tests con datos históricos

---

### Story 3.5: Governance Attack Detector
**Story Type:** Feature
**Points:** 4

**As a** parachain governor,
**I want** detectar ataques a governance,
**So that** puedo responder antes de proposals maliciosos.

**Acceptance Criteria:**
- Monitoreo de governance proposals
- Detección de voting patterns anormales
- Last-minute voting surge detection
- Whale voting alerts (>5% de supply)
- Severidad: High
- Notificaciones a stakeholders

**Tasks:**
- Implementar governance monitor
- Voting pattern analysis
- Whale detection
- Alert system para governance
- Tests con proposals simulados
- Documentar attack vectors

---

### Story 3.6: Alert System con Webhooks
**Story Type:** Feature
**Points:** 3

**As a** security team,
**I want** recibir alertas vía webhook,
**So that** puedo integrar con mis sistemas de monitoring.

**Acceptance Criteria:**
- Configuración de webhooks vía API
- POST request a webhook URL cuando hay alert
- Payload JSON con detalles completos
- Retry logic (3 intentos con backoff)
- Alert de-duplication (no duplicates en 5 min)
- Logging de deliveries

**Tasks:**
- Implementar webhook delivery system
- Retry logic con exponential backoff
- De-duplication logic
- Webhook configuration API
- Tests de delivery
- Documentar payload format

---

### Story 3.7: REST API para Monitoring
**Story Type:** Feature
**Points:** 3

**As a** developer,
**I want** una API REST para acceder a datos de monitoring,
**So that** puedo integrar en mis aplicaciones.

**Acceptance Criteria:**
- GET /alerts: Lista de alertas con paginación
- GET /alerts/:id: Detalle de alerta específica
- POST /webhooks: Configurar webhook
- GET /stats: Estadísticas de monitoring
- Authentication con API keys
- Rate limiting (100 req/min)
- Swagger/OpenAPI documentation

**Tasks:**
- Implementar REST API usando actix-web
- Endpoints de alerts
- Endpoints de webhooks
- Authentication middleware
- Rate limiting
- Swagger docs
- Tests de API

---

## EPIC 4: Privacy Layer (ZKP)

### Story 4.1: ZK Circuit - Vulnerability Existence Proof
**Story Type:** Feature
**Points:** 8

**As a** security auditor,
**I want** generar zero-knowledge proof de una vulnerabilidad,
**So that** puedo reportarla sin revelar detalles.

**Acceptance Criteria:**
- Circuit implementado usando arkworks
- Public inputs: contract_hash, timestamp, severity
- Private inputs: vulnerability_description, exploit_code
- Proof generation < 30 segundos
- Proof size < 1KB
- Verification < 5 segundos
- Tests con múltiples vulnerabilidades

**Tasks:**
- Setup de arkworks library
- Diseñar circuit constraints
- Implementar prover
- Implementar verifier
- Optimization de performance
- Extensive testing
- Documentar circuit design

---

### Story 4.2: ZK Circuit - Verifiable Credentials
**Story Type:** Feature
**Points:** 8

**As a** security auditor,
**I want** probar mis credenciales sin revelar identidad,
**So that** puedo aplicar a trabajos de auditoría anónimamente.

**Acceptance Criteria:**
- Circuit para credentials verification
- Public inputs: credential_type, min_experience
- Private inputs: identity, experience_level, past_audits
- Proof de experiencia > threshold
- Proof de certificaciones
- Performance similar a Story 4.1
- Tests con diferentes credential types

**Tasks:**
- Diseñar credentials circuit
- Implementar credential constraints
- Prover implementation
- Verifier implementation
- Integration con issuer system
- Testing
- Documentation

---

### Story 4.3: ink! Smart Contract - Bug Bounty Marketplace
**Story Type:** Feature
**Points:** 5

**As a** project owner,
**I want** un marketplace de bug bounties on-chain,
**So that** puedo incentivar reportes de seguridad.

**Acceptance Criteria:**
- Smart contract en ink!
- Funciones: create_bounty, submit_vulnerability, verify, claim_reward
- Escrow de fondos automático
- Verification de ZK proofs on-chain
- Events para indexing
- Tests exhaustivos de contract
- Deployment en Kusama testnet

**Tasks:**
- Setup de ink! project
- Implementar bounty storage
- Escrow logic
- ZK proof verification on-chain
- Event emission
- Tests de contract
- Deploy scripts

---

### Story 4.4: ink! Smart Contract - Auditor Registry
**Story Type:** Feature
**Points:** 4

**As a** auditor,
**I want** registrarme on-chain con credentials verificables,
**So that** puedo construir reputación en el ecosistema.

**Acceptance Criteria:**
- Smart contract para auditor registry
- Funciones: register, verify_credentials, update_reputation
- Storage de credentials (hash, no datos privados)
- Reputation scoring
- Events para tracking
- Tests de contract
- Integration con Bug Bounty contract

**Tasks:**
- Implementar auditor registry contract
- Credential verification logic
- Reputation system
- Storage optimization
- Events
- Tests
- Integration testing

---

### Story 4.5: Commitment Scheme para Disclosure
**Story Type:** Feature
**Points:** 3

**As a** auditor,
**I want** crear commitment de vulnerabilidad con timestamp,
**So that** puedo probar que la descubrí primero sin revelar detalles.

**Acceptance Criteria:**
- Hash-based commitment scheme
- Timestamp on-chain
- Reveal mechanism
- Verification de reveals
- Time-lock opcional (ej: 90 días)
- Tests de commit-reveal flow

**Tasks:**
- Implementar commitment logic
- On-chain storage de commitments
- Reveal verification
- Time-lock mechanism
- Tests
- Documentation

---

### Story 4.6: Integration Layer para ZKP
**Story Type:** Feature
**Points:** 4

**As a** developer,
**I want** una librería fácil de usar para ZK proofs,
**So that** puedo integrar privacy features en mi app.

**Acceptance Criteria:**
- Rust library con API simple
- Funciones: generate_proof, verify_proof
- Serialization de proofs
- Error handling robusto
- Examples de uso
- Documentation completa
- Published en crates.io (opcional)

**Tasks:**
- Implementar high-level API
- Proof serialization
- Error types
- Examples
- Documentation
- Tests de integration
- Publishing (opcional)

---

## EPIC 5: Hyperbridge Integration (Cross-Chain)

### Story 5.1: ISMP Protocol Client
**Story Type:** Feature
**Points:** 5

**As a** cross-chain monitor,
**I want** conectar con Hyperbridge vía ISMP,
**So that** puedo monitorear seguridad cross-chain.

**Acceptance Criteria:**
- Client para protocolo ISMP
- Soporte para POST requests (envío de datos)
- Soporte para GET requests (lectura de storage)
- State proof verification
- Conexión a múltiples chains
- Error handling para network issues

**Tasks:**
- Implementar ISMP client
- POST request handling
- GET request handling
- State proof verification
- Multi-chain connection management
- Tests de protocol
- Documentation

---

### Story 5.2: State Proof Verification
**Story Type:** Feature
**Points:** 4

**As a** cross-chain monitor,
**I want** verificar state proofs de otras chains,
**So that** puedo confiar en datos cross-chain sin intermediarios.

**Acceptance Criteria:**
- Verification de Merkle proofs
- Validation de light client states
- Soporte para múltiples consensus (Ethereum, Polkadot, etc.)
- Caching de verified states
- Performance: verificación < 1 segundo
- Tests con proofs reales

**Tasks:**
- Implementar Merkle proof verifier
- Light client validation
- Multi-consensus support
- Caching layer
- Performance optimization
- Extensive testing

---

### Story 5.3: Multi-Chain Monitoring (Ethereum)
**Story Type:** Feature
**Points:** 4

**As a** cross-chain monitor,
**I want** monitorear Ethereum vía Hyperbridge,
**So that** puedo detectar ataques cross-chain.

**Acceptance Criteria:**
- Conexión a Ethereum vía Hyperbridge
- Monitoreo de transactions
- Detección de vulnerabilidades específicas de Ethereum
- State sync con Ethereum
- Alertas cross-chain
- Integration con monitoring engine

**Tasks:**
- Implementar Ethereum connector
- Transaction monitoring
- Ethereum-specific detectors
- State synchronization
- Alert integration
- Tests con Ethereum testnet

---

### Story 5.4: Multi-Chain Monitoring (Arbitrum)
**Story Type:** Feature
**Points:** 3

**As a** cross-chain monitor,
**I want** monitorear Arbitrum vía Hyperbridge,
**So that** puedo cubrir ecosistema L2.

**Acceptance Criteria:**
- Similar a Story 5.3 pero para Arbitrum
- Detección de vulnerabilidades de L2
- Sequencer monitoring
- Integration con Ethereum monitoring

**Tasks:**
- Similar a Story 5.3
- L2-specific features
- Sequencer integration

---

### Story 5.5: Cross-Chain Dashboard
**Story Type:** Feature
**Points:** 5

**As a** security team,
**I want** dashboard unificado de múltiples chains,
**So that** puedo ver seguridad cross-chain en un solo lugar.

**Acceptance Criteria:**
- Vista unificada de 3+ chains
- Filtros por chain
- Alertas cross-chain correlacionadas
- Métricas comparativas
- Real-time updates vía WebSocket
- Responsive design

**Tasks:**
- Diseñar UI multi-chain
- Implementar chain selector
- Cross-chain alert correlation
- Comparative metrics
- WebSocket integration
- Responsive CSS

---

## EPIC 6: Hydration Integration (DeFi)

### Story 6.1: Conexión a Hydration Parachain
**Story Type:** Feature
**Points:** 3

**As a** DeFi monitor,
**I want** conectar a Hydration parachain,
**So that** puedo monitorear Omnipool y lending.

**Acceptance Criteria:**
- Conexión a Hydration node
- Subscription a events de Omnipool
- Subscription a events de Lending
- Data parsing de Hydration-specific types
- Error handling
- Logging

**Tasks:**
- Setup Hydration client
- Event subscription
- Data parsing
- Error handling
- Tests de conexión
- Documentation

---

### Story 6.2: Omnipool Monitoring
**Story Type:** Feature
**Points:** 5

**As a** DeFi monitor,
**I want** monitorear el Omnipool de Hydration,
**So that** puedo detectar manipulación de liquidez.

**Acceptance Criteria:**
- Tracking de 160+ activos
- Monitoreo de TVL
- Detección de swaps anormales (>10% slippage)
- Detección de liquidity drain
- Alertas en tiempo real
- Historical data storage

**Tasks:**
- Implementar Omnipool monitor
- Asset tracking
- TVL calculation
- Swap analysis
- Liquidity monitoring
- Alert generation
- DB schema

---

### Story 6.3: Lending Protocol Health Monitoring
**Story Type:** Feature
**Points:** 4

**As a** DeFi monitor,
**I want** monitorear health de positions de lending,
**So that** puedo detectar riesgo de liquidaciones en cascada.

**Acceptance Criteria:**
- Tracking de health factors
- Detección de positions en riesgo (health < 1.1)
- Simulación de liquidaciones
- Alertas preventivas
- Integration con oracle prices
- Dashboard de lending health

**Tasks:**
- Implementar lending monitor
- Health factor calculation
- Risk detection
- Liquidation simulation
- Oracle integration
- Dashboard UI

---

### Story 6.4: Integración con HOLLAR
**Story Type:** Feature
**Points:** 3

**As a** user,
**I want** pagar servicios con HOLLAR,
**So that** puedo usar stablecoin nativa de Hydration.

**Acceptance Criteria:**
- Payment processing en HOLLAR
- Smart contract para pagos
- Conversion rate tracking
- Transaction receipts
- Refund mechanism
- Tests de payments

**Tasks:**
- Implementar HOLLAR payment processor
- Smart contract para escrow
- Rate tracking
- Receipt generation
- Refund logic
- Tests

---

### Story 6.5: Circuit Breakers para DeFi
**Story Type:** Feature
**Points:** 4

**As a** DeFi protocol,
**I want** circuit breakers automáticos,
**So that** puedo pausar operaciones si se detecta ataque.

**Acceptance Criteria:**
- Triggers configurables (ej: TVL drop >20%)
- Pausa automática de swaps
- Notificaciones a governance
- Manual override
- Logging de activaciones
- Tests de triggers

**Tasks:**
- Implementar circuit breaker logic
- Configurable triggers
- Pause mechanism
- Governance notification
- Override system
- Extensive testing

---

## EPIC 7: Web Dashboard

### Story 7.1: Setup de Next.js Dashboard
**Story Type:** Feature
**Points:** 2

**As a** developer,
**I want** proyecto Next.js configurado,
**So that** puedo construir el dashboard web.

**Acceptance Criteria:**
- Next.js 14 con App Router
- TypeScript configurado
- TailwindCSS setup
- shadcn/ui components instalados
- Layout base con navigation
- Responsive design base

**Tasks:**
- Create Next.js project
- Configure TypeScript
- Setup TailwindCSS
- Install shadcn/ui
- Create base layout
- Responsive navigation

---

### Story 7.2: Dashboard - Overview Page
**Story Type:** Feature
**Points:** 5

**As a** user,
**I want** ver overview de seguridad,
**So that** puedo entender estado general del sistema.

**Acceptance Criteria:**
- Métricas principales (total vulnerabilities, alerts, audits)
- Gráficos de tendencias (últimos 30 días)
- Recent activity feed
- Security score por parachain
- Real-time updates
- Loading states

**Tasks:**
- Diseñar overview UI
- Implementar metric cards
- Charts con Recharts
- Activity feed
- Real-time WebSocket
- Loading skeletons

---

### Story 7.3: Dashboard - Vulnerabilities Page
**Story Type:** Feature
**Points:** 5

**As a** security auditor,
**I want** ver lista de vulnerabilidades,
**So that** puedo priorizar fixes.

**Acceptance Criteria:**
- Lista filtrable y ordenable
- Filtros: severity, status, parachain
- Sorting: date, severity
- Pagination (20 items/page)
- Detail modal con info completa
- Export a CSV/JSON

**Tasks:**
- Diseñar vulnerabilities table
- Implementar filters
- Sorting logic
- Pagination
- Detail modal
- Export functionality

---

### Story 7.4: Dashboard - Real-Time Monitoring Page
**Story Type:** Feature
**Points:** 5

**As a** security monitor,
**I want** ver monitoring en tiempo real,
**So that** puedo responder inmediatamente a ataques.

**Acceptance Criteria:**
- Live feed de transacciones monitoreadas
- Active alerts con highlighting
- Mempool statistics (tx/segundo, etc.)
- Performance metrics del monitoring engine
- Auto-refresh cada 5 segundos
- Filter por chain

**Tasks:**
- Diseñar monitoring UI
- Live transaction feed
- Alert highlighting
- Stats visualization
- Auto-refresh logic
- Chain filter

---

### Story 7.5: Dashboard - Cross-Chain Page
**Story Type:** Feature
**Points:** 5

**As a** cross-chain monitor,
**I want** vista multi-chain,
**So that** puedo monitorear todas las chains desde un lugar.

**Acceptance Criteria:**
- Multi-chain overview
- Chain selector
- State proof verification status
- Bridge health metrics
- Cross-chain alerts
- Comparative metrics

**Tasks:**
- Diseñar multi-chain UI
- Chain selector component
- State proof status
- Bridge metrics
- Alert correlation
- Comparative charts

---

### Story 7.6: Dashboard - DeFi Security Page
**Story Type:** Feature
**Points:** 5

**As a** DeFi user,
**I want** ver métricas de seguridad DeFi,
**So that** puedo evaluar riesgo de protocols.

**Acceptance Criteria:**
- Hydration Omnipool metrics
- Lending protocol health
- TVL tracking con historical
- DeFi-specific alerts
- Health score visualization
- Circuit breaker status

**Tasks:**
- Diseñar DeFi UI
- Omnipool metrics display
- Lending health visualization
- TVL charts
- Alert display
- Circuit breaker status

---

### Story 7.7: Dashboard - Bug Bounty Marketplace
**Story Type:** Feature
**Points:** 8

**As a** auditor,
**I want** marketplace de bug bounties,
**So that** puedo reportar vulnerabilidades y recibir recompensas.

**Acceptance Criteria:**
- Lista de active bounties
- Submit vulnerability con ZK proof
- Proof generation UI
- Claim rewards
- Auditor leaderboard
- Reputation display
- Wallet integration (Polkadot.js Extension)

**Tasks:**
- Diseñar marketplace UI
- Bounty list display
- Submission form
- ZK proof generation integration
- Claim rewards flow
- Leaderboard
- Wallet integration

---

### Story 7.8: Dashboard - Settings Page
**Story Type:** Feature
**Points:** 3

**As a** user,
**I want** configurar mis preferencias,
**So that** puedo personalizar mi experiencia.

**Acceptance Criteria:**
- Alert configuration (severity, channels)
- Webhook setup
- API key generation
- User profile
- Notification preferences
- Theme selection (light/dark)

**Tasks:**
- Diseñar settings UI
- Alert configuration form
- Webhook management
- API key generation
- Profile editing
- Theme toggle

---

### Story 7.9: API Client Library
**Story Type:** Feature
**Points:** 3

**As a** frontend developer,
**I want** librería de API client type-safe,
**So that** puedo hacer requests con autocompletion.

**Acceptance Criteria:**
- TypeScript client con types completos
- All endpoints cubiertos
- Error handling
- Retry logic
- Request/response interceptors
- React hooks (opcional)

**Tasks:**
- Generar types desde OpenAPI
- Implementar client class
- Error handling
- Retry logic
- Interceptors
- React hooks wrapper

---

## EPIC 8: API Server

### Story 8.1: API Server Setup
**Story Type:** Feature
**Points:** 2

**As a** backend developer,
**I want** servidor API configurado,
**So that** puedo exponer datos del monitoring engine.

**Acceptance Criteria:**
- Express.js o Fastify configurado
- TypeScript setup
- CORS configurado
- Body parsing
- Error handling middleware
- Logging (Winston o similar)

**Tasks:**
- Setup Node.js project
- Configure TypeScript
- Setup web framework
- Middleware configuration
- Error handler
- Logging setup

---

### Story 8.2: Authentication & Authorization
**Story Type:** Feature
**Points:** 4

**As a** API provider,
**I want** authentication con API keys,
**So that** solo usuarios autorizados accedan.

**Acceptance Criteria:**
- API key generation
- Key validation middleware
- Rate limiting por key (100 req/min)
- Key rotation
- Scopes/permissions
- Admin endpoints protegidos

**Tasks:**
- Implementar API key system
- Validation middleware
- Rate limiting
- Key management endpoints
- Permissions system
- Admin guards

---

### Story 8.3: Alerts API Endpoints
**Story Type:** Feature
**Points:** 3

**As a** developer,
**I want** endpoints para gestionar alertas,
**So that** puedo integrar alertas en mi app.

**Acceptance Criteria:**
- GET /api/alerts (list con pagination)
- GET /api/alerts/:id (detail)
- PATCH /api/alerts/:id (mark as read)
- DELETE /api/alerts/:id (dismiss)
- Query params: severity, status, chain
- OpenAPI documentation

**Tasks:**
- Implementar GET /alerts
- Implementar GET /alerts/:id
- Implementar PATCH /alerts/:id
- Implementar DELETE /alerts/:id
- Query filtering
- OpenAPI docs

---

### Story 8.4: Vulnerabilities API Endpoints
**Story Type:** Feature
**Points:** 3

**As a** developer,
**I want** endpoints para vulnerabilidades,
**So that** puedo mostrar resultados de SAFT.

**Acceptance Criteria:**
- GET /api/vulnerabilities
- GET /api/vulnerabilities/:id
- POST /api/vulnerabilities (submit from SAFT)
- PATCH /api/vulnerabilities/:id (update status)
- Filtering y sorting
- OpenAPI docs

**Tasks:**
- Implementar CRUD endpoints
- Filtering logic
- Sorting
- Pagination
- Validation
- Documentation

---

### Story 8.5: Webhooks API Endpoints
**Story Type:** Feature
**Points:** 3

**As a** developer,
**I want** configurar webhooks vía API,
**So that** puedo recibir notificaciones en mis sistemas.

**Acceptance Criteria:**
- POST /api/webhooks (create)
- GET /api/webhooks (list)
- PUT /api/webhooks/:id (update)
- DELETE /api/webhooks/:id (delete)
- POST /api/webhooks/:id/test (test webhook)
- Validation de webhook URL

**Tasks:**
- Implementar CRUD endpoints
- URL validation
- Test endpoint
- Security validation
- Documentation

---

### Story 8.6: Stats API Endpoints
**Story Type:** Feature
**Points:** 2

**As a** developer,
**I want** endpoints de estadísticas,
**So that** puedo mostrar métricas en mi dashboard.

**Acceptance Criteria:**
- GET /api/stats/overview
- GET /api/stats/trends
- GET /api/stats/chains
- Date range filtering
- Aggregation de métricas
- Caching (Redis)

**Tasks:**
- Implementar stats endpoints
- Aggregation queries
- Date filtering
- Caching layer
- Documentation

---

### Story 8.7: WebSocket Server
**Story Type:** Feature
**Points:** 4

**As a** developer,
**I want** WebSocket para updates en tiempo real,
**So that** mi dashboard se actualice automáticamente.

**Acceptance Criteria:**
- WebSocket server (Socket.io o ws)
- Authentication en WebSocket
- Channels: alerts, vulnerabilities, monitoring
- Subscribe/unsubscribe
- Reconnection automática del cliente
- Heartbeat para keep-alive

**Tasks:**
- Setup WebSocket server
- Authentication
- Channel management
- Event broadcasting
- Client reconnection
- Heartbeat

---

### Story 8.8: OpenAPI/Swagger Documentation
**Story Type:** Feature
**Points:** 2

**As a** API consumer,
**I want** documentación interactiva de API,
**So that** puedo entender y testear endpoints.

**Acceptance Criteria:**
- Swagger UI disponible en /api-docs
- All endpoints documentados
- Request/response examples
- Authentication instructions
- Try-it-out funcional
- Downloadable OpenAPI spec

**Tasks:**
- Setup Swagger/OpenAPI
- Documentar todos los endpoints
- Ejemplos de requests/responses
- Authentication docs
- UI customization

---

## EPIC 9: Substrate Pallets

### Story 9.1: Security Registry Pallet
**Story Type:** Feature
**Points:** 8

**As a** parachain developer,
**I want** pallet para registry de auditorías,
**So that** puedo almacenar resultados on-chain.

**Acceptance Criteria:**
- Pallet usando FRAME
- Storage de audit records
- Extrinsics: register_audit, update_audit
- Events: AuditRegistered, AuditUpdated
- Weights calculados
- Tests completos
- Benchmarking

**Tasks:**
- Setup pallet structure
- Implementar storage
- Extrinsics implementation
- Events
- Weight calculation
- Tests
- Benchmarking

---

### Story 9.2: Reputation Pallet
**Story Type:** Feature
**Points:** 8

**As a** auditor,
**I want** pallet de reputación on-chain,
**So that** mi reputación sea verificable y portable.

**Acceptance Criteria:**
- Pallet para reputation system
- Storage de reputation scores
- Extrinsics: update_reputation, slash_reputation
- Score calculation on-chain
- Events de reputation changes
- Tests y benchmarking

**Tasks:**
- Pallet structure
- Storage implementation
- Score calculation logic
- Extrinsics
- Events
- Tests
- Benchmarking

---

### Story 9.3: Integración de Pallets en Runtime
**Story Type:** Feature
**Points:** 3

**As a** parachain developer,
**I want** integrar pallets en runtime,
**So that** puedan ser usados en producción.

**Acceptance Criteria:**
- Pallets añadidos a runtime
- Configuration types
- Genesis config
- Runtime compilation exitosa
- Integration tests
- Documentation

**Tasks:**
- Añadir pallets a Cargo.toml
- Runtime configuration
- Genesis setup
- Compile runtime
- Integration tests
- Docs

---

## EPIC 10: Testing & Quality

### Story 10.1: Unit Tests para Rust Packages
**Story Type:** Chore
**Points:** 5

**As a** developer,
**I want** unit tests exhaustivos,
**So that** puedo confiar en la calidad del código.

**Acceptance Criteria:**
- Code coverage > 80%
- Tests para todos los detectores de SAFT
- Tests para monitoring engine
- Tests para ZK circuits
- Tests para pallets
- CI ejecuta tests automáticamente

**Tasks:**
- Escribir tests para SAFT
- Tests para monitoring
- Tests para privacy layer
- Tests para pallets
- Coverage reporting
- CI integration

---

### Story 10.2: Integration Tests
**Story Type:** Chore
**Points:** 5

**As a** developer,
**I want** integration tests,
**So that** puedo verificar que componentes funcionan juntos.

**Acceptance Criteria:**
- Tests de SAFT + API
- Tests de monitoring + alerts
- Tests de dashboard + API
- Tests de smart contracts + backend
- Tests end-to-end
- CI integration

**Tasks:**
- Setup test environment
- SAFT integration tests
- Monitoring integration tests
- Dashboard E2E tests
- Contract integration tests
- CI configuration

---

### Story 10.3: E2E Tests con Playwright
**Story Type:** Chore
**Points:** 3

**As a** QA engineer,
**I want** E2E tests del dashboard,
**So that** puedo verificar flujos de usuario.

**Acceptance Criteria:**
- Playwright configurado
- Tests de flujos críticos:
  - Login flow
  - View vulnerabilities
  - Submit bug bounty
  - Configure alerts
- Tests en múltiples browsers
- Screenshots en failure

**Tasks:**
- Setup Playwright
- Escribir E2E tests
- Multi-browser configuration
- Screenshot setup
- CI integration

---

### Story 10.4: Performance Testing
**Story Type:** Chore
**Points:** 3

**As a** DevOps engineer,
**I want** performance tests,
**So that** pueda verificar que el sistema escala.

**Acceptance Criteria:**
- Load testing con k6 o Artillery
- Throughput: 1000 tx/segundo
- API latency: p95 < 100ms
- Dashboard load time: < 2 segundos
- Memory leaks: ninguno
- Reports de performance

**Tasks:**
- Setup k6/Artillery
- Escribir load tests
- Latency benchmarks
- Memory profiling
- Report generation

---

### Story 10.5: Security Audit del Sistema
**Story Type:** Chore
**Points:** 5

**As a** security lead,
**I want** audit de seguridad del sistema,
**So that** pueda garantizar que no tiene vulnerabilidades.

**Acceptance Criteria:**
- Code audit completo
- Dependency audit (cargo audit, npm audit)
- Penetration testing de API
- Smart contract audit
- Zero critical vulnerabilities
- Audit report

**Tasks:**
- Code review
- Dependency scanning
- API pen testing
- Contract audit
- Vulnerability remediation
- Final report

---

## EPIC 11: Documentation

### Story 11.1: Architecture Documentation
**Story Type:** Chore
**Points:** 3

**As a** new developer,
**I want** documentación de arquitectura,
**So that** pueda entender el sistema rápidamente.

**Acceptance Criteria:**
- System overview diagram
- Component interaction diagrams
- Data flow documentation
- Security model explanation
- Technology stack documentation
- Decision records (ADRs)

**Tasks:**
- Crear diagramas
- Escribir system overview
- Documentar data flow
- Security model doc
- Tech stack doc
- ADRs

---

### Story 11.2: API Documentation
**Story Type:** Chore
**Points:** 2

**As a** API consumer,
**I want** documentación completa de API,
**So that** pueda integrar fácilmente.

**Acceptance Criteria:**
- OpenAPI spec completo
- Postman collection
- Code examples (curl, JavaScript, Rust)
- Authentication guide
- Rate limiting docs
- Error handling docs

**Tasks:**
- Complete OpenAPI spec
- Crear Postman collection
- Escribir code examples
- Auth guide
- Rate limiting docs
- Error docs

---

### Story 11.3: User Guide
**Story Type:** Chore
**Points:** 3

**As a** user,
**I want** user guide detallado,
**So that** pueda usar todas las features.

**Acceptance Criteria:**
- Getting started guide
- Dashboard user guide
- SAFT CLI guide
- Bug bounty submission guide
- Alert configuration guide
- Troubleshooting guide
- FAQ

**Tasks:**
- Getting started
- Dashboard guide
- CLI guide
- Bug bounty guide
- Alert config guide
- Troubleshooting
- FAQ

---

### Story 11.4: Integration Guides
**Story Type:** Chore
**Points:** 2

**As a** developer,
**I want** integration guides,
**So that** pueda integrar en mi workflow.

**Acceptance Criteria:**
- CI/CD integration guide (GitHub Actions, GitLab CI)
- Webhook setup guide
- Custom detector development guide
- API integration examples
- Docker deployment guide

**Tasks:**
- CI/CD guide
- Webhook guide
- Custom detector guide
- API examples
- Docker guide

---

### Story 11.5: Video Demo & Pitch Materials
**Story Type:** Chore
**Points:** 5

**As a** presenter,
**I want** video demo profesional,
**So that** pueda presentar en el hackathon.

**Acceptance Criteria:**
- Video demo 3-5 minutos
- Narración clara
- Demo de features clave
- Pitch deck (10-15 slides)
- Impact report con métricas
- Screenshots de alta calidad

**Tasks:**
- Script del video
- Grabación de demo
- Edición de video
- Crear pitch deck
- Impact report
- Screenshots

---

## EPIC 12: Deployment & DevOps

### Story 12.1: Kusama Testnet Deployment
**Story Type:** Feature
**Points:** 5

**As a** DevOps engineer,
**I want** deploy en Kusama testnet,
**So that** pueda testear en ambiente productivo.

**Acceptance Criteria:**
- Pallets deployed en Kusama testnet
- Smart contracts deployed
- Node conectado y sincronizado
- Monitoring activo
- Logs centralizados
- Documentation de deployment

**Tasks:**
- Preparar deployment scripts
- Deploy pallets
- Deploy contracts
- Configure node
- Setup monitoring
- Documentation

---

### Story 12.2: Infrastructure as Code
**Story Type:** Chore
**Points:** 3

**As a** DevOps engineer,
**I want** infraestructura como código,
**So that** pueda replicar environments fácilmente.

**Acceptance Criteria:**
- Terraform o similar para infrastructure
- Kubernetes manifests (si aplica)
- Docker Compose para local
- Environment configs
- Secrets management
- Documentation

**Tasks:**
- Setup Terraform
- Kubernetes manifests
- Docker Compose
- Env configs
- Secrets management
- Docs

---

### Story 12.3: Monitoring & Logging Infrastructure
**Story Type:** Feature
**Points:** 4

**As a** DevOps engineer,
**I want** monitoring y logging,
**So that** pueda detectar problemas rápidamente.

**Acceptance Criteria:**
- Prometheus para métricas
- Grafana dashboards
- Loki para logs
- Alerting rules
- Log aggregation
- Retention policies

**Tasks:**
- Setup Prometheus
- Grafana dashboards
- Loki setup
- Alert rules
- Log aggregation
- Retention config

---

### Story 12.4: Backup & Disaster Recovery
**Story Type:** Chore
**Points:** 2

**As a** DevOps engineer,
**I want** backup y disaster recovery,
**So that** pueda recuperar datos si hay fallo.

**Acceptance Criteria:**
- Database backups automáticos
- Backup retention (30 días)
- Restore procedures documentados
- Disaster recovery plan
- Testing de restores
- Documentation

**Tasks:**
- Setup automated backups
- Retention policies
- Restore procedures
- DR plan
- Test restores
- Documentation

---

## EPIC 13: Final Polish & Presentation

### Story 13.1: Bug Fixes & Refinement
**Story Type:** Chore
**Points:** 5

**As a** developer,
**I want** fixing de todos los bugs conocidos,
**So that** la demo sea impecable.

**Acceptance Criteria:**
- Zero critical bugs
- Zero high bugs
- Medium/low bugs documented
- Code cleanup
- Performance optimization
- UX improvements based on feedback

**Tasks:**
- Bug triage
- Critical bug fixes
- High bug fixes
- Code cleanup
- Performance tuning
- UX improvements

---

### Story 13.2: README & Getting Started
**Story Type:** Chore
**Points:** 2

**As a** hackathon judge,
**I want** README claro y conciso,
**So that** pueda entender el proyecto rápidamente.

**Acceptance Criteria:**
- README.md completo
- Project description
- Key features
- Quick start guide
- Screenshots/GIFs
- Links a documentation
- License

**Tasks:**
- Escribir README
- Add screenshots
- Quick start guide
- Links organization
- License file

---

### Story 13.3: Final Testing & QA
**Story Type:** Chore
**Points:** 3

**As a** QA engineer,
**I want** testing final completo,
**So that** pueda garantizar calidad.

**Acceptance Criteria:**
- All tests passing
- Manual testing de flujos críticos
- Cross-browser testing
- Mobile responsiveness testing
- Performance validation
- Security validation
- QA signoff

**Tasks:**
- Run all tests
- Manual QA
- Browser testing
- Mobile testing
- Performance validation
- Security check
- Final approval

---

### Story 13.4: Hackathon Submission Materials
**Story Type:** Chore
**Points:** 3

**As a** team lead,
**I want** todos los materiales de submission,
**So that** pueda enviar al hackathon.

**Acceptance Criteria:**
- Project description (500 palabras)
- Video demo (3-5 min)
- Pitch deck (PDF)
- GitHub repo limpio
- Demo URL funcional
- Impact metrics document
- Team bios

**Tasks:**
- Escribir description
- Finalizar video
- Export pitch deck
- Clean repo
- Deploy demo
- Metrics document
- Team bios

---

## Resumen de Estimación

### Por Épica:

| Épica | Stories | Points | Semanas Estimadas |
|-------|---------|--------|-------------------|
| 1. Infraestructura | 4 | 8 | 0.5 |
| 2. SAFT Enhanced | 7 | 24 | 1.5 |
| 3. Monitoring Engine | 7 | 27 | 2 |
| 4. Privacy Layer (ZKP) | 6 | 32 | 2.5 |
| 5. Hyperbridge Integration | 5 | 21 | 1.5 |
| 6. Hydration Integration | 5 | 19 | 1.5 |
| 7. Web Dashboard | 9 | 38 | 2.5 |
| 8. API Server | 8 | 23 | 1.5 |
| 9. Substrate Pallets | 3 | 19 | 1.5 |
| 10. Testing & Quality | 5 | 21 | 1.5 |
| 11. Documentation | 5 | 15 | 1 |
| 12. Deployment | 4 | 14 | 1 |
| 13. Final Polish | 4 | 13 | 1 |
| **TOTAL** | **72** | **274** | **~12 semanas** |

### Velocity Estimado:
- Con equipo de 3 desarrolladores
- Velocity de 20-25 points/semana
- Timeline: 11-14 semanas
- Buffer: 2 semanas para imprevistos

### Priorización (MoSCoW):

**Must Have (MVP para hackathon):**
- SAFT Enhanced (análisis estático básico)
- Monitoring Engine (detección en tiempo real)
- Dashboard básico
- API básica
- Kusama deployment

**Should Have:**
- Privacy Layer completo
- Cross-chain monitoring
- DeFi integration
- Full documentation

**Could Have:**
- Advanced ML detection
- Mobile app
- White-label solution

**Won't Have (post-hackathon):**
- Enterprise tier
- Series A materials
- Multiple language support

---

## Notas de Uso

### Para Pivotal Tracker:
1. Crear proyecto en Pivotal Tracker
2. Importar épicas como "Epics"
3. Cada story como "Story"
4. Asignar points
5. Priorizar en backlog
6. Empezar iteración

### Story Points Scale:
- 1 point = ~4 horas
- 2 points = ~1 día
- 3 points = ~1.5 días
- 5 points = ~2-3 días
- 8 points = ~1 semana

### Labels Sugeridos:
- `frontend` - Dashboard/UI work
- `backend` - API/Server work
- `rust` - Rust implementation
- `zkp` - Zero-knowledge proofs
- `substrate` - Substrate/Pallet work
- `docs` - Documentation
- `testing` - Testing/QA
- `devops` - Infrastructure/Deployment
- `bug` - Bug fix
- `critical` - Critical priority

---

**Última actualización:** 2025-01-14
**Versión:** 1.0
**Contacto:** Ver README.md
