'use client'

import { LucideIcon, TrendingUp, TrendingDown, Minus } from 'lucide-react'
import clsx from 'clsx'

interface StatsCardProps {
  title: string
  value: string
  change: string
  icon: LucideIcon
  trend: 'up' | 'down' | 'neutral'
  color: 'blue' | 'green' | 'red' | 'yellow'
}

const colorClasses = {
  blue: 'bg-blue-100 text-blue-600',
  green: 'bg-green-100 text-green-600',
  red: 'bg-red-100 text-red-600',
  yellow: 'bg-yellow-100 text-yellow-600',
}

const trendIcons = {
  up: TrendingUp,
  down: TrendingDown,
  neutral: Minus,
}

const trendColors = {
  up: 'text-green-600',
  down: 'text-red-600',
  neutral: 'text-gray-600',
}

export default function StatsCard({
  title,
  value,
  change,
  icon: Icon,
  trend,
  color,
}: StatsCardProps) {
  const TrendIcon = trendIcons[trend]

  return (
    <div className="bg-white rounded-lg shadow p-6">
      <div className="flex items-center justify-between">
        <div className="flex-1">
          <p className="text-sm font-medium text-gray-600">{title}</p>
          <p className="text-3xl font-bold text-gray-900 mt-2">{value}</p>
          <div className="flex items-center gap-1 mt-2">
            <TrendIcon className={clsx('w-4 h-4', trendColors[trend])} />
            <span className="text-sm text-gray-600">{change}</span>
          </div>
        </div>
        <div className={clsx('p-3 rounded-lg', colorClasses[color])}>
          <Icon className="w-6 h-6" />
        </div>
      </div>
    </div>
  )
}
