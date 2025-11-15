'use client'

import { AlertTriangle, AlertCircle, Info } from 'lucide-react'
import clsx from 'clsx'
import { useAlerts } from '@/hooks/useAnalysis'
import { formatDistanceToNow } from 'date-fns'

const severityConfig = {
  critical: {
    icon: AlertTriangle,
    bgColor: 'bg-red-100',
    textColor: 'text-red-700',
    borderColor: 'border-red-200',
  },
  high: {
    icon: AlertCircle,
    bgColor: 'bg-orange-100',
    textColor: 'text-orange-700',
    borderColor: 'border-orange-200',
  },
  medium: {
    icon: Info,
    bgColor: 'bg-yellow-100',
    textColor: 'text-yellow-700',
    borderColor: 'border-yellow-200',
  },
  low: {
    icon: Info,
    bgColor: 'bg-blue-100',
    textColor: 'text-blue-700',
    borderColor: 'border-blue-200',
  },
}

export default function AlertsPanel() {
  const { data: alertsData, isLoading } = useAlerts(10);
  const alerts = alertsData?.alerts || [];

  return (
    <div className="bg-white rounded-lg shadow">
      <div className="p-6 border-b border-gray-200">
        <h2 className="text-xl font-bold text-gray-900">Active Alerts</h2>
        <p className="text-sm text-gray-600 mt-1">
          {alerts.length > 0 ? `${alerts.length} security alert${alerts.length !== 1 ? 's' : ''}` : 'No active alerts'}
        </p>
      </div>

      <div className="p-4 space-y-3 max-h-[600px] overflow-y-auto">
        {isLoading ? (
          <div className="text-center py-8 text-gray-500">
            Loading alerts...
          </div>
        ) : alerts.length === 0 ? (
          <div className="text-center py-8 text-gray-500">
            All clear! No security alerts at this time.
          </div>
        ) : (
          alerts.map((alert) => {
            const config = severityConfig[alert.severity]
            const Icon = config.icon

            return (
              <div
                key={alert.id}
                className={clsx(
                  'p-4 rounded-lg border-l-4',
                  config.bgColor,
                  config.borderColor
                )}
              >
                <div className="flex items-start gap-3">
                  <Icon className={clsx('w-5 h-5 mt-0.5', config.textColor)} />
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center justify-between gap-2">
                      <h3 className="font-semibold text-gray-900 text-sm">
                        {alert.title}
                      </h3>
                      <span
                        className={clsx(
                          'text-xs font-medium px-2 py-1 rounded',
                          config.bgColor,
                          config.textColor
                        )}
                      >
                        {alert.severity.toUpperCase()}
                      </span>
                    </div>
                    <p className="text-sm text-gray-600 mt-1">
                      {alert.description}
                    </p>
                    <div className="flex items-center gap-3 mt-2 text-xs text-gray-500">
                      <span className="font-medium">{alert.file}</span>
                      <span>{formatDistanceToNow(new Date(alert.timestamp), { addSuffix: true })}</span>
                    </div>
                  </div>
                </div>
              </div>
            )
          })
        )}
      </div>

      {alerts.length > 0 && (
        <div className="p-4 border-t border-gray-200">
          <button className="w-full py-2 text-sm font-medium text-blue-600 hover:text-blue-700 transition-colors">
            View All Alerts
          </button>
        </div>
      )}
    </div>
  )
}
