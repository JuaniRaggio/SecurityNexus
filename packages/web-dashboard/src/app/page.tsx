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
    <div className="space-y-8">
      {/* Hero Section with Gradient */}
      <div className="relative overflow-hidden rounded-2xl bg-gradient-to-br from-pink-500 via-purple-600 to-blue-600 p-8 shadow-2xl">
        <div className="absolute inset-0 bg-black/10"></div>
        <div className="relative z-10">
          <div className="flex items-center gap-3 mb-3">
            <div className="p-3 bg-white/20 backdrop-blur-sm rounded-xl">
              <Shield className="h-8 w-8 text-white" />
            </div>
            <h1 className="text-4xl font-bold text-white">Security Dashboard</h1>
          </div>
          <p className="text-white/90 text-lg max-w-2xl">
            Real-time security monitoring and static analysis for the Polkadot ecosystem
          </p>
          <div className="mt-6 flex gap-4">
            <div className="px-4 py-2 bg-white/20 backdrop-blur-sm rounded-lg border border-white/30">
              <p className="text-white/80 text-sm">Powered by SAFT Enhanced</p>
            </div>
            <div className="px-4 py-2 bg-white/20 backdrop-blur-sm rounded-lg border border-white/30">
              <p className="text-white/80 text-sm">ZK Privacy Layer</p>
            </div>
          </div>
        </div>
        {/* Decorative elements */}
        <div className="absolute -right-12 -top-12 h-64 w-64 rounded-full bg-white/10 blur-3xl"></div>
        <div className="absolute -left-12 -bottom-12 h-64 w-64 rounded-full bg-white/10 blur-3xl"></div>
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
