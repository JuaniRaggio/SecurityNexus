'use client'

import { useState } from 'react'
import { AlertTriangle, CheckCircle, ExternalLink } from 'lucide-react'
import {
  useUnacknowledgedAlerts,
  useAcknowledgeAlert,
  formatAlertTime,
  getSeverityColor,
  type Alert,
} from '@/hooks/useMonitoring'

export default function AlertsPanel() {
  const { data: alerts = [], isLoading } = useUnacknowledgedAlerts(5000)
  const acknowledgeMutation = useAcknowledgeAlert()
  const [filter, setFilter] = useState<'all' | 'critical' | 'high' | 'medium' | 'low'>('all')

  const filteredAlerts = filter === 'all'
    ? alerts
    : alerts.filter(alert => alert.severity === filter)

  const handleAcknowledge = (alertId: string) => {
    acknowledgeMutation.mutate(alertId)
  }

  const getSeverityBadge = (severity: Alert['severity']) => {
    const color = getSeverityColor(severity)
    const bgColor = {
      red: 'bg-red-100 text-red-800 border-red-200',
      orange: 'bg-orange-100 text-orange-800 border-orange-200',
      yellow: 'bg-yellow-100 text-yellow-800 border-yellow-200',
      blue: 'bg-blue-100 text-blue-800 border-blue-200',
      gray: 'bg-gray-100 text-gray-800 border-gray-200',
    }[color]

    return (
      <span className={`px-2 py-1 text-xs font-semibold rounded border ${bgColor} uppercase`}>
        {severity}
      </span>
    )
  }

  return (
    <div className="bg-white rounded-lg shadow">
      <div className="p-6 border-b border-gray-200">
        <div className="flex items-center justify-between">
          <div>
            <h2 className="text-xl font-semibold text-gray-900 flex items-center gap-2">
              <AlertTriangle className="w-5 h-5 text-orange-600" />
              Security Alerts
            </h2>
            <p className="text-sm text-gray-600 mt-1">
              Real-time security threat notifications
            </p>
          </div>
          <div className="flex items-center gap-2">
            <span className="text-sm text-gray-600">
              {filteredAlerts.length} unacknowledged
            </span>
          </div>
        </div>

        <div className="flex gap-2 mt-4">
          <button
            onClick={() => setFilter('all')}
            className={`px-3 py-1 text-sm rounded ${
              filter === 'all'
                ? 'bg-gray-900 text-white'
                : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
            }`}
          >
            All
          </button>
          <button
            onClick={() => setFilter('critical')}
            className={`px-3 py-1 text-sm rounded ${
              filter === 'critical'
                ? 'bg-red-600 text-white'
                : 'bg-red-100 text-red-700 hover:bg-red-200'
            }`}
          >
            Critical
          </button>
          <button
            onClick={() => setFilter('high')}
            className={`px-3 py-1 text-sm rounded ${
              filter === 'high'
                ? 'bg-orange-600 text-white'
                : 'bg-orange-100 text-orange-700 hover:bg-orange-200'
            }`}
          >
            High
          </button>
          <button
            onClick={() => setFilter('medium')}
            className={`px-3 py-1 text-sm rounded ${
              filter === 'medium'
                ? 'bg-yellow-600 text-white'
                : 'bg-yellow-100 text-yellow-700 hover:bg-yellow-200'
            }`}
          >
            Medium
          </button>
          <button
            onClick={() => setFilter('low')}
            className={`px-3 py-1 text-sm rounded ${
              filter === 'low'
                ? 'bg-blue-600 text-white'
                : 'bg-blue-100 text-blue-700 hover:bg-blue-200'
            }`}
          >
            Low
          </button>
        </div>
      </div>

      <div className="divide-y divide-gray-200 max-h-[600px] overflow-y-auto">
        {isLoading ? (
          <div className="p-8 text-center text-gray-500">
            <div className="animate-spin w-8 h-8 border-4 border-gray-300 border-t-gray-900 rounded-full mx-auto mb-2"></div>
            Loading alerts...
          </div>
        ) : filteredAlerts.length === 0 ? (
          <div className="p-8 text-center text-gray-500">
            <CheckCircle className="w-12 h-12 mx-auto mb-2 text-green-500" />
            <p className="font-medium">No {filter !== 'all' && filter} alerts</p>
            <p className="text-sm mt-1">All systems operating normally</p>
          </div>
        ) : (
          filteredAlerts.map((alert) => (
            <div
              key={alert.id}
              className={`p-4 hover:bg-gray-50 transition-colors ${
                acknowledgeMutation.variables === alert.id ? 'opacity-50' : ''
              }`}
            >
              <div className="flex items-start justify-between">
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2 mb-2">
                    {getSeverityBadge(alert.severity)}
                    <span className="text-xs text-gray-500">
                      {formatAlertTime(alert.timestamp)}
                    </span>
                  </div>

                  <h3 className="font-semibold text-gray-900 mb-1">
                    {alert.pattern.replace(/_/g, ' ')}
                  </h3>

                  <p className="text-sm text-gray-700 mb-2">{alert.description}</p>

                  {alert.transaction_hash && (
                    <div className="flex items-center gap-2 text-xs text-gray-600 mb-2">
                      <span className="font-medium">Transaction:</span>
                      <code className="bg-gray-100 px-2 py-1 rounded font-mono">
                        {alert.transaction_hash.slice(0, 10)}...{alert.transaction_hash.slice(-8)}
                      </code>
                      {alert.block_number && (
                        <span>Block #{alert.block_number}</span>
                      )}
                    </div>
                  )}

                  {alert.recommended_actions.length > 0 && (
                    <div className="mt-2">
                      <p className="text-xs font-medium text-gray-700 mb-1">
                        Recommended Actions:
                      </p>
                      <ul className="list-disc list-inside space-y-1">
                        {alert.recommended_actions.map((action, idx) => (
                          <li key={idx} className="text-xs text-gray-600">
                            {action}
                          </li>
                        ))}
                      </ul>
                    </div>
                  )}
                </div>

                <div className="flex flex-col gap-2 ml-4">
                  <button
                    onClick={() => handleAcknowledge(alert.id)}
                    disabled={acknowledgeMutation.isPending}
                    className="p-2 text-green-600 hover:bg-green-50 rounded transition-colors disabled:opacity-50"
                    title="Acknowledge alert"
                  >
                    <CheckCircle className="w-5 h-5" />
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
          ))
        )}
      </div>

      {filteredAlerts.length > 5 && (
        <div className="p-4 border-t border-gray-200 bg-gray-50 text-center">
          <p className="text-sm text-gray-600">
            Showing {filteredAlerts.length} alerts. Auto-refreshing every 5 seconds.
          </p>
        </div>
      )}
    </div>
  )
}
