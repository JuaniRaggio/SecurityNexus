# Dynamic Endpoint Switcher - Implementation Roadmap

## Overview

This document outlines the implementation plan for adding dynamic blockchain endpoint switching to Security Nexus. This feature allows users to switch between different Polkadot/Kusama networks in real-time without restarting the monitoring engine.

**Status:** Post-Hackathon Feature (v2.0)
**Priority:** Medium
**Estimated Time:** 3-5 hours
**Complexity:** Medium-High

---

## Use Cases

1. **Network Comparison:** Switch between Polkadot and Kusama to compare security postures
2. **Demo Flexibility:** Show different networks during presentations without restarting
3. **Multi-Network Monitoring:** Quick switching for teams monitoring multiple chains
4. **Testnet → Mainnet:** Easy transition from testing to production monitoring

---

## Technical Architecture

### Current Architecture (Static Endpoint)

```
Startup → Load WS_ENDPOINT from env → Create ConnectionManager → Connect → Monitor
                                              ↓
                                      endpoint: String (immutable)
```

**Limitations:**
- Endpoint is set once at startup
- Changing requires full restart
- Config is immutable after creation

### Proposed Architecture (Dynamic Endpoint)

```
Startup → Load default endpoint → Create ConnectionManager → Connect → Monitor
                                           ↓
                                   endpoint: Arc<RwLock<String>>
                                           ↓
User action → API call → Update endpoint → Disconnect → Reconnect → Monitor
```

**Benefits:**
- Hot-swap endpoints without restart
- Maintains engine uptime
- Graceful connection management

---

## Implementation Plan

### Phase 1: Backend Refactoring (2-3 hours)

#### 1.1 Modify ConnectionManager (`packages/monitoring-engine/src/connection.rs`)

**Current Structure:**
```rust
pub struct ConnectionManager {
    endpoint: String,  // Line 11 - Immutable
    client: Arc<RwLock<Option<OnlineClient<PolkadotConfig>>>>,
    reconnect_attempts: Arc<AtomicU32>,
    should_reconnect: Arc<AtomicBool>,
}
```

**Proposed Changes:**
```rust
pub struct ConnectionManager {
    endpoint: Arc<RwLock<String>>,  // Make mutable behind RwLock
    client: Arc<RwLock<Option<OnlineClient<PolkadotConfig>>>>,
    reconnect_attempts: Arc<AtomicU32>,
    should_reconnect: Arc<AtomicBool>,
}

impl ConnectionManager {
    /// Change endpoint and reconnect
    pub async fn change_endpoint(&self, new_endpoint: String) -> Result<(), MonitoringError> {
        info!("Changing endpoint to: {}", new_endpoint);

        // 1. Stop automatic reconnection
        self.should_reconnect.store(false, Ordering::SeqCst);

        // 2. Disconnect from current endpoint
        self.disconnect().await;

        // 3. Update endpoint
        {
            let mut endpoint = self.endpoint.write().await;
            *endpoint = new_endpoint;
        }

        // 4. Re-enable reconnection
        self.should_reconnect.store(true, Ordering::SeqCst);
        self.reconnect_attempts.store(0, Ordering::SeqCst);

        // 5. Connect to new endpoint
        self.connect_with_retry(5).await?;

        info!("Successfully switched to new endpoint");
        Ok(())
    }

    /// Get current endpoint
    pub async fn get_endpoint(&self) -> String {
        self.endpoint.read().await.clone()
    }
}
```

**Files to Update:**
- Lines to modify: 11, 27-31 (constructor), 47-93 (all methods using endpoint)
- Add new methods: `change_endpoint()`, `get_endpoint()`

#### 1.2 Update MonitoringEngine (`packages/monitoring-engine/src/lib.rs`)

**Current Structure:**
```rust
pub struct MonitoringEngine {
    config: MonitorConfig,  // Immutable
    connection: Arc<ConnectionManager>,
    // ...
}
```

**Proposed Changes:**
```rust
pub struct MonitoringEngine {
    config: Arc<RwLock<MonitorConfig>>,  // Make mutable
    connection: Arc<ConnectionManager>,
    // ...
}

impl MonitoringEngine {
    /// Change the monitoring endpoint
    pub async fn change_endpoint(
        &self,
        new_endpoint: String,
        new_chain_name: Option<String>,
    ) -> Result<(), MonitoringError> {
        // 1. Change connection endpoint
        self.connection.change_endpoint(new_endpoint.clone()).await?;

        // 2. Update config
        {
            let mut config = self.config.write().await;
            config.ws_endpoint = new_endpoint;
            if let Some(chain_name) = new_chain_name {
                config.chain_name = chain_name;
            }
        }

        // 3. Reset statistics (optional)
        self.reset_stats().await;

        Ok(())
    }

    /// Reset monitoring statistics
    async fn reset_stats(&self) {
        let mut stats = self.stats.write().await;
        stats.blocks_processed = 0;
        stats.transactions_analyzed = 0;
        stats.alerts_triggered = 0;
        // Keep uptime and reconnect attempts
    }
}
```

