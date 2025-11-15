import { useQuery } from '@tanstack/react-query'

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
