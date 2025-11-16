'use client'

import { useState } from 'react'
import {
  AlertTriangle,
  Shield,
  TrendingUp,
  Search,
  Filter,
  Download,
  Eye,
  CheckCircle,
  ExternalLink,
  Calendar,
  Lock,
  BarChart3,
  ChevronDown,
  ChevronUp,
} from 'lucide-react'
import {
  useAlerts,
  useAcknowledgeAlert,
  formatAlertTime,
  getSeverityColor,
  type Alert,
} from '@/hooks/useMonitoring'
import { useRouter } from 'next/navigation'

type FilterType = 'all' | 'critical' | 'high' | 'medium' | 'low'
type TimeFilter = 'all' | '1h' | '24h' | '7d' | '30d'

export default function AlertsPage() {
  const router = useRouter()
  const [severityFilter, setSeverityFilter] = useState<FilterType>('all')
  const [timeFilter, setTimeFilter] = useState<TimeFilter>('24h')
  const [searchQuery, setSearchQuery] = useState('')
  const [showStats, setShowStats] = useState(true)
  const [expandedAlerts, setExpandedAlerts] = useState<Set<string>>(new Set())
  const [demoMode, setDemoMode] = useState(false)

  const { data: allAlerts = [], isLoading } = useAlerts(5000, demoMode)
  const acknowledgeMutation = useAcknowledgeAlert()

  const toggleAlertExpansion = (alertId: string) => {
    setExpandedAlerts((prev) => {
      const newSet = new Set(prev)
      if (newSet.has(alertId)) {
        newSet.delete(alertId)
      } else {
        newSet.add(alertId)
      }
      return newSet
    })
  }

  // Filter alerts
  const filteredAlerts = allAlerts.filter((alert) => {
    // Severity filter
    if (severityFilter !== 'all' && alert.severity !== severityFilter) {
      return false
    }

    // Time filter
    const now = Date.now()
    const alertTime = alert.timestamp
    const timeFilters = {
      '1h': 60 * 60 * 1000,
      '24h': 24 * 60 * 60 * 1000,
      '7d': 7 * 24 * 60 * 60 * 1000,
      '30d': 30 * 24 * 60 * 60 * 1000,
    }
    if (timeFilter !== 'all' && now - alertTime > timeFilters[timeFilter]) {
      return false
    }

    // Search filter
    if (
      searchQuery &&
      !alert.pattern.toLowerCase().includes(searchQuery.toLowerCase()) &&
      !alert.description.toLowerCase().includes(searchQuery.toLowerCase())
    ) {
      return false
    }

    return true
  })

  // Calculate statistics
  const stats = {
    total: allAlerts.length,
    critical: allAlerts.filter((a) => a.severity === 'critical').length,
    high: allAlerts.filter((a) => a.severity === 'high').length,
    medium: allAlerts.filter((a) => a.severity === 'medium').length,
    low: allAlerts.filter((a) => a.severity === 'low').length,
    acknowledged: allAlerts.filter((a) => a.acknowledged).length,
    unacknowledged: allAlerts.filter((a) => !a.acknowledged).length,
  }

  // Pattern distribution
  const patternStats = allAlerts.reduce(
    (acc, alert) => {
      acc[alert.pattern] = (acc[alert.pattern] || 0) + 1
      return acc
    },
    {} as Record<string, number>
  )

  const topPatterns = Object.entries(patternStats)
    .sort(([, a], [, b]) => b - a)
    .slice(0, 5)

  const getSeverityBadge = (severity: Alert['severity']) => {
    const color = getSeverityColor(severity)
    const styles = {
      red: 'bg-red-100 text-red-800 border-red-200',
      orange: 'bg-orange-100 text-orange-800 border-orange-200',
      yellow: 'bg-yellow-100 text-yellow-800 border-yellow-200',
      blue: 'bg-blue-100 text-blue-800 border-blue-200',
      gray: 'bg-gray-100 text-gray-800 border-gray-200',
    }[color]

    return (
      <span
        className={`px-2 py-1 text-xs font-semibold rounded border ${styles} uppercase`}
      >
        {severity}
      </span>
    )
  }

  const handleCreateZKPReport = (alert: Alert) => {
    // Navigate to privacy page with pre-filled data
    const reportData = {
      severity: alert.severity,
      category: alert.pattern.replace(/_/g, ' '),
      description: alert.description,
      affectedCode: alert.transaction_hash || '',
    }
    router.push(
      `/privacy?prefill=${encodeURIComponent(JSON.stringify(reportData))}`
    )
  }

  const handleExportAlerts = () => {
    const csvContent = [
      ['Timestamp', 'Severity', 'Pattern', 'Description', 'Transaction', 'Block', 'Status'].join(
        ','
      ),
      ...filteredAlerts.map((alert) =>
        [
          new Date(alert.timestamp).toISOString(),
          alert.severity,
          alert.pattern,
          `"${alert.description}"`,
          alert.transaction_hash || '',
          alert.block_number || '',
          alert.acknowledged ? 'Acknowledged' : 'Unacknowledged',
        ].join(',')
      ),
    ].join('\n')

    const blob = new Blob([csvContent], { type: 'text/csv' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `security-alerts-${new Date().toISOString()}.csv`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
  }

  return (
    <div className="space-y-6">
      {/* Header with gradient */}
      <div className="relative overflow-hidden rounded-2xl bg-gradient-to-r from-red-500 via-orange-500 to-yellow-500 p-8 shadow-xl">
        <div className="absolute inset-0 bg-black/10"></div>
        <div className="relative z-10">
          <div className="flex items-center justify-between">
            <div>
              <div className="flex items-center gap-3 mb-2">
                <Shield className="h-10 w-10 text-white" />
                <h1 className="text-4xl font-bold text-white">Security Alerts</h1>
              </div>
              <p className="text-white/90 text-lg">
                Comprehensive security threat monitoring and alert management
              </p>
            </div>
            <div className="flex flex-col gap-2 text-right">
              <div className="px-4 py-2 bg-white/20 backdrop-blur-sm rounded-lg border border-white/30">
                <p className="text-white text-2xl font-bold">{stats.total}</p>
                <p className="text-white/80 text-sm">Total Alerts</p>
              </div>
            </div>
          </div>
        </div>
        <div className="absolute -right-12 -top-12 h-64 w-64 rounded-full bg-white/10 blur-3xl"></div>
      </div>

      {/* Statistics Grid */}
      {showStats && (
        <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
          <div className="bg-gradient-to-br from-red-50 to-red-100 rounded-xl p-6 border border-red-200">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-red-600 font-medium">Critical</p>
                <p className="text-3xl font-bold text-red-700 mt-1">{stats.critical}</p>
              </div>
              <AlertTriangle className="w-8 h-8 text-red-600" />
            </div>
          </div>
          <div className="bg-gradient-to-br from-orange-50 to-orange-100 rounded-xl p-6 border border-orange-200">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-orange-600 font-medium">High</p>
                <p className="text-3xl font-bold text-orange-700 mt-1">{stats.high}</p>
              </div>
              <TrendingUp className="w-8 h-8 text-orange-600" />
            </div>
          </div>
          <div className="bg-gradient-to-br from-yellow-50 to-yellow-100 rounded-xl p-6 border border-yellow-200">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-yellow-600 font-medium">Medium</p>
                <p className="text-3xl font-bold text-yellow-700 mt-1">{stats.medium}</p>
              </div>
              <Filter className="w-8 h-8 text-yellow-600" />
            </div>
          </div>
          <div className="bg-gradient-to-br from-green-50 to-green-100 rounded-xl p-6 border border-green-200">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-green-600 font-medium">Acknowledged</p>
                <p className="text-3xl font-bold text-green-700 mt-1">{stats.acknowledged}</p>
              </div>
              <CheckCircle className="w-8 h-8 text-green-600" />
            </div>
          </div>
        </div>
      )}

      {/* Top Attack Patterns */}
      {showStats && topPatterns.length > 0 && (
        <div className="bg-white rounded-xl shadow-lg p-6 border border-gray-100">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-semibold text-gray-900 flex items-center gap-2">
              <BarChart3 className="w-5 h-5 text-blue-600" />
              Top Attack Patterns
            </h3>
          </div>
          <div className="space-y-3">
            {topPatterns.map(([pattern, count]) => (
              <div key={pattern} className="flex items-center justify-between">
                <span className="text-sm font-medium text-gray-700">
                  {pattern.replace(/_/g, ' ')}
                </span>
                <div className="flex items-center gap-3">
                  <div className="w-32 bg-gray-200 rounded-full h-2">
                    <div
                      className="bg-gradient-to-r from-blue-500 to-purple-500 h-2 rounded-full"
                      style={{ width: `${(count / allAlerts.length) * 100}%` }}
                    ></div>
                  </div>
                  <span className="text-sm font-bold text-gray-900 w-8 text-right">
                    {count}
                  </span>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Filters and Actions */}
      <div className="bg-white rounded-xl shadow-lg p-6 border border-gray-100">
        <div className="flex flex-col md:flex-row gap-4 items-start md:items-center justify-between">
          <div className="flex-1 w-full md:w-auto">
            <div className="relative">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-5 h-5 text-gray-400" />
              <input
                type="text"
                placeholder="Search alerts..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              />
            </div>
          </div>

          <div className="flex flex-wrap gap-2">
            <button
              onClick={() => setTimeFilter('1h')}
              className={`px-3 py-2 text-sm rounded-lg font-medium transition-all ${
                timeFilter === '1h'
                  ? 'bg-blue-600 text-white'
                  : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
              }`}
            >
              <Calendar className="w-4 h-4 inline mr-1" />
              1h
            </button>
            <button
              onClick={() => setTimeFilter('24h')}
              className={`px-3 py-2 text-sm rounded-lg font-medium transition-all ${
                timeFilter === '24h'
                  ? 'bg-blue-600 text-white'
                  : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
              }`}
            >
              24h
            </button>
            <button
              onClick={() => setTimeFilter('7d')}
              className={`px-3 py-2 text-sm rounded-lg font-medium transition-all ${
                timeFilter === '7d'
                  ? 'bg-blue-600 text-white'
                  : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
              }`}
            >
              7d
            </button>
            <button
              onClick={() => setTimeFilter('all')}
              className={`px-3 py-2 text-sm rounded-lg font-medium transition-all ${
                timeFilter === 'all'
                  ? 'bg-blue-600 text-white'
                  : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
              }`}
            >
              All
            </button>
          </div>

          <div className="flex gap-2">
            {process.env.NODE_ENV === 'development' && (
              <button
                onClick={() => setDemoMode(!demoMode)}
                className={`px-4 py-2 rounded-lg font-medium transition-all flex items-center gap-2 ${
                  demoMode
                    ? 'bg-purple-600 text-white hover:bg-purple-700'
                    : 'bg-purple-100 text-purple-700 hover:bg-purple-200'
                }`}
              >
                <Shield className="w-4 h-4" />
                {demoMode ? 'Exit' : 'Demo'} Mode
              </button>
            )}
            <button
              onClick={() => setShowStats(!showStats)}
              className="px-4 py-2 bg-purple-100 text-purple-700 hover:bg-purple-200 rounded-lg font-medium transition-all flex items-center gap-2"
            >
              <Eye className="w-4 h-4" />
              {showStats ? 'Hide' : 'Show'} Stats
            </button>
            <button
              onClick={handleExportAlerts}
              className="px-4 py-2 bg-green-600 text-white hover:bg-green-700 rounded-lg font-medium transition-all flex items-center gap-2"
            >
              <Download className="w-4 h-4" />
              Export CSV
            </button>
          </div>
        </div>

        {/* Severity Filters */}
        <div className="flex gap-2 mt-4 pt-4 border-t border-gray-200">
          <button
            onClick={() => setSeverityFilter('all')}
            className={`px-3 py-1 text-sm rounded ${
              severityFilter === 'all'
                ? 'bg-gray-900 text-white'
                : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
            }`}
          >
            All
          </button>
          <button
            onClick={() => setSeverityFilter('critical')}
            className={`px-3 py-1 text-sm rounded ${
              severityFilter === 'critical'
                ? 'bg-red-600 text-white'
                : 'bg-red-100 text-red-700 hover:bg-red-200'
            }`}
          >
            Critical
          </button>
          <button
            onClick={() => setSeverityFilter('high')}
            className={`px-3 py-1 text-sm rounded ${
              severityFilter === 'high'
                ? 'bg-orange-600 text-white'
                : 'bg-orange-100 text-orange-700 hover:bg-orange-200'
            }`}
          >
            High
          </button>
          <button
            onClick={() => setSeverityFilter('medium')}
            className={`px-3 py-1 text-sm rounded ${
              severityFilter === 'medium'
                ? 'bg-yellow-600 text-white'
                : 'bg-yellow-100 text-yellow-700 hover:bg-yellow-200'
            }`}
          >
            Medium
          </button>
          <button
            onClick={() => setSeverityFilter('low')}
            className={`px-3 py-1 text-sm rounded ${
              severityFilter === 'low'
                ? 'bg-blue-600 text-white'
                : 'bg-blue-100 text-blue-700 hover:bg-blue-200'
            }`}
          >
            Low
          </button>
        </div>
      </div>

      {/* Alerts List */}
      <div className="bg-white rounded-xl shadow-lg border border-gray-100">
        <div className="p-6 border-b border-gray-200">
          <div className="flex items-center justify-between">
            <h2 className="text-xl font-semibold text-gray-900">
              Alert History ({filteredAlerts.length})
            </h2>
            <span className="text-sm text-gray-600">
              {stats.unacknowledged} unacknowledged
            </span>
          </div>
        </div>

        <div className="divide-y divide-gray-200">
          {isLoading ? (
            <div className="p-8 text-center text-gray-500">
              <div className="animate-spin w-8 h-8 border-4 border-gray-300 border-t-gray-900 rounded-full mx-auto mb-2"></div>
              Loading alerts...
            </div>
          ) : filteredAlerts.length === 0 ? (
            <div className="p-8 text-center text-gray-500">
              <CheckCircle className="w-12 h-12 mx-auto mb-2 text-green-500" />
              <p className="font-medium">No alerts found</p>
              <p className="text-sm mt-1">
                {searchQuery
                  ? 'Try adjusting your search filters'
                  : 'All systems operating normally'}
              </p>
            </div>
          ) : (
            filteredAlerts.map((alert) => {
              const isExpanded = expandedAlerts.has(alert.id)
              return (
                <div
                  key={alert.id}
                  className={`p-5 hover:bg-gray-50 transition-colors ${
                    alert.acknowledged ? 'opacity-60' : ''
                  }`}
                >
                  <div className="flex items-start justify-between gap-4">
                    <div className="flex-1 min-w-0">
                      <div className="flex items-center gap-2 mb-2">
                        {getSeverityBadge(alert.severity)}
                        <span className="text-xs text-gray-500">
                          {formatAlertTime(alert.timestamp)}
                        </span>
                        {alert.acknowledged && (
                          <span className="px-2 py-1 text-xs font-semibold bg-green-100 text-green-800 border border-green-200 rounded">
                            ACKNOWLEDGED
                          </span>
                        )}
                        <span className="px-2 py-1 text-xs font-semibold bg-blue-100 text-blue-800 border border-blue-200 rounded">
                          {Math.round(alert.confidence * 100)}% confidence
                        </span>
                      </div>

                      <div className="flex items-center justify-between">
                        <h3 className="font-semibold text-gray-900 mb-1">
                          {alert.pattern.replace(/_/g, ' ')}
                        </h3>
                        <button
                          onClick={() => toggleAlertExpansion(alert.id)}
                          className="flex items-center gap-1 text-sm text-gray-600 hover:text-gray-900 transition-colors"
                        >
                          {isExpanded ? (
                            <>
                              <span>Hide Details</span>
                              <ChevronUp className="w-4 h-4" />
                            </>
                          ) : (
                            <>
                              <span>Show Details</span>
                              <ChevronDown className="w-4 h-4" />
                            </>
                          )}
                        </button>
                      </div>

                      <p className="text-sm text-gray-700 mb-2">{alert.description}</p>

                      {alert.transaction_hash && (
                        <div className="flex items-center gap-2 text-xs text-gray-600 mb-2">
                          <span className="font-medium">Transaction:</span>
                          <code className="bg-gray-100 px-2 py-1 rounded font-mono">
                            {alert.transaction_hash.slice(0, 10)}...
                            {alert.transaction_hash.slice(-8)}
                          </code>
                          {alert.block_number && (
                            <span>Block #{alert.block_number}</span>
                          )}
                        </div>
                      )}

                      {/* Expandable Evidence Section */}
                      {isExpanded && (
                        <div className="mt-4 space-y-3 border-t border-gray-200 pt-3">
                          {/* Evidence */}
                          {alert.evidence && alert.evidence.length > 0 && (
                            <div className="bg-blue-50 rounded-lg p-3 border border-blue-200">
                              <p className="text-sm font-semibold text-blue-900 mb-2 flex items-center gap-2">
                                <Shield className="w-4 h-4" />
                                Detection Evidence
                              </p>
                              <ul className="space-y-1.5">
                                {alert.evidence.map((item, idx) => (
                                  <li key={idx} className="text-xs text-blue-800 flex items-start gap-2">
                                    <span className="text-blue-500 mt-0.5">•</span>
                                    <span>{item}</span>
                                  </li>
                                ))}
                              </ul>
                            </div>
                          )}

                          {/* Recommended Actions */}
                          {alert.recommended_actions.length > 0 && (
                            <div className="bg-yellow-50 rounded-lg p-3 border border-yellow-200">
                              <p className="text-sm font-semibold text-yellow-900 mb-2 flex items-center gap-2">
                                <AlertTriangle className="w-4 h-4" />
                                Recommended Actions
                              </p>
                              <ul className="space-y-1.5">
                                {alert.recommended_actions.map((action, idx) => (
                                  <li key={idx} className="text-xs text-yellow-800 flex items-start gap-2">
                                    <span className="text-yellow-500 mt-0.5">→</span>
                                    <span>{action}</span>
                                  </li>
                                ))}
                              </ul>
                            </div>
                          )}

                          {/* Technical Details */}
                          <div className="bg-gray-50 rounded-lg p-3 border border-gray-200">
                            <p className="text-sm font-semibold text-gray-900 mb-2">
                              Technical Details
                            </p>
                            <div className="grid grid-cols-2 gap-2 text-xs">
                              <div>
                                <span className="text-gray-600">Pattern:</span>
                                <span className="ml-2 font-mono text-gray-900">{alert.pattern}</span>
                              </div>
                              <div>
                                <span className="text-gray-600">Confidence:</span>
                                <span className="ml-2 font-mono text-gray-900">
                                  {(alert.confidence * 100).toFixed(2)}%
                                </span>
                              </div>
                              {alert.chain_name && (
                                <div>
                                  <span className="text-gray-600">Chain:</span>
                                  <span className="ml-2 font-mono text-gray-900">{alert.chain_name}</span>
                                </div>
                              )}
                              <div>
                                <span className="text-gray-600">Alert ID:</span>
                                <span className="ml-2 font-mono text-gray-900">{alert.id.slice(0, 8)}</span>
                              </div>
                            </div>
                          </div>
                        </div>
                      )}
                    </div>

                    <div className="flex flex-col gap-2">
                      {!alert.acknowledged && (
                        <button
                          onClick={() => acknowledgeMutation.mutate(alert.id)}
                          disabled={acknowledgeMutation.isPending}
                          className="p-2 text-green-600 hover:bg-green-50 rounded transition-colors disabled:opacity-50"
                          title="Acknowledge alert"
                        >
                          <CheckCircle className="w-5 h-5" />
                        </button>
                      )}
                      <button
                        onClick={() => handleCreateZKPReport(alert)}
                        className="p-2 text-purple-600 hover:bg-purple-50 rounded transition-colors"
                        title="Create anonymous ZKP report"
                      >
                        <Lock className="w-5 h-5" />
                      </button>
                      {alert.transaction_hash && (
                        <a
                          href={`https://polkadot.js.org/apps/#/explorer/query/${alert.transaction_hash}`}
                          target="_blank"
                          rel="noopener noreferrer"
                          className="p-2 text-blue-600 hover:bg-blue-50 rounded transition-colors"
                          title="View on explorer"
                        >
                          <ExternalLink className="w-5 h-5" />
                        </a>
                      )}
                    </div>
                  </div>
                </div>
              )
            })
          )}
        </div>
      </div>
    </div>
  )
}
