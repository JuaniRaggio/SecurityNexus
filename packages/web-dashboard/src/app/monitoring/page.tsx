'use client'

import { Activity, TrendingUp, AlertTriangle, CheckCircle } from 'lucide-react'
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
  AreaChart,
  Area,
} from 'recharts'

const transactionData = [
  { time: '00:00', transactions: 4000, alerts: 2 },
  { time: '04:00', transactions: 3000, alerts: 1 },
  { time: '08:00', transactions: 5000, alerts: 3 },
  { time: '12:00', transactions: 7800, alerts: 5 },
  { time: '16:00', transactions: 6500, alerts: 2 },
  { time: '20:00', transactions: 5200, alerts: 1 },
]

const detectorStats = [
  { name: 'Flash Loan Detector', active: true, detections: 12, lastAlert: '2h ago' },
  { name: 'MEV Detector', active: true, detections: 8, lastAlert: '5h ago' },
  { name: 'Volume Anomaly', active: true, detections: 15, lastAlert: '1h ago' },
  { name: 'Reentrancy Detector', active: false, detections: 0, lastAlert: 'Never' },
]

export default function MonitoringPage() {
  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold text-gray-900">Real-Time Monitoring</h1>
        <p className="text-gray-600 mt-2">
          Monitor blockchain activity and detect security threats in real-time
        </p>
      </div>

      {/* Stats Grid */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-600">Transactions/Hour</p>
              <p className="text-2xl font-bold text-gray-900 mt-1">7,845</p>
            </div>
            <TrendingUp className="w-8 h-8 text-green-600" />
          </div>
        </div>

        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-600">Active Detectors</p>
              <p className="text-2xl font-bold text-gray-900 mt-1">3/4</p>
            </div>
            <Activity className="w-8 h-8 text-blue-600" />
          </div>
        </div>

        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-600">Threats Detected</p>
              <p className="text-2xl font-bold text-gray-900 mt-1">35</p>
            </div>
            <AlertTriangle className="w-8 h-8 text-orange-600" />
          </div>
        </div>

        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-600">System Health</p>
              <p className="text-2xl font-bold text-gray-900 mt-1">98%</p>
            </div>
            <CheckCircle className="w-8 h-8 text-green-600" />
          </div>
        </div>
      </div>

      {/* Transaction Chart */}
      <div className="bg-white rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold text-gray-900 mb-6">
          Transaction Volume & Alerts
        </h2>
        <ResponsiveContainer width="100%" height={300}>
          <AreaChart data={transactionData}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="time" />
            <YAxis />
            <Tooltip />
            <Legend />
            <Area
              type="monotone"
              dataKey="transactions"
              stroke="#3b82f6"
              fill="#3b82f6"
              fillOpacity={0.6}
            />
            <Area
              type="monotone"
              dataKey="alerts"
              stroke="#ef4444"
              fill="#ef4444"
              fillOpacity={0.6}
            />
          </AreaChart>
        </ResponsiveContainer>
      </div>

      {/* Detector Status */}
      <div className="bg-white rounded-lg shadow">
        <div className="p-6 border-b border-gray-200">
          <h2 className="text-xl font-semibold text-gray-900">
            Attack Pattern Detectors
          </h2>
        </div>
        <div className="p-6">
          <div className="space-y-4">
            {detectorStats.map((detector) => (
              <div
                key={detector.name}
                className="flex items-center justify-between p-4 border border-gray-200 rounded-lg"
              >
                <div className="flex items-center gap-4">
                  <div
                    className={`w-3 h-3 rounded-full ${
                      detector.active ? 'bg-green-500' : 'bg-gray-300'
                    }`}
                  />
                  <div>
                    <h3 className="font-medium text-gray-900">{detector.name}</h3>
                    <p className="text-sm text-gray-600">
                      Last alert: {detector.lastAlert}
                    </p>
                  </div>
                </div>
                <div className="text-right">
                  <p className="text-2xl font-bold text-gray-900">
                    {detector.detections}
                  </p>
                  <p className="text-sm text-gray-600">detections today</p>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  )
}
