# Test Samples - Vulnerable Pallets

Este directorio contiene ejemplos de pallets con vulnerabilidades **INTENCIONALES** para testear SAFT Enhanced.

**⚠️ ADVERTENCIA**: Estos pallets contienen vulnerabilidades conocidas y NO deben usarse en producción.

## Propósito

Estos archivos son para:
- Testear que SAFT Enhanced detecta vulnerabilidades correctamente
- Demos del dashboard Security Nexus
- Educación sobre security patterns en Substrate/Polkadot
- Validar cobertura de Polkadot Top 10 Security Issues

## Estado del Ecosistema Polkadot 2024

**Nota importante**: Durante 2024, el ecosistema Polkadot ha demostrado excelente seguridad:
- ✅ **Cero hacks mayores** en parachains durante 2024
- ✅ Shared security model funcionando correctamente
- ✅ Comunidad activa en bug bounties (Immunefi, HackenProof)
- ✅ Auditorías continuas de Parity y la comunidad

Sin embargo, las vulnerabilidades en código custom siguen siendo posibles, por eso Security Nexus es importante.

## Mapeo a Polkadot Top 10 (2024)

Estos test samples cubren el [Polkadot Top 10 oficial](https://security.parity.io/top):

| # | Vulnerabilidad | Test Sample | Severidad | Estado |
|---|----------------|-------------|-----------|--------|
| 1 | Insecure Randomness | `insecure_randomness.rs` | Critical | ✅ |
| 2 | Unsafe Arithmetic | `defi_vault.rs` | Critical | ✅ |
| 3 | Incorrect Benchmarking | `incorrect_weights.rs` | High | ✅ |
| 4 | Storage Issues | `unsafe_storage.rs` | High | ✅ |
| 5 | Replay Attacks | `replay_attack.rs` | Critical | ✅ |
| 6 | Transactional Missing | `transactional_missing.rs` | High | ✅ |
| 7 | XCM Vulnerabilities | `vulnerable_xcm_transfer.rs` | Critical | ✅ |
| 8 | Reentrancy | `defi_vault.rs` | Critical | ✅ |
| 9 | Access Control | `defi_vault.rs` | High | ✅ |
| 10 | Race Conditions | `defi_vault.rs` | Medium | ✅ |

**Cobertura: 10/10 (100%)** del Polkadot Top 10

---

## Vulnerable Pallets

### 1. `vulnerable-pallets/defi_vault.rs`
**Inspirado en**: Curve Finance Vyper bug (2024), integer overflows en DeFi

**Vulnerabilidades (7 total)**:
1. ✅ **Integer Overflow** (Critical) - Línea 74: `current_deposit + amount` sin checked arithmetic
2. ✅ **Race Condition** (Medium) - Líneas 76-81: TVL actualizado después del storage de deposit
3. ✅ **Reentrancy Attack** (Critical) - Línea 103: Callback antes de actualizar balance
4. ✅ **Missing Balance Check** (High) - Línea 125: Claim rewards sin verificar saldo del vault
5. ✅ **Missing Access Control** (High) - Línea 140: `set_reward_rate` sin `ensure_root!`
6. ✅ **Integer Overflow in Rewards** (Critical) - Línea 168: Multiplicación sin overflow check
7. ✅ **CEI Pattern Violation** (High) - Línea 185: External call antes de state update

**Severidad esperada**: 4 Critical, 2 High, 1 Medium

**Incident real**: Curve Finance Vyper reentrancy (Julio 2024) - $70M en riesgo

---

### 2. `vulnerable-pallets/insecure_randomness.rs`
**Basado en**: Polkadot Top 10 #1 - Insecure Randomness

**Vulnerabilidades (6 total)**:
1. ✅ **Block Hash Randomness** (Critical) - Línea 48: Usando block hash como fuente de aleatoriedad
2. ✅ **Block Number Randomness** (Critical) - Línea 64: Block number 100% predecible
3. ✅ **Randomness Collective Flip** (High) - Línea 81: Últimos 81 bloques manipulables por validadores
4. ✅ **Timestamp Manipulation** (High) - Línea 103: Timestamp como fuente de aleatoriedad
5. ✅ **User Input Entropy** (Medium) - Línea 124: Usuario controla datos de entrada
6. ✅ **Randomness Reuse** (Medium) - Línea 149: Misma fuente para múltiples operaciones

**Severidad esperada**: 2 Critical, 2 High, 2 Medium

**Impacto real**: Lotteries, NFT minting, validator selection

---

### 3. `vulnerable-pallets/incorrect_weights.rs`
**Basado en**: Polkadot Top 10 #3 - Incorrect Benchmarking

**Vulnerabilidades (8 total)**:
1. ✅ **Constant Weight for Variable Data** (Critical) - Línea 46: Peso fijo para datos variables
2. ✅ **Missing Storage I/O Weight** (High) - Línea 68: No contabiliza reads/writes
3. ✅ **Underestimated Loop Complexity** (Critical) - Línea 93: O(n²) con peso O(n)
4. ✅ **Missing Crypto Operation Weight** (High) - Línea 115: Hashing sin peso
5. ✅ **Unbounded Iteration** (Critical) - Línea 135: Iteración sin límite
6. ✅ **Conditional Branch Weight** (Medium) - Línea 160: Mismo peso para diferentes paths
7. ✅ **Zero Weight** (Critical) - Línea 190: Peso cero = DoS gratuito
8. ✅ **Missing Database Lookups** (High) - Línea 210: Múltiples DB reads sin peso

**Severidad esperada**: 4 Critical, 3 High, 1 Medium

**Impacto real**: DoS attacks, chain halts, unfair fees

---

### 4. `vulnerable-pallets/unsafe_storage.rs`
**Basado en**: Polkadot Top 10 #4 - Storage Issues

**Vulnerabilidades (8 total)**:
1. ✅ **Unbounded Vector** (Critical) - Línea 38: Vec sin límite de tamaño
2. ✅ **No Storage Deposit** (Critical) - Línea 68: Storage gratis
3. ✅ **Fixed Fee for Variable Size** (High) - Línea 89: Depósito fijo independiente de tamaño
4. ✅ **Unbounded Global List** (High) - Línea 109: Lista global sin límite
5. ✅ **No Cleanup on Update** (Medium) - Línea 129: Memory leak al actualizar
6. ✅ **Batch Without Aggregate Deposit** (High) - Línea 143: Batch sin depósito total
7. ✅ **No Refund on Deletion** (High) - Línea 165: Depósito no devuelto
8. ✅ **No Cleanup on Account Close** (Medium) - Línea 190: Datos huérfanos

**Severidad esperada**: 2 Critical, 4 High, 2 Medium

**Impacto real**: State bloat, chain performance degradation

---

### 5. `vulnerable-pallets/replay_attack.rs`
**Basado en**: Polkadot Top 10 #5 - Replay Attacks

**Vulnerabilidades (8 total)**:
1. ✅ **Unsigned Without Replay Protection** (Critical) - Línea 50: Sin nonce
2. ✅ **Heartbeat Without Nonce** (High) - Línea 69: Heartbeat replay
3. ✅ **Weak Faucet Protection** (High) - Línea 84: Boolean simple
4. ✅ **Vote Replay** (Critical) - Línea 106: Votos duplicados
5. ✅ **Operation ID Without Account** (High) - Línea 124: ID no vinculado a cuenta
6. ✅ **No Timestamp Validation** (High) - Línea 143: Sin timestamp
7. ✅ **Cross-Chain Replay** (Critical) - Línea 163: Sin chain ID
8. ✅ **Missing Deadline** (Medium) - Línea 181: Sin expiración

**Severidad esperada**: 3 Critical, 4 High, 1 Medium

**Incident real**: Ethereum/ETC split (2016), similar scenarios possible

---

### 6. `vulnerable-pallets/transactional_missing.rs`
**Basado en**: Substrate Best Practices - Transactional Issues

**Vulnerabilidades (7 total)**:
1. ✅ **Multi-step Without Transactional** (Critical) - Línea 56: Transfer + fee sin atomicidad
2. ✅ **Mint Without Rollback** (Critical) - Línea 93: Total supply sin rollback
3. ✅ **Complex Swap Non-Atomic** (Critical) - Línea 122: Swap parcial
4. ✅ **Lock Without Rollback** (High) - Línea 157: Lock permanente en error
5. ✅ **Batch Without Transactional** (High) - Línea 184: Batch incompleto
6. ✅ **Burn Without Supply Adjustment** (High) - Línea 212: Accounting error
7. ✅ **Safe Example Provided** (Info) - Línea 243: Ejemplo correcto

**Severidad esperada**: 3 Critical, 3 High, 1 Info

**Impacto real**: Accounting errors, locked funds, partial state changes

---

### 7. `vulnerable-pallets/vulnerable_xcm_transfer.rs`
**Basado en**: XCM Decimal Precision Issues

**Vulnerabilidades (4 total)**:
1. ✅ **Hardcoded Decimal Amount** (Critical) - Línea 65: 1 DOT sin conversión
2. ✅ **No Decimal Conversion** (Critical) - Línea 91: User amount sin conversión
3. ✅ **Hardcoded Withdrawal** (High) - Línea 121: 10 DOT hardcoded
4. ✅ **Batch Without Conversion** (High) - Línea 148: Multiple transfers

**Severidad esperada**: 2 Critical, 2 High

**Incident real**: Acala aUSD depeg (2022, pre-2024 pero relevante)

---

## Cómo Usar

### Análisis Individual

```bash
# Analizar un pallet específico
cargo run --release --package saft-enhanced -- analyze test-samples/vulnerable-pallets/defi_vault.rs

# Con formato JSON
cargo run --release --package saft-enhanced -- analyze test-samples/vulnerable-pallets/insecure_randomness.rs --format json

# Con formato HTML
cargo run --release --package saft-enhanced -- analyze test-samples/vulnerable-pallets/incorrect_weights.rs --format html > report.html
```

### Desde el Dashboard

```bash
# 1. Iniciar dashboard
cd packages/web-dashboard
pnpm dev

# 2. Abrir navegador
open http://localhost:3000/analysis

# 3. Upload cualquier archivo .rs
# Recomendado para demo: defi_vault.rs (7 vulnerabilidades detectadas)
```

### Análisis Batch

```bash
# Analizar todos los pallets vulnerables
for file in test-samples/vulnerable-pallets/*.rs; do
    echo "Analyzing $file..."
    cargo run --release --package saft-enhanced -- analyze "$file" --format json > "${file%.rs}_report.json"
done
```

---

## Expected Results

SAFT Enhanced debería detectar:

| Archivo | Critical | High | Medium | Total |
|---------|----------|------|--------|-------|
| `defi_vault.rs` | 4 | 2 | 1 | 7 |
| `insecure_randomness.rs` | 2 | 2 | 2 | 6 |
| `incorrect_weights.rs` | 4 | 3 | 1 | 8 |
| `unsafe_storage.rs` | 2 | 4 | 2 | 8 |
| `replay_attack.rs` | 3 | 4 | 1 | 8 |
| `transactional_missing.rs` | 3 | 3 | 0 | 6 |
| `vulnerable_xcm_transfer.rs` | 2 | 2 | 0 | 4 |
| **TOTAL** | **20** | **20** | **7** | **47** |

**Benchmark de calidad**: Si SAFT detecta 35+ vulnerabilidades (75%+), es considerado excelente.

---

## Educational Value

Estos ejemplos son valiosos para:

1. **Desarrolladores Substrate**: Aprender patterns seguros vs inseguros
2. **Auditores**: Casos de estudio de vulnerabilidades reales
3. **Estudiantes**: Entender seguridad en blockchain
4. **Testing**: Validar herramientas de análisis estático

Cada archivo incluye:
- Comentarios explicativos del "por qué" es vulnerable
- Referencias a incidents reales
- Ejemplos de código seguro
- Links a documentación oficial

---

## Referencias

### Polkadot Security
- [Polkadot Top 10 Security Issues](https://security.parity.io/top)
- [Substrate Security Best Practices](https://docs.substrate.io/build/troubleshoot-your-code/)
- [Parity Security Hub](https://security.parity.io/)

### Incidents Reales
- [Curve Finance Vyper Bug (Jul 2024)](https://blog.curve.fi/july-30-2024-exploit-post-mortem/) - $70M
- [Acala aUSD Depeg (Aug 2022)](https://medium.com/acalanetwork/acala-usd-ausd-incident-investigation-and-remediation-plan-5a1e56dd99e9) - $1.2B minted
- [Immunefi $200M Save (2023)](https://www.theblock.co/post/199718/immunefi-researcher-saves-200-million-from-potential-theft-on-three-polkadot-parachains)

### Substrate Documentation
- [Runtime Storage](https://docs.substrate.io/build/runtime-storage/)
- [Transaction Weights](https://docs.substrate.io/build/tx-weights-fees/)
- [Unsigned Transactions](https://docs.substrate.io/build/unsigned-transactions/)
- [Randomness in Substrate](https://docs.substrate.io/build/randomness/)
- [Benchmarking](https://docs.substrate.io/test/benchmark/)

### Security Tools
- [Trail of Bits - Not So Smart Pallets](https://github.com/trailofbits/substrate-not-so-smart-contracts)
- [Immunefi Bug Bounty Platform](https://immunefi.com/)
- [HackenProof](https://hackenproof.com/)

---

## Contributing

Para agregar nuevos test samples:

1. Identificar vulnerabilidad del Polkadot Top 10 o incident real
2. Crear pallet mínimo que demuestre la vulnerabilidad
3. Documentar con comentarios inline
4. Agregar referencias a incidents reales
5. Incluir ejemplo de código seguro
6. Actualizar este README

---

## License

Apache 2.0 - Educational purposes only

**Authors**: Juan Ignacio Raggio, Victoria Helena Park
