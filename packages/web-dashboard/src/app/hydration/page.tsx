'use client'

import { Droplet } from 'lucide-react'
import { Activity } from 'lucide-react'
import { AlertTriangle } from 'lucide-react'
import { Shield } from 'lucide-react'
import { TrendingDown } from 'lucide-react'
import { DollarSign } from 'lucide-react'
import { useState, useEffect } from 'react'
import toast from 'react-hot-toast'

interface AttackTrend {
  hour: string
  attack_pattern: string
  count: number
  avg_confidence: number
}

export default function HydrationPage() {
  const [trends, setTrends] = useState<AttackTrend[]>([])
  const [loading, setLoading] = useState(true)
  const [hours, setHours] = useState(24)

  useEffect(() => {
    fetchTrends()
  }, [hours])

  const fetchTrends = async () => {
    try {
      setLoading(true)
      // Try real API first, fallback to demo data
      let response = await fetch(`http://localhost:8080/api/analytics/attack-trends?hours=${hours}`)

      if (!response.ok) {
        // Fallback to demo data
        response = await fetch(`/api/demo-monitoring?hours=${hours}`)
      }

      if (response.ok) {
        const data = await response.json()
        setTrends(data.filter((t: AttackTrend) =>
          t.attack_pattern.includes('Omnipool') ||
          t.attack_pattern.includes('Liquidity') ||
          t.attack_pattern.includes('Collateral')
        ))
      }
    } catch (error) {
      // Use demo data on error
      try {
        const demoResponse = await fetch(`/api/demo-monitoring?hours=${hours}`)
        if (demoResponse.ok) {
          const data = await demoResponse.json()
          setTrends(data.filter((t: AttackTrend) =>
            t.attack_pattern.includes('Omnipool') ||
            t.attack_pattern.includes('Liquidity') ||
            t.attack_pattern.includes('Collateral')
          ))
        }
      } catch (demoError) {
        toast.error('Failed to fetch Hydration data')
      }
    } finally {
      setLoading(false)
    }
  }

  const omnipoolCount = trends.filter(t => t.attack_pattern.includes('Omnipool')).length
  const liquidityCount = trends.filter(t => t.attack_pattern.includes('Liquidity')).length
  const collateralCount = trends.filter(t => t.attack_pattern.includes('Collateral')).length
  const totalDetections = trends.reduce((sum, t) => sum + t.count, 0)

  return (
    <div className="space-y-8">
      {/* Header */}
      <div className="relative overflow-hidden rounded-2xl bg-gradient-to-r from-emerald-600 via-teal-600 to-cyan-600 p-8 shadow-xl">
        <div className="absolute inset-0 bg-black/10"></div>
        <div className="relative z-10">
          <div className="flex items-center gap-3 mb-2">
            <Droplet className="h-10 w-10 text-white" />
            <h1 className="text-4xl font-bold text-white">Hydration DeFi Monitoring</h1>
          </div>
          <p className="text-white/90 text-lg">
            Omnipool manipulation, liquidity drain, and collateral attack detection
          </p>
        </div>
      </div>

      {/* Time Range Selector */}
      <div className="flex gap-2">
        {[6, 12, 24, 48, 168].map((h) => (
          <button
            key={h}
            onClick={() => setHours(h)}
            className={`px-4 py-2 rounded-lg font-medium transition-all ${
              hours === h
                ? 'bg-emerald-600 text-white shadow-lg'
                : 'bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700'
            }`}
          >
            {h}h
          </button>
        ))}
      </div>

      {/* Stats Cards */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
        <div className="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-lg border border-gray-200 dark:border-gray-700">
          <div className="flex items-center justify-between mb-4">
            <div className="p-3 bg-emerald-100 dark:bg-emerald-900/30 rounded-lg">
              <Activity className="h-6 w-6 text-emerald-600 dark:text-emerald-400" />
            </div>
            <TrendingDown className="h-5 w-5 text-green-500" />
          </div>
          <h3 className="text-gray-600 dark:text-gray-400 text-sm font-medium">Total DeFi Detections</h3>
          <p className="text-3xl font-bold text-gray-900 dark:text-white mt-2">{totalDetections}</p>
        </div>

        <div className="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-lg border border-gray-200 dark:border-gray-700">
          <div className="flex items-center justify-between mb-4">
            <div className="p-3 bg-orange-100 dark:bg-orange-900/30 rounded-lg">
              <AlertTriangle className="h-6 w-6 text-orange-600 dark:text-orange-400" />
            </div>
          </div>
          <h3 className="text-gray-600 dark:text-gray-400 text-sm font-medium">Omnipool Attacks</h3>
          <p className="text-3xl font-bold text-gray-900 dark:text-white mt-2">{omnipoolCount}</p>
        </div>

        <div className="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-lg border border-gray-200 dark:border-gray-700">
          <div className="flex items-center justify-between mb-4">
            <div className="p-3 bg-red-100 dark:bg-red-900/30 rounded-lg">
              <TrendingDown className="h-6 w-6 text-red-600 dark:text-red-400" />
            </div>
          </div>
          <h3 className="text-gray-600 dark:text-gray-400 text-sm font-medium">Liquidity Drains</h3>
          <p className="text-3xl font-bold text-gray-900 dark:text-white mt-2">{liquidityCount}</p>
        </div>

        <div className="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-lg border border-gray-200 dark:border-gray-700">
          <div className="flex items-center justify-between mb-4">
            <div className="p-3 bg-purple-100 dark:bg-purple-900/30 rounded-lg">
              <DollarSign className="h-6 w-6 text-purple-600 dark:text-purple-400" />
            </div>
          </div>
          <h3 className="text-gray-600 dark:text-gray-400 text-sm font-medium">Collateral Issues</h3>
          <p className="text-3xl font-bold text-gray-900 dark:text-white mt-2">{collateralCount}</p>
        </div>
      </div>

      {/* Attack Trends Table */}
      <div className="bg-white dark:bg-gray-800 rounded-xl shadow-lg border border-gray-200 dark:border-gray-700 overflow-hidden">
        <div className="p-6 border-b border-gray-200 dark:border-gray-700">
          <h2 className="text-xl font-bold text-gray-900 dark:text-white">DeFi Attack Timeline</h2>
        </div>
        <div className="overflow-x-auto">
          {loading ? (
            <div className="p-12 text-center text-gray-500">Loading...</div>
          ) : trends.length === 0 ? (
            <div className="p-12 text-center text-gray-500">No DeFi detections in the last {hours} hours</div>
          ) : (
            <table className="w-full">
              <thead className="bg-gray-50 dark:bg-gray-900">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    Time
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    Attack Pattern
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    Count
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    Avg Confidence
                  </th>
                </tr>
              </thead>
              <tbody className="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
                {trends.map((trend, idx) => (
                  <tr key={idx} className="hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors">
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-300">
                      {new Date(trend.hour).toLocaleString()}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className={`px-3 py-1 rounded-full text-xs font-medium ${
                        trend.attack_pattern.includes('Omnipool')
                          ? 'bg-orange-100 text-orange-800 dark:bg-orange-900/30 dark:text-orange-400'
                          : trend.attack_pattern.includes('Liquidity')
                          ? 'bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-400'
                          : 'bg-purple-100 text-purple-800 dark:bg-purple-900/30 dark:text-purple-400'
                      }`}>
                        {trend.attack_pattern}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm font-semibold text-gray-900 dark:text-white">
                      {trend.count}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-300">
                      {(trend.avg_confidence * 100).toFixed(1)}%
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>
      </div>

      {/* DeFi Security Tips */}
      <div className="bg-gradient-to-r from-emerald-50 to-teal-50 dark:from-gray-800 dark:to-gray-900 rounded-xl p-6 border border-emerald-200 dark:border-gray-700">
        <div className="flex items-start gap-4">
          <div className="p-3 bg-emerald-100 dark:bg-emerald-900/30 rounded-lg">
            <Shield className="h-6 w-6 text-emerald-600 dark:text-emerald-400" />
          </div>
          <div className="flex-1">
            <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-2">DeFi Security Monitoring</h3>
            <p className="text-gray-700 dark:text-gray-300 text-sm mb-4">
              Our system actively monitors for Omnipool price manipulation, flash loan attacks,
              liquidity drain attempts, and collateral manipulation patterns in real-time.
            </p>
            <div className="flex gap-4">
              <a
                href={`http://localhost:8080/api/export/csv?hours=${hours}`}
                className="px-6 py-3 bg-emerald-600 text-white rounded-lg font-medium hover:bg-emerald-700 transition-colors shadow-lg"
              >
                Download CSV
              </a>
              <a
                href={`http://localhost:8080/api/export/json?hours=${hours}`}
                className="px-6 py-3 bg-teal-600 text-white rounded-lg font-medium hover:bg-teal-700 transition-colors shadow-lg"
              >
                Download JSON
              </a>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
