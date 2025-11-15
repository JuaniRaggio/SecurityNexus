import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'

export interface MonitoringStats {
  is_running: boolean
  blocks_processed: number
  transactions_analyzed: number
  alerts_triggered: number
  chain_name: string
  endpoint: string
  reconnect_attempts: number
  error?: string
}

export interface HealthStatus {
  status: string
  version: string
  uptime_seconds: number
}

export interface Alert {
  id: string
  timestamp: number
  chain: string
  severity: 'low' | 'medium' | 'high' | 'critical'
  pattern: string
  description: string
  transaction_hash?: string
  block_number?: number
  metadata: Record<string, string>
  recommended_actions: string[]
  acknowledged: boolean
}

async function fetchMonitoringStats(): Promise<MonitoringStats> {
  const response = await fetch('/api/monitoring?endpoint=stats')
  if (!response.ok) {
    throw new Error('Failed to fetch monitoring stats')
  }
  return response.json()
}

async function fetchHealthStatus(): Promise<HealthStatus> {
  const response = await fetch('/api/monitoring?endpoint=health')
  if (!response.ok) {
    throw new Error('Failed to fetch health status')
  }
  return response.json()
}

export function useMonitoringStats(refreshInterval = 2000) {
  return useQuery<MonitoringStats>({
    queryKey: ['monitoring', 'stats'],
    queryFn: fetchMonitoringStats,
    refetchInterval: refreshInterval,
    refetchOnWindowFocus: true,
    retry: 3,
    retryDelay: 1000,
  })
}

export function useHealthStatus(refreshInterval = 5000) {
  return useQuery<HealthStatus>({
    queryKey: ['monitoring', 'health'],
    queryFn: fetchHealthStatus,
    refetchInterval: refreshInterval,
    refetchOnWindowFocus: true,
    retry: 3,
    retryDelay: 1000,
  })
}

// Calculate blocks per second
export function calculateBlocksPerSecond(
  blocks: number,
  uptimeSeconds: number
): number {
  if (uptimeSeconds === 0) return 0
  return parseFloat((blocks / uptimeSeconds).toFixed(2))
}

// Format uptime to human readable
export function formatUptime(seconds: number): string {
  const hours = Math.floor(seconds / 3600)
  const minutes = Math.floor((seconds % 3600) / 60)
  const secs = seconds % 60

  if (hours > 0) {
    return `${hours}h ${minutes}m`
  } else if (minutes > 0) {
    return `${minutes}m ${secs}s`
  }
  return `${secs}s`
}

// Fetch all alerts
async function fetchAlerts(): Promise<Alert[]> {
  const response = await fetch('/api/monitoring?endpoint=alerts')
  if (!response.ok) {
    throw new Error('Failed to fetch alerts')
  }
  return response.json()
}

// Fetch unacknowledged alerts only
async function fetchUnacknowledgedAlerts(): Promise<Alert[]> {
  const response = await fetch('/api/monitoring?endpoint=alerts/unacknowledged')
  if (!response.ok) {
    throw new Error('Failed to fetch unacknowledged alerts')
  }
  return response.json()
}

// Acknowledge an alert
async function acknowledgeAlert(alertId: string): Promise<void> {
  const response = await fetch(`/api/monitoring/acknowledge/${alertId}`, {
    method: 'POST',
  })
  if (!response.ok) {
    throw new Error('Failed to acknowledge alert')
  }
}

// Hook to fetch all alerts with auto-refresh
export function useAlerts(refreshInterval = 5000) {
  return useQuery<Alert[]>({
    queryKey: ['monitoring', 'alerts'],
    queryFn: fetchAlerts,
    refetchInterval: refreshInterval,
    refetchOnWindowFocus: true,
    retry: 3,
    retryDelay: 1000,
  })
}

// Hook to fetch unacknowledged alerts
export function useUnacknowledgedAlerts(refreshInterval = 5000) {
  return useQuery<Alert[]>({
    queryKey: ['monitoring', 'alerts', 'unacknowledged'],
    queryFn: fetchUnacknowledgedAlerts,
    refetchInterval: refreshInterval,
    refetchOnWindowFocus: true,
    retry: 3,
    retryDelay: 1000,
  })
}

// Hook to acknowledge an alert
export function useAcknowledgeAlert() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: acknowledgeAlert,
    onSuccess: () => {
      // Invalidate queries to refetch alerts
      queryClient.invalidateQueries({ queryKey: ['monitoring', 'alerts'] })
    },
  })
}

// Format timestamp to readable date
export function formatAlertTime(timestamp: number): string {
  const date = new Date(timestamp * 1000)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffMins = Math.floor(diffMs / 60000)
  const diffHours = Math.floor(diffMs / 3600000)
  const diffDays = Math.floor(diffMs / 86400000)

  if (diffMins < 1) return 'Just now'
  if (diffMins < 60) return `${diffMins}m ago`
  if (diffHours < 24) return `${diffHours}h ago`
  if (diffDays < 7) return `${diffDays}d ago`

  return date.toLocaleDateString()
}

// Get severity color
export function getSeverityColor(severity: Alert['severity']): string {
  switch (severity) {
    case 'critical':
      return 'red'
    case 'high':
      return 'orange'
    case 'medium':
      return 'yellow'
    case 'low':
      return 'blue'
    default:
      return 'gray'
  }
}
