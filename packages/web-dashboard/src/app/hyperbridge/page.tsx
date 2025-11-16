'use client'

import { GitBranch } from 'lucide-react'
import { Activity } from 'lucide-react'
import { AlertTriangle } from 'lucide-react'
import { CheckCircle2 } from 'lucide-react'
import { TrendingUp } from 'lucide-react'
import { useState, useEffect } from 'react'
import toast from 'react-hot-toast'

interface AttackTrend {
  hour: string
  attack_pattern: string
  count: number
  avg_confidence: number
}

export default function HyperbridgePage() {
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
          t.attack_pattern.includes('CrossChain') ||
          t.attack_pattern.includes('StateProof')
        ))
      }
    } catch (error) {
      // Use demo data on error
      try {
        const demoResponse = await fetch(`/api/demo-monitoring?hours=${hours}`)
        if (demoResponse.ok) {
          const data = await demoResponse.json()
          setTrends(data.filter((t: AttackTrend) =>
            t.attack_pattern.includes('CrossChain') ||
            t.attack_pattern.includes('StateProof')
          ))
        }
      } catch (demoError) {
        toast.error('Failed to fetch Hyperbridge data')
      }
    } finally {
      setLoading(false)
    }
  }

  const crossChainCount = trends.filter(t => t.attack_pattern.includes('CrossChain')).length
  const proofCount = trends.filter(t => t.attack_pattern.includes('StateProof')).length
  const totalDetections = trends.reduce((sum, t) => sum + t.count, 0)

  return (
    <div className="space-y-8">
      {/* Header */}
      <div className="relative overflow-hidden rounded-2xl bg-gradient-to-r from-cyan-600 via-blue-600 to-indigo-600 p-8 shadow-xl">
        <div className="absolute inset-0 bg-black/10"></div>
        <div className="relative z-10">
          <div className="flex items-center gap-3 mb-2">
            <GitBranch className="h-10 w-10 text-white" />
            <h1 className="text-4xl font-bold text-white">Hyperbridge Monitoring</h1>
          </div>
          <p className="text-white/90 text-lg">
            Cross-chain message monitoring and state proof verification
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
                ? 'bg-blue-600 text-white shadow-lg'
                : 'bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700'
            }`}
          >
            {h}h
          </button>
        ))}
      </div>

      {/* Stats Cards */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-lg border border-gray-200 dark:border-gray-700">
          <div className="flex items-center justify-between mb-4">
            <div className="p-3 bg-blue-100 dark:bg-blue-900/30 rounded-lg">
              <Activity className="h-6 w-6 text-blue-600 dark:text-blue-400" />
            </div>
            <TrendingUp className="h-5 w-5 text-green-500" />
          </div>
          <h3 className="text-gray-600 dark:text-gray-400 text-sm font-medium">Total Detections</h3>
          <p className="text-3xl font-bold text-gray-900 dark:text-white mt-2">{totalDetections}</p>
        </div>

        <div className="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-lg border border-gray-200 dark:border-gray-700">
          <div className="flex items-center justify-between mb-4">
            <div className="p-3 bg-orange-100 dark:bg-orange-900/30 rounded-lg">
              <AlertTriangle className="h-6 w-6 text-orange-600 dark:text-orange-400" />
            </div>
          </div>
          <h3 className="text-gray-600 dark:text-gray-400 text-sm font-medium">Bridge Attacks</h3>
          <p className="text-3xl font-bold text-gray-900 dark:text-white mt-2">{crossChainCount}</p>
        </div>

        <div className="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-lg border border-gray-200 dark:border-gray-700">
          <div className="flex items-center justify-between mb-4">
            <div className="p-3 bg-purple-100 dark:bg-purple-900/30 rounded-lg">
              <CheckCircle2 className="h-6 w-6 text-purple-600 dark:text-purple-400" />
            </div>
          </div>
          <h3 className="text-gray-600 dark:text-gray-400 text-sm font-medium">Proof Issues</h3>
          <p className="text-3xl font-bold text-gray-900 dark:text-white mt-2">{proofCount}</p>
        </div>
      </div>

      {/* Attack Trends Table */}
      <div className="bg-white dark:bg-gray-800 rounded-xl shadow-lg border border-gray-200 dark:border-gray-700 overflow-hidden">
        <div className="p-6 border-b border-gray-200 dark:border-gray-700">
          <h2 className="text-xl font-bold text-gray-900 dark:text-white">Detection Timeline</h2>
        </div>
        <div className="overflow-x-auto">
          {loading ? (
            <div className="p-12 text-center text-gray-500">Loading...</div>
          ) : trends.length === 0 ? (
            <div className="p-12 text-center text-gray-500">No detections in the last {hours} hours</div>
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
                        trend.attack_pattern.includes('CrossChain')
                          ? 'bg-orange-100 text-orange-800 dark:bg-orange-900/30 dark:text-orange-400'
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

      {/* Export Section */}
      <div className="bg-gradient-to-r from-blue-50 to-indigo-50 dark:from-gray-800 dark:to-gray-900 rounded-xl p-6 border border-blue-200 dark:border-gray-700">
        <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">Export Data</h3>
        <div className="flex gap-4">
          <a
            href={`http://localhost:8080/api/export/csv?hours=${hours}`}
            className="px-6 py-3 bg-blue-600 text-white rounded-lg font-medium hover:bg-blue-700 transition-colors shadow-lg"
          >
            Download CSV
          </a>
          <a
            href={`http://localhost:8080/api/export/json?hours=${hours}`}
            className="px-6 py-3 bg-indigo-600 text-white rounded-lg font-medium hover:bg-indigo-700 transition-colors shadow-lg"
          >
            Download JSON
          </a>
        </div>
      </div>
    </div>
  )
}
