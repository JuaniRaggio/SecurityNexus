'use client'

import { useState, useEffect } from 'react'
import { Activity, TrendingUp, AlertTriangle, CheckCircle, Wifi, WifiOff, BarChart3, Clock } from 'lucide-react'
import {
  useMonitoringStats,
  useHealthStatus,
  useDetectorStats,
  calculateBlocksPerSecond,
  formatUptime,
  formatLastDetection,
} from '@/hooks/useMonitoring'
import AlertsPanel from '@/components/AlertsPanel'
import ChainSelector from '@/components/ChainSelector'

interface ActivityData {
  timestamp: number
  blocks: number
  transactions: number
}

export default function MonitoringPage() {
  const { data: stats, isLoading: statsLoading, error: _statsError } = useMonitoringStats(2000)
  const { data: health, isLoading: healthLoading } = useHealthStatus(5000)
  const { data: detectorStats, isLoading: detectorsLoading } = useDetectorStats(5000)

  // Track activity history for charts
  const [activityHistory, setActivityHistory] = useState<ActivityData[]>([])

  // Update activity history
  useEffect(() => {
    if (stats) {
      const newData: ActivityData = {
        timestamp: Date.now(),
        blocks: stats.blocks_processed,
        transactions: stats.transactions_analyzed,
      }

      setActivityHistory((prev) => {
        const updated = [...prev, newData]
        // Keep only last 20 data points (40 seconds at 2s interval)
        return updated.slice(-20)
      })
    }
  }, [stats])

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
        <div className="flex items-center gap-3">
          <ChainSelector />
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

      {/* Real-Time Activity Chart */}
      <div className="bg-white rounded-lg shadow">
        <div className="p-6 border-b border-gray-200">
          <div className="flex items-center justify-between">
            <div>
              <h2 className="text-xl font-semibold text-gray-900 flex items-center gap-2">
                <BarChart3 className="w-5 h-5 text-blue-600" />
                Real-Time Activity
              </h2>
              <p className="text-sm text-gray-600 mt-1">
                Live blockchain processing metrics
              </p>
            </div>
            {activityHistory.length > 1 && (
              <div className="text-right">
                <p className="text-sm text-gray-600">Activity Rate</p>
                <p className="text-lg font-semibold text-blue-600">
                  {activityHistory[activityHistory.length - 1].blocks -
                   activityHistory[activityHistory.length - 2].blocks} blocks/2s
                </p>
              </div>
            )}
          </div>
        </div>
        <div className="p-6">
          {activityHistory.length < 2 ? (
            <div className="text-center py-12 text-gray-500">
              <Clock className="w-12 h-12 mx-auto mb-2 text-gray-400" />
              <p>Collecting activity data...</p>
              <p className="text-xs mt-1">Activity chart will appear in a few seconds</p>
            </div>
          ) : (
            <div className="space-y-6">
              {/* Simple Bar Chart */}
              <div>
                <p className="text-sm font-medium text-gray-700 mb-3">Block Processing Trend</p>
                <div className="flex items-end gap-1 h-32">
                  {activityHistory.map((data, idx) => {
                    const blocksDiff = idx > 0 ? data.blocks - activityHistory[idx - 1].blocks : 0
                    const maxBlocks = Math.max(...activityHistory.map((d, i) =>
                      i > 0 ? d.blocks - activityHistory[i - 1].blocks : 0
                    ))
                    const height = maxBlocks > 0 ? (blocksDiff / maxBlocks) * 100 : 0

                    return (
                      <div
                        key={data.timestamp}
                        className="flex-1 bg-gradient-to-t from-blue-500 to-blue-300 rounded-t transition-all duration-300 hover:from-blue-600 hover:to-blue-400"
                        style={{ height: `${Math.max(height, 5)}%` }}
                        title={`${blocksDiff} blocks at ${new Date(data.timestamp).toLocaleTimeString()}`}
                      />
                    )
                  })}
                </div>
                <div className="flex justify-between mt-2 text-xs text-gray-500">
                  <span>
                    {activityHistory.length > 0
                      ? new Date(activityHistory[0].timestamp).toLocaleTimeString()
                      : ''}
                  </span>
                  <span>Now</span>
                </div>
              </div>

              {/* Activity Stats Grid */}
              <div className="grid grid-cols-3 gap-4 pt-4 border-t border-gray-200">
                <div className="text-center">
                  <p className="text-2xl font-bold text-gray-900">
                    {stats?.blocks_processed || 0}
                  </p>
                  <p className="text-xs text-gray-600 mt-1">Total Blocks</p>
                </div>
                <div className="text-center">
                  <p className="text-2xl font-bold text-gray-900">
                    {stats?.transactions_analyzed || 0}
                  </p>
                  <p className="text-xs text-gray-600 mt-1">Total Transactions</p>
                </div>
                <div className="text-center">
                  <p className="text-2xl font-bold text-gray-900">
                    {activityHistory.length > 1
                      ? (
                          (activityHistory[activityHistory.length - 1].transactions -
                           activityHistory[0].transactions) /
                          activityHistory.length
                        ).toFixed(1)
                      : '0'}
                  </p>
                  <p className="text-xs text-gray-600 mt-1">Avg TX/Update</p>
                </div>
              </div>
            </div>
          )}
        </div>
      </div>

      {/* Recent Activity Log */}
      {activityHistory.length > 0 && (
        <div className="bg-white rounded-lg shadow">
          <div className="p-6 border-b border-gray-200">
            <h2 className="text-xl font-semibold text-gray-900 flex items-center gap-2">
              <Activity className="w-5 h-5 text-green-600" />
              Recent Activity
            </h2>
            <p className="text-sm text-gray-600 mt-1">
              Latest blockchain processing events
            </p>
          </div>
          <div className="divide-y divide-gray-200">
            {activityHistory.slice(-5).reverse().map((data, idx) => {
              const prevIdx = activityHistory.length - 5 + idx - 1
              const prevData = prevIdx >= 0 ? activityHistory[prevIdx] : null
              const blocksDiff = prevData ? data.blocks - prevData.blocks : 0
              const txDiff = prevData ? data.transactions - prevData.transactions : 0

              return (
                <div key={data.timestamp} className="p-4 hover:bg-gray-50 transition-colors">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-3">
                      <div className="w-2 h-2 bg-green-500 rounded-full animate-pulse" />
                      <div>
                        <p className="text-sm font-medium text-gray-900">
                          {blocksDiff > 0 ? `+${blocksDiff} blocks` : 'No new blocks'}
                          {txDiff > 0 && `, +${txDiff} transactions`}
                        </p>
                        <p className="text-xs text-gray-600">
                          {new Date(data.timestamp).toLocaleTimeString()}
                        </p>
                      </div>
                    </div>
                    <div className="text-right">
                      <p className="text-sm font-mono text-gray-900">
                        Block #{data.blocks}
                      </p>
                      <p className="text-xs text-gray-600">
                        {stats?.chain_name || 'Unknown'}
                      </p>
                    </div>
                  </div>
                </div>
              )
            })}
          </div>
        </div>
      )}

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