**Files to Update:**
- Lines to modify: 15 (config field), 82-97 (constructor)
- Add new methods: `change_endpoint()`, `reset_stats()`

#### 1.3 Add API Endpoint (`packages/monitoring-engine/src/api.rs`)

**New Endpoint:**
```rust
#[derive(Debug, Deserialize)]
struct ChangeEndpointRequest {
    ws_endpoint: String,
    chain_name: Option<String>,
}

#[derive(Debug, Serialize)]
struct ChangeEndpointResponse {
    success: bool,
    previous_endpoint: String,
    new_endpoint: String,
    message: String,
}

/// POST /api/switch-endpoint
async fn switch_endpoint(
    data: web::Data<ApiState>,
    payload: web::Json<ChangeEndpointRequest>,
) -> impl Responder {
    // Validate endpoint format
    if !payload.ws_endpoint.starts_with("ws://") &&
       !payload.ws_endpoint.starts_with("wss://") {
        return HttpResponse::BadRequest().json(json!({
            "error": "Invalid endpoint format. Must start with ws:// or wss://"
        }));
    }

    // Get current endpoint
    let previous_endpoint = data.engine.connection.get_endpoint().await;

    // Change endpoint
    match data.engine.change_endpoint(
        payload.ws_endpoint.clone(),
        payload.chain_name.clone(),
    ).await {
        Ok(_) => {
            HttpResponse::Ok().json(ChangeEndpointResponse {
                success: true,
                previous_endpoint,
                new_endpoint: payload.ws_endpoint.clone(),
                message: format!(
                    "Successfully switched to {} ({})",
                    payload.chain_name.as_deref().unwrap_or("Unknown"),
                    payload.ws_endpoint
                ),
            })
        }
        Err(e) => {
            error!("Failed to switch endpoint: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to switch endpoint",
                "details": e.to_string()
            }))
        }
    }
}

// In configure() method:
.route("/api/switch-endpoint", web::post().to(switch_endpoint))
```

**Files to Update:**
- Add new structs and function
- Update `configure()` method to register route

---

### Phase 2: Frontend Implementation (1-2 hours)

#### 2.1 Create EndpointSelector Component

**New File:** `/packages/web-dashboard/src/components/EndpointSelector.tsx`

