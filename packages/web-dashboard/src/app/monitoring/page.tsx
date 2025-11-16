'use client'

import { Activity, TrendingUp, AlertTriangle, CheckCircle, Wifi, WifiOff } from 'lucide-react'
import {
  useMonitoringStats,
  useHealthStatus,
  useDetectorStats,
  calculateBlocksPerSecond,
  formatUptime,
  formatLastDetection,
} from '@/hooks/useMonitoring'
import AlertsPanel from '@/components/AlertsPanel'

export default function MonitoringPage() {
  const { data: stats, isLoading: statsLoading, error: _statsError } = useMonitoringStats(2000)
  const { data: health, isLoading: healthLoading } = useHealthStatus(5000)
  const { data: detectorStats, isLoading: detectorsLoading } = useDetectorStats(5000)

  const isConnected = stats?.is_running && !stats?.error
  const blocksPerSecond = health?.uptime_seconds
    ? calculateBlocksPerSecond(stats?.blocks_processed || 0, health.uptime_seconds)
    : 0

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Real-Time Monitoring</h1>
          <p className="text-gray-600 mt-2">
            Monitor blockchain activity and detect security threats in real-time
          </p>
        </div>
        <div className="flex items-center gap-2">
          {isConnected ? (
            <div className="flex items-center gap-2 px-4 py-2 bg-green-100 text-green-800 rounded-lg">
              <Wifi className="w-4 h-4" />
              <span className="font-medium">Connected</span>
            </div>
          ) : (
            <div className="flex items-center gap-2 px-4 py-2 bg-red-100 text-red-800 rounded-lg">
              <WifiOff className="w-4 h-4" />
              <span className="font-medium">Disconnected</span>
            </div>
          )}
        </div>
      </div>

      {/* Connection Error Banner */}
      {stats?.error && (
        <div className="bg-yellow-50 border-l-4 border-yellow-400 p-4">
          <div className="flex">
            <div className="flex-shrink-0">
              <AlertTriangle className="h-5 w-5 text-yellow-400" />
            </div>
            <div className="ml-3">
              <p className="text-sm text-yellow-700">
                {stats.error} - Showing offline mode
              </p>
              <p className="text-xs text-yellow-600 mt-1">
                Make sure the monitoring engine is running on port 8080
              </p>
            </div>
          </div>
        </div>
      )}

      {/* Stats Grid */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-600">Blocks Processed</p>
              <p className="text-2xl font-bold text-gray-900 mt-1">
                {statsLoading ? '...' : (stats?.blocks_processed || 0).toLocaleString()}
              </p>
              <p className="text-xs text-gray-500 mt-1">
                {blocksPerSecond > 0 ? `${blocksPerSecond} blocks/sec` : 'Waiting...'}
              </p>
            </div>
            <TrendingUp className="w-8 h-8 text-green-600" />
          </div>
        </div>

        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-600">Transactions Analyzed</p>
              <p className="text-2xl font-bold text-gray-900 mt-1">
                {statsLoading ? '...' : (stats?.transactions_analyzed || 0).toLocaleString()}
              </p>
              <p className="text-xs text-gray-500 mt-1">
                {stats?.chain_name || 'Not connected'}
              </p>
            </div>
            <Activity className="w-8 h-8 text-blue-600" />
          </div>
        </div>

        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-600">Alerts Triggered</p>
              <p className="text-2xl font-bold text-gray-900 mt-1">
                {statsLoading ? '...' : stats?.alerts_triggered || 0}
              </p>
              <p className="text-xs text-gray-500 mt-1">
                {stats?.reconnect_attempts !== undefined && stats.reconnect_attempts > 0
                  ? `${stats.reconnect_attempts} reconnect attempts`
                  : 'All systems operational'}
              </p>
            </div>
            <AlertTriangle
              className={`w-8 h-8 ${
                (stats?.alerts_triggered || 0) > 0 ? 'text-orange-600' : 'text-gray-400'
              }`}
            />
          </div>
        </div>

        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-600">System Status</p>
              <p className="text-2xl font-bold text-gray-900 mt-1">
                {healthLoading ? '...' : health?.status === 'healthy' ? 'Healthy' : 'Offline'}
              </p>
              <p className="text-xs text-gray-500 mt-1">
                {health?.uptime_seconds ? `Uptime: ${formatUptime(health.uptime_seconds)}` : 'No data'}
              </p>
            </div>
            <CheckCircle
              className={`w-8 h-8 ${
                health?.status === 'healthy' ? 'text-green-600' : 'text-gray-400'
              }`}
            />
          </div>
        </div>
      </div>

      {/* Chain Info */}
      <div className="bg-white rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold text-gray-900 mb-4">Chain Information</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <p className="text-sm text-gray-600">Chain Name</p>
            <p className="text-lg font-medium text-gray-900 mt-1">
              {stats?.chain_name || 'Not connected'}
            </p>
          </div>
          <div>
            <p className="text-sm text-gray-600">Endpoint</p>
            <p className="text-lg font-medium text-gray-900 mt-1 font-mono text-sm">
              {stats?.endpoint || 'Not connected'}
            </p>
          </div>
          <div>
            <p className="text-sm text-gray-600">Engine Version</p>
            <p className="text-lg font-medium text-gray-900 mt-1">
              {health?.version || 'Unknown'}
            </p>
          </div>
          <div>
            <p className="text-sm text-gray-600">Connection Status</p>
            <p className={`text-lg font-medium mt-1 ${isConnected ? 'text-green-600' : 'text-red-600'}`}>
              {isConnected ? 'Active' : 'Inactive'}
            </p>
          </div>
        </div>
      </div>

      {/* Detector Status */}
      <div className="bg-white rounded-lg shadow">
        <div className="p-6 border-b border-gray-200">
          <h2 className="text-xl font-semibold text-gray-900">
            Attack Pattern Detectors
          </h2>
          <p className="text-sm text-gray-600 mt-1">
            Real-time detection of security threats and attack patterns
          </p>
        </div>
        <div className="p-6">
          {detectorsLoading ? (
            <div className="text-center py-8 text-gray-500">
              <div className="animate-spin w-8 h-8 border-4 border-gray-300 border-t-gray-900 rounded-full mx-auto mb-2"></div>
              Loading detectors...
            </div>
          ) : (
            <div className="space-y-4">
              {detectorStats?.detectors.map((detector) => (
                <div
                  key={detector.name}
                  className="flex items-center justify-between p-4 border border-gray-200 rounded-lg"
                >
                  <div className="flex items-center gap-4">
                    <div
                      className={`w-3 h-3 rounded-full ${
                        detector.enabled && isConnected ? 'bg-green-500 animate-pulse' : 'bg-gray-300'
                      }`}
                    />
                    <div>
                      <h3 className="font-medium text-gray-900">{detector.name}</h3>
                      <p className="text-sm text-gray-600">
                        Last detection: {formatLastDetection(detector.last_detection)}
                      </p>
                    </div>
                  </div>
                  <div className="text-right">
                    <p className="text-2xl font-bold text-gray-900">
                      {detector.detections}
                    </p>
                    <p className="text-sm text-gray-600">detections total</p>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>

      {/* Security Alerts Panel */}
      <AlertsPanel />

      {/* Debug Info (only in development) */}
      {process.env.NODE_ENV === 'development' && (
        <div className="bg-gray-50 rounded-lg shadow p-6">
          <h3 className="text-sm font-semibold text-gray-700 mb-2">Debug Information</h3>
          <pre className="text-xs text-gray-600 overflow-auto">
            {JSON.stringify({ stats, health }, null, 2)}
          </pre>
        </div>
      )}
    </div>
  )
}
