'use client'

import { BarChart3 } from 'lucide-react'
import { TrendingUp } from 'lucide-react'
import { Shield } from 'lucide-react'
import { Zap } from 'lucide-react'
import { Database } from 'lucide-react'
import { Activity } from 'lucide-react'
import { useState, useEffect } from 'react'
import toast from 'react-hot-toast'

interface DetectorStat {
  detector_name: string
  chain: string
  attack_pattern: string
  total_detections: number
  avg_confidence: number
  critical_count: number
}

interface MLFeature {
  timestamp: string
  tx_hash: string
  caller: string
  pallet: string
  call_name: string
  features: any
}

export default function AnalyticsPage() {
  const [detectorStats, setDetectorStats] = useState<DetectorStat[]>([])
  const [mlFeatures, setMLFeatures] = useState<MLFeature[]>([])
  const [loading, setLoading] = useState(true)
  const [hours, setHours] = useState(24)

  useEffect(() => {
    fetchAnalytics()
  }, [hours])

  const fetchAnalytics = async () => {
    try {
      setLoading(true)

      const [statsResponse, mlResponse] = await Promise.all([
        fetch(`http://localhost:8080/api/analytics/detector-stats?hours=${hours}`),
        fetch(`http://localhost:8080/api/analytics/ml-features?limit=20`)
      ])

      if (statsResponse.ok) {
        const stats = await statsResponse.json()
        setDetectorStats(stats)
      }

      if (mlResponse.ok) {
        const ml = await mlResponse.json()
        setMLFeatures(ml)
      }
    } catch (error) {
      toast.error('Failed to fetch analytics data')
    } finally {
      setLoading(false)
    }
  }

  const totalDetections = detectorStats.reduce((sum, stat) => sum + stat.total_detections, 0)
  const avgConfidence = detectorStats.length > 0
    ? detectorStats.reduce((sum, stat) => sum + stat.avg_confidence, 0) / detectorStats.length
    : 0
  const criticalAlerts = detectorStats.reduce((sum, stat) => sum + stat.critical_count, 0)
  const activeDetectors = new Set(detectorStats.map(s => s.detector_name)).size

  return (
    <div className="space-y-8">
      {/* Header */}
      <div className="relative overflow-hidden rounded-2xl bg-gradient-to-r from-indigo-600 via-purple-600 to-pink-600 p-8 shadow-xl">
        <div className="absolute inset-0 bg-black/10"></div>
        <div className="relative z-10">
          <div className="flex items-center gap-3 mb-2">
            <BarChart3 className="h-10 w-10 text-white" />
            <h1 className="text-4xl font-bold text-white">Security Analytics</h1>
          </div>
          <p className="text-white/90 text-lg">
            Comprehensive analysis of detection patterns and ML-driven insights
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
                ? 'bg-indigo-600 text-white shadow-lg'
                : 'bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700'
            }`}
          >
            {h}h
          </button>
        ))}
      </div>

      {/* Overview Stats Cards */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
        <div className="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-lg border border-gray-200 dark:border-gray-700">
          <div className="flex items-center justify-between mb-4">
            <div className="p-3 bg-indigo-100 dark:bg-indigo-900/30 rounded-lg">
              <Activity className="h-6 w-6 text-indigo-600 dark:text-indigo-400" />
            </div>
            <TrendingUp className="h-5 w-5 text-green-500" />
          </div>
          <h3 className="text-gray-600 dark:text-gray-400 text-sm font-medium">Total Detections</h3>
          <p className="text-3xl font-bold text-gray-900 dark:text-white mt-2">{totalDetections}</p>
        </div>

        <div className="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-lg border border-gray-200 dark:border-gray-700">
          <div className="flex items-center justify-between mb-4">
            <div className="p-3 bg-purple-100 dark:bg-purple-900/30 rounded-lg">
              <Shield className="h-6 w-6 text-purple-600 dark:text-purple-400" />
            </div>
          </div>
          <h3 className="text-gray-600 dark:text-gray-400 text-sm font-medium">Avg Confidence</h3>
          <p className="text-3xl font-bold text-gray-900 dark:text-white mt-2">
            {(avgConfidence * 100).toFixed(1)}%
          </p>
        </div>

        <div className="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-lg border border-gray-200 dark:border-gray-700">
          <div className="flex items-center justify-between mb-4">
            <div className="p-3 bg-red-100 dark:bg-red-900/30 rounded-lg">
              <Zap className="h-6 w-6 text-red-600 dark:text-red-400" />
            </div>
          </div>
          <h3 className="text-gray-600 dark:text-gray-400 text-sm font-medium">Critical Alerts</h3>
          <p className="text-3xl font-bold text-gray-900 dark:text-white mt-2">{criticalAlerts}</p>
        </div>

        <div className="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-lg border border-gray-200 dark:border-gray-700">
          <div className="flex items-center justify-between mb-4">
            <div className="p-3 bg-blue-100 dark:bg-blue-900/30 rounded-lg">
              <Database className="h-6 w-6 text-blue-600 dark:text-blue-400" />
            </div>
          </div>
          <h3 className="text-gray-600 dark:text-gray-400 text-sm font-medium">Active Detectors</h3>
          <p className="text-3xl font-bold text-gray-900 dark:text-white mt-2">{activeDetectors}</p>
        </div>
      </div>

      {/* Detector Performance Table */}
      <div className="bg-white dark:bg-gray-800 rounded-xl shadow-lg border border-gray-200 dark:border-gray-700 overflow-hidden">
        <div className="p-6 border-b border-gray-200 dark:border-gray-700">
          <h2 className="text-xl font-bold text-gray-900 dark:text-white">Detector Performance</h2>
        </div>
        <div className="overflow-x-auto">
          {loading ? (
            <div className="p-12 text-center text-gray-500">Loading...</div>
          ) : detectorStats.length === 0 ? (
            <div className="p-12 text-center text-gray-500">No detector statistics available</div>
          ) : (
            <table className="w-full">
              <thead className="bg-gray-50 dark:bg-gray-900">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    Detector
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    Chain
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    Attack Pattern
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    Detections
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    Confidence
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    Critical
                  </th>
                </tr>
              </thead>
              <tbody className="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
                {detectorStats.map((stat, idx) => (
                  <tr key={idx} className="hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors">
                    <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white">
                      {stat.detector_name}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-700 dark:text-gray-300">
                      {stat.chain}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className="px-3 py-1 rounded-full text-xs font-medium bg-indigo-100 text-indigo-800 dark:bg-indigo-900/30 dark:text-indigo-400">
                        {stat.attack_pattern}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm font-semibold text-gray-900 dark:text-white">
                      {stat.total_detections}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-300">
                      {(stat.avg_confidence * 100).toFixed(1)}%
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className={`px-3 py-1 rounded-full text-xs font-medium ${
                        stat.critical_count > 0
                          ? 'bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-400'
                          : 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-400'
                      }`}>
                        {stat.critical_count}
                      </span>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>
      </div>

      {/* ML Feature Analysis */}
      <div className="bg-white dark:bg-gray-800 rounded-xl shadow-lg border border-gray-200 dark:border-gray-700 overflow-hidden">
        <div className="p-6 border-b border-gray-200 dark:border-gray-700">
          <h2 className="text-xl font-bold text-gray-900 dark:text-white">Recent ML Feature Extractions</h2>
          <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">
            Showing the latest {mlFeatures.length} transactions analyzed by our ML pipeline
          </p>
        </div>
        <div className="overflow-x-auto">
          {loading ? (
            <div className="p-12 text-center text-gray-500">Loading...</div>
          ) : mlFeatures.length === 0 ? (
            <div className="p-12 text-center text-gray-500">No ML features available</div>
          ) : (
            <table className="w-full">
              <thead className="bg-gray-50 dark:bg-gray-900">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    Timestamp
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    Transaction
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    Pallet
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    Call
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    Features Extracted
                  </th>
                </tr>
              </thead>
              <tbody className="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
                {mlFeatures.map((feature, idx) => (
                  <tr key={idx} className="hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors">
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-300">
                      {new Date(feature.timestamp).toLocaleString()}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-700 dark:text-gray-400">
                      {feature.tx_hash.substring(0, 10)}...
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-white">
                      {feature.pallet}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-700 dark:text-gray-300">
                      {feature.call_name}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className="px-3 py-1 rounded-full text-xs font-medium bg-purple-100 text-purple-800 dark:bg-purple-900/30 dark:text-purple-400">
                        33 features
                      </span>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>
      </div>

      {/* Export Section */}
      <div className="bg-gradient-to-r from-indigo-50 to-purple-50 dark:from-gray-800 dark:to-gray-900 rounded-xl p-6 border border-indigo-200 dark:border-gray-700">
        <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">Export Analytics Data</h3>
        <p className="text-gray-700 dark:text-gray-300 text-sm mb-4">
          Download complete detection data including transaction details, ML features, and detector statistics
        </p>
        <div className="flex gap-4">
          <a
            href={`http://localhost:8080/api/export/csv?hours=${hours}`}
            className="px-6 py-3 bg-indigo-600 text-white rounded-lg font-medium hover:bg-indigo-700 transition-colors shadow-lg"
          >
            Download CSV
          </a>
          <a
            href={`http://localhost:8080/api/export/json?hours=${hours}`}
            className="px-6 py-3 bg-purple-600 text-white rounded-lg font-medium hover:bg-purple-700 transition-colors shadow-lg"
          >
            Download JSON
          </a>
        </div>
      </div>
    </div>
  )
}