```typescript
'use client'

import { useState } from 'react'
import { Globe, RefreshCw, Check, X } from 'lucide-react'

const ENDPOINTS = [
  {
    name: 'Polkadot',
    url: 'wss://polkadot.api.onfinality.io/public-ws',
    description: 'High activity production network',
    testnet: false,
  },
  {
    name: 'Kusama',
    url: 'wss://kusama.api.onfinality.io/public-ws',
    description: 'Canary network',
    testnet: false,
  },
  {
    name: 'Westend',
    url: 'wss://westend-rpc.dwellir.com',
    description: 'Test network',
    testnet: true,
  },
  {
    name: 'Paseo',
    url: 'wss://paseo-rpc.dwellir.com',
    description: 'Community testnet',
    testnet: true,
  },
]

interface Props {
  currentEndpoint?: string
  onSwitch: (endpoint: string, chainName: string) => Promise<void>
}

export default function EndpointSelector({ currentEndpoint, onSwitch }: Props) {
  const [isOpen, setIsOpen] = useState(false)
  const [isSwitching, setIsSwitching] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [success, setSuccess] = useState(false)

  const handleSwitch = async (endpoint: typeof ENDPOINTS[0]) => {
    setIsSwitching(true)
    setError(null)
    setSuccess(false)

    try {
      await onSwitch(endpoint.url, endpoint.name)
      setSuccess(true)
      setTimeout(() => {
        setSuccess(false)
        setIsOpen(false)
      }, 2000)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to switch endpoint')
    } finally {
      setIsSwitching(false)
    }
  }

  const currentNetwork = ENDPOINTS.find(e => e.url === currentEndpoint)

  return (
    <div className="relative">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
        disabled={isSwitching}
      >
        <Globe className="w-4 h-4" />
        <span>{currentNetwork?.name || 'Switch Network'}</span>
        {isSwitching && <RefreshCw className="w-4 h-4 animate-spin" />}
      </button>

      {isOpen && (
        <div className="absolute top-full mt-2 right-0 w-80 bg-white rounded-lg shadow-lg border border-gray-200 z-50">
          <div className="p-4 border-b border-gray-200">
            <h3 className="font-semibold text-gray-900">Select Network</h3>
            <p className="text-sm text-gray-600 mt-1">
              Switch to a different blockchain endpoint
            </p>
          </div>

          <div className="max-h-96 overflow-y-auto">
            {ENDPOINTS.map((endpoint) => (
              <button
                key={endpoint.url}
                onClick={() => handleSwitch(endpoint)}
                disabled={isSwitching || endpoint.url === currentEndpoint}
                className={`w-full p-4 text-left hover:bg-gray-50 transition-colors border-b border-gray-100 last:border-0 ${
                  endpoint.url === currentEndpoint ? 'bg-blue-50' : ''
                } ${isSwitching ? 'opacity-50 cursor-not-allowed' : ''}`}
              >
                <div className="flex items-start justify-between">
                  <div className="flex-1">
                    <div className="flex items-center gap-2">
                      <span className="font-medium text-gray-900">
                        {endpoint.name}
                      </span>
                      {endpoint.testnet && (
                        <span className="px-2 py-0.5 text-xs bg-yellow-100 text-yellow-800 rounded">
                          Testnet
                        </span>
                      )}
                    </div>
                    <p className="text-sm text-gray-600 mt-1">
                      {endpoint.description}
                    </p>
                    <p className="text-xs text-gray-500 mt-1 font-mono">
                      {endpoint.url}
                    </p>
                  </div>
                  {endpoint.url === currentEndpoint && (
                    <Check className="w-5 h-5 text-blue-600 flex-shrink-0" />
                  )}
                </div>
              </button>
            ))}
          </div>

          {error && (
            <div className="p-4 bg-red-50 border-t border-red-200">
              <div className="flex items-start gap-2">
                <X className="w-5 h-5 text-red-600 flex-shrink-0 mt-0.5" />
                <div>
                  <p className="text-sm font-medium text-red-900">
                    Failed to switch endpoint
                  </p>
                  <p className="text-xs text-red-700 mt-1">{error}</p>
                </div>
              </div>
            </div>
          )}

          {success && (
            <div className="p-4 bg-green-50 border-t border-green-200">
              <div className="flex items-center gap-2">
                <Check className="w-5 h-5 text-green-600" />
                <p className="text-sm font-medium text-green-900">
                  Endpoint switched successfully
                </p>
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  )
}
```

#### 2.2 Add Hook for Endpoint Switching

**File:** `/packages/web-dashboard/src/hooks/useMonitoring.ts`

```typescript
// Add to existing file

export function useSwitchEndpoint() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async ({
      endpoint,
      chainName
    }: {
      endpoint: string
      chainName: string
    }) => {
      const response = await fetch('/api/monitoring/switch-endpoint', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          ws_endpoint: endpoint,
          chain_name: chainName,
        }),
      })

      if (!response.ok) {
        const error = await response.json()
        throw new Error(error.error || 'Failed to switch endpoint')
      }

      return response.json()
    },
    onSuccess: () => {
      // Invalidate all queries to refresh with new network data
      queryClient.invalidateQueries({ queryKey: ['monitoring'] })
    },
  })
}
```

#### 2.3 Update Monitoring Page

**File:** `/packages/web-dashboard/src/app/monitoring/page.tsx`

```typescript
import EndpointSelector from '@/components/EndpointSelector'
import { useSwitchEndpoint } from '@/hooks/useMonitoring'

export default function MonitoringPage() {
  // ... existing code ...
  const switchEndpointMutation = useSwitchEndpoint()

  const handleEndpointSwitch = async (endpoint: string, chainName: string) => {
    await switchEndpointMutation.mutateAsync({ endpoint, chainName })
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Real-Time Monitoring</h1>
          {/* ... */}
        </div>
        <div className="flex items-center gap-4">
          <EndpointSelector
            currentEndpoint={stats?.endpoint}
            onSwitch={handleEndpointSwitch}
          />
          {/* Connection status badge */}
        </div>
      </div>
      {/* ... rest of page ... */}
    </div>
  )
}
```

#### 2.4 Add Proxy Route

**New File:** `/packages/web-dashboard/src/app/api/monitoring/switch-endpoint/route.ts`

