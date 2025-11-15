'use client'

import { Shield, Activity, AlertTriangle, FileSearch } from 'lucide-react'
import StatsCard from '@/components/StatsCard'
import AlertsPanel from '@/components/AlertsPanel'
import RecentAnalysis from '@/components/RecentAnalysis'
import ChainStatus from '@/components/ChainStatus'
import { useStats } from '@/hooks/useAnalysis'

export default function Home() {
  const { data: statsData, isLoading, error } = useStats();
  const stats = statsData?.stats;

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold text-gray-900">Security Dashboard</h1>
        <p className="text-gray-600 mt-2">
          Real-time security monitoring for Polkadot ecosystem
        </p>
      </div>

      {/* Stats Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <StatsCard
          title="Active Alerts"
          value={isLoading ? '-' : String(stats?.activeAlerts || 0)}
          change={stats?.activeAlerts === 0 ? 'All clear' : 'Attention required'}
          icon={AlertTriangle}
          trend={stats?.activeAlerts === 0 ? 'neutral' : 'up'}
          color="red"
        />
        <StatsCard
          title="Pallets Analyzed"
          value={isLoading ? '-' : String(stats?.totalPalletsAnalyzed || 0)}
          change={stats?.totalPalletsAnalyzed === 0 ? 'Upload a pallet to get started' : 'Total analyzed'}
          icon={FileSearch}
          trend="neutral"
          color="blue"
        />
        <StatsCard
          title="Chains Monitored"
          value={isLoading ? '-' : String(stats?.chainsMonitored || 0)}
          change="All operational"
          icon={Activity}
          trend="neutral"
          color="green"
        />
        <StatsCard
          title="Security Score"
          value={isLoading ? '-' : `${stats?.securityScore || 0}%`}
          change={(stats?.securityScore ?? 0) >= 90 ? 'Excellent' : (stats?.securityScore ?? 0) >= 70 ? 'Good' : 'Needs attention'}
          icon={Shield}
          trend={(stats?.securityScore ?? 0) >= 90 ? 'up' : (stats?.securityScore ?? 0) >= 70 ? 'neutral' : 'down'}
          color="green"
        />
      </div>

      {error && (
        <div className="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded">
          Failed to load dashboard stats. Make sure the SAFT binary is built and configured correctly.
        </div>
      )}

      {/* Main Content Grid */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="lg:col-span-2 space-y-6">
          <RecentAnalysis />
          <ChainStatus />
        </div>
        <div className="lg:col-span-1">
          <AlertsPanel />
        </div>
      </div>
    </div>
  )
}
