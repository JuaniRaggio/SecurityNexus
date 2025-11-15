'use client'

import { AlertTriangle, AlertCircle, Info } from 'lucide-react'
import clsx from 'clsx'

interface Alert {
  id: string
  severity: 'critical' | 'high' | 'medium' | 'low'
  title: string
  description: string
  chain: string
  timestamp: string
}

const alerts: Alert[] = [
  {
    id: '1',
    severity: 'critical',
    title: 'Flash Loan Attack Detected',
    description: 'Suspicious flash loan pattern on Hydration DEX',
    chain: 'Hydration',
    timestamp: '2 minutes ago',
  },
  {
    id: '2',
    severity: 'high',
    title: 'Unchecked Arithmetic',
    description: 'Integer overflow vulnerability in pallet_token',
    chain: 'Polkadot',
    timestamp: '15 minutes ago',
  },
  {
    id: '3',
    severity: 'medium',
    title: 'Unusual Volume Spike',
    description: 'Trading volume 300% above average',
    chain: 'Kusama',
    timestamp: '1 hour ago',
  },
]

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
  return (
    <div className="bg-white rounded-lg shadow">
      <div className="p-6 border-b border-gray-200">
        <h2 className="text-xl font-bold text-gray-900">Active Alerts</h2>
        <p className="text-sm text-gray-600 mt-1">Real-time security alerts</p>
      </div>

      <div className="p-4 space-y-3 max-h-[600px] overflow-y-auto">
        {alerts.map((alert) => {
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
                    <span className="font-medium">{alert.chain}</span>
                    <span>{alert.timestamp}</span>
                  </div>
                </div>
              </div>
            </div>
          )
        })}
      </div>

      <div className="p-4 border-t border-gray-200">
        <button className="w-full py-2 text-sm font-medium text-blue-600 hover:text-blue-700 transition-colors">
          View All Alerts
        </button>
      </div>
    </div>
  )
}
