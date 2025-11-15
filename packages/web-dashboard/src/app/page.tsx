'use client'

import { Shield, Activity, AlertTriangle, FileSearch } from 'lucide-react'
import StatsCard from '@/components/StatsCard'
import AlertsPanel from '@/components/AlertsPanel'
import RecentAnalysis from '@/components/RecentAnalysis'
import ChainStatus from '@/components/ChainStatus'

export default function Home() {
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
          value="3"
          change="+2 from yesterday"
          icon={AlertTriangle}
          trend="up"
          color="red"
        />
        <StatsCard
          title="Pallets Analyzed"
          value="127"
          change="+12 this week"
          icon={FileSearch}
          trend="up"
          color="blue"
        />
        <StatsCard
          title="Chains Monitored"
          value="5"
          change="All operational"
          icon={Activity}
          trend="neutral"
          color="green"
        />
        <StatsCard
          title="Security Score"
          value="94%"
          change="+3% from last month"
          icon={Shield}
          trend="up"
          color="green"
        />
      </div>

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
