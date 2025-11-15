'use client'

import { Activity } from 'lucide-react'
import clsx from 'clsx'

interface Chain {
  id: string
  name: string
  status: 'online' | 'offline' | 'degraded'
  blocksProcessed: number
  transactionsAnalyzed: number
  alerts: number
}

const chains: Chain[] = [
  {
    id: '1',
    name: 'Polkadot',
    status: 'online',
    blocksProcessed: 12453,
    transactionsAnalyzed: 45231,
    alerts: 0,
  },
  {
    id: '2',
    name: 'Kusama',
    status: 'online',
    blocksProcessed: 23412,
    transactionsAnalyzed: 67123,
    alerts: 1,
  },
  {
    id: '3',
    name: 'Hydration',
    status: 'degraded',
    blocksProcessed: 8932,
    transactionsAnalyzed: 23451,
    alerts: 2,
  },
  {
    id: '4',
    name: 'AssetHub',
    status: 'online',
    blocksProcessed: 15234,
    transactionsAnalyzed: 34521,
    alerts: 0,
  },
]

const statusConfig = {
  online: {
    color: 'bg-green-500',
    textColor: 'text-green-700',
    label: 'Online',
  },
  offline: {
    color: 'bg-gray-500',
    textColor: 'text-gray-700',
    label: 'Offline',
  },
  degraded: {
    color: 'bg-yellow-500',
    textColor: 'text-yellow-700',
    label: 'Degraded',
  },
}

export default function ChainStatus() {
  return (
    <div className="bg-white rounded-lg shadow">
      <div className="p-6 border-b border-gray-200">
        <h2 className="text-xl font-bold text-gray-900">Chain Status</h2>
        <p className="text-sm text-gray-600 mt-1">
          Real-time monitoring across parachains
        </p>
      </div>

      <div className="p-6">
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {chains.map((chain) => {
            const config = statusConfig[chain.status]

            return (
              <div
                key={chain.id}
                className="p-4 border border-gray-200 rounded-lg hover:border-blue-300 transition-colors"
              >
                <div className="flex items-center justify-between mb-3">
                  <div className="flex items-center gap-3">
                    <Activity className="w-5 h-5 text-gray-600" />
                    <h3 className="font-semibold text-gray-900">{chain.name}</h3>
                  </div>
                  <div className="flex items-center gap-2">
                    <div className={clsx('w-2 h-2 rounded-full', config.color)} />
                    <span className={clsx('text-sm font-medium', config.textColor)}>
                      {config.label}
                    </span>
                  </div>
                </div>

                <div className="space-y-2">
                  <div className="flex justify-between text-sm">
                    <span className="text-gray-600">Blocks Processed</span>
                    <span className="font-medium text-gray-900">
                      {chain.blocksProcessed.toLocaleString()}
                    </span>
                  </div>
                  <div className="flex justify-between text-sm">
                    <span className="text-gray-600">Transactions</span>
                    <span className="font-medium text-gray-900">
                      {chain.transactionsAnalyzed.toLocaleString()}
                    </span>
                  </div>
                  <div className="flex justify-between text-sm">
                    <span className="text-gray-600">Active Alerts</span>
                    <span
                      className={clsx(
                        'font-medium',
                        chain.alerts > 0 ? 'text-red-600' : 'text-green-600'
                      )}
                    >
                      {chain.alerts}
                    </span>
                  </div>
                </div>
              </div>
            )
          })}
        </div>
      </div>
    </div>
  )
}