```typescript
import { NextResponse } from 'next/server'

const MONITORING_ENGINE_URL = process.env.MONITORING_ENGINE_URL || 'http://localhost:8080'

export async function POST(request: Request) {
  try {
    const body = await request.json()

    const response = await fetch(`${MONITORING_ENGINE_URL}/api/switch-endpoint`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(body),
    })

    if (!response.ok) {
      const error = await response.json()
      return NextResponse.json(error, { status: response.status })
    }

    const data = await response.json()
    return NextResponse.json(data)
  } catch (error) {
    console.error('Error switching endpoint:', error)
    return NextResponse.json(
      {
        error: error instanceof Error ? error.message : 'Failed to switch endpoint',
        details: 'Make sure the monitoring engine is running',
      },
      { status: 503 }
    )
  }
}
```

---

### Phase 3: Testing & Polish (1 hour)

#### 3.1 Unit Tests

**Backend Tests:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_endpoint_change() {
        let manager = ConnectionManager::new(
            "wss://kusama.api.onfinality.io/public-ws".to_string()
        );

        manager.change_endpoint(
            "wss://polkadot.api.onfinality.io/public-ws".to_string()
        ).await.unwrap();

        let current = manager.get_endpoint().await;
        assert_eq!(current, "wss://polkadot.api.onfinality.io/public-ws");
    }
}
```

**Frontend Tests:**
- Test endpoint selector rendering
- Test network switching flow
- Test error handling

#### 3.2 Integration Tests

1. Switch from Polkadot to Kusama
2. Verify connection is established
3. Verify stats are reset
4. Verify alerts continue to work
5. Test error recovery (invalid endpoint)

#### 3.3 UI Polish

- Add loading states during switch
- Add success/error notifications
- Smooth transitions
- Keyboard navigation support
- Click outside to close dropdown

---

## Success Metrics

- [ ] Endpoint can be switched without engine restart
- [ ] Connection establishes within 10 seconds
- [ ] Statistics reset correctly
- [ ] No memory leaks after multiple switches
- [ ] UI is responsive during switch
- [ ] Error handling is graceful

---

## Risks & Mitigations

### Risk 1: Connection Failure During Switch
**Mitigation:**
- Implement timeout (30s max)
- Revert to previous endpoint if switch fails
- Show clear error message to user

### Risk 2: Race Conditions
**Mitigation:**
- Use RwLock for endpoint access
- Atomic flag for reconnection state
- Proper async/await handling

### Risk 3: Memory Leaks
**Mitigation:**
- Properly close old WebSocket connection
- Clear old subscriptions
- Test with valgrind/heaptrack

---

## Future Enhancements (v3.0+)

1. **Multi-Network Monitoring:** Monitor multiple chains simultaneously
2. **Endpoint Health Checks:** Automatically test endpoint before switching
3. **Custom Endpoints:** Allow users to add their own RPC endpoints
4. **Endpoint Profiles:** Save favorite endpoint configurations
5. **Auto-Failover:** Automatically switch to backup if primary fails

---

## Checklist for Implementation

### Backend
- [ ] Refactor ConnectionManager to use Arc<RwLock<String>> for endpoint
- [ ] Add change_endpoint() method
- [ ] Update all endpoint access to use read locks
- [ ] Make MonitoringEngine config mutable
- [ ] Add reset_stats() method
- [ ] Create POST /api/switch-endpoint route
- [ ] Add input validation for endpoints
- [ ] Write unit tests
- [ ] Test connection lifecycle

### Frontend
- [ ] Create EndpointSelector component
- [ ] Add useSwitchEndpoint hook
- [ ] Create proxy API route
- [ ] Update monitoring page layout
- [ ] Add loading/error states
- [ ] Style component to match design system
- [ ] Test error handling
- [ ] Add keyboard navigation
- [ ] Mobile responsive design

### Documentation
- [ ] Update README with endpoint switching guide
- [ ] Add API documentation for switch-endpoint
- [ ] Update demo script
- [ ] Add troubleshooting section

### Testing
- [ ] Manual testing with all endpoints
- [ ] Error case testing (invalid endpoint, timeout, etc.)
- [ ] Load testing (rapid switching)
- [ ] Memory leak testing
- [ ] Integration tests

---

## Timeline Estimate

| Phase | Task | Time |
|-------|------|------|
| 1 | Backend refactoring | 2-3 hours |
| 2 | Frontend implementation | 1-2 hours |
| 3 | Testing & polish | 1 hour |
| **Total** | | **4-6 hours** |

---

## Conclusion

The dynamic endpoint switcher is a valuable feature for v2.0 that enhances the flexibility and usability of Security Nexus. While it requires significant refactoring, the implementation is straightforward and the benefits are clear.

**Recommendation:** Implement this feature after the hackathon when there's time to properly test and polish it. For the Sub0 demo, stick with the static Polkadot mainnet endpoint which provides better reliability.
