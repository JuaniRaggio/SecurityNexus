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
  blue: {
    bg: 'bg-gradient-to-br from-blue-500 to-blue-600',
    text: 'text-blue-600',
    border: 'border-blue-200',
    glow: 'shadow-blue-500/20',
  },
  green: {
    bg: 'bg-gradient-to-br from-green-500 to-emerald-600',
    text: 'text-green-600',
    border: 'border-green-200',
    glow: 'shadow-green-500/20',
  },
  red: {
    bg: 'bg-gradient-to-br from-red-500 to-rose-600',
    text: 'text-red-600',
    border: 'border-red-200',
    glow: 'shadow-red-500/20',
  },
  yellow: {
    bg: 'bg-gradient-to-br from-yellow-500 to-orange-600',
    text: 'text-yellow-600',
    border: 'border-yellow-200',
    glow: 'shadow-yellow-500/20',
  },
}

const trendIcons = {
  up: TrendingUp,
  down: TrendingDown,
  neutral: Minus,
}

const trendColors = {
  up: 'text-green-600 bg-green-50',
  down: 'text-red-600 bg-red-50',
  neutral: 'text-gray-600 bg-gray-50',
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
  const colors = colorClasses[color]

  return (
    <div className={clsx(
      'group relative bg-white rounded-xl shadow-lg hover:shadow-2xl',
      'transition-all duration-300 ease-out p-6 border border-gray-100',
      'hover:-translate-y-1 overflow-hidden',
      colors.glow
    )}>
      {/* Animated gradient background on hover */}
      <div className="absolute inset-0 bg-gradient-to-br from-gray-50 to-white opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>

      <div className="relative z-10">
        <div className="flex items-start justify-between">
          <div className="flex-1">
            <p className="text-sm font-semibold text-gray-500 uppercase tracking-wide">{title}</p>
            <p className={clsx(
              'text-4xl font-bold mt-3 bg-gradient-to-r bg-clip-text text-transparent',
              'transition-all duration-300',
              color === 'blue' && 'from-blue-600 to-cyan-600',
              color === 'green' && 'from-green-600 to-emerald-600',
              color === 'red' && 'from-red-600 to-rose-600',
              color === 'yellow' && 'from-yellow-600 to-orange-600'
            )}>
              {value}
            </p>
            <div className={clsx(
              'inline-flex items-center gap-1.5 mt-3 px-2.5 py-1 rounded-full',
              'text-xs font-medium',
              trendColors[trend]
            )}>
              <TrendIcon className="w-3.5 h-3.5" />
              <span>{change}</span>
            </div>
          </div>

          {/* Icon with gradient background */}
          <div className={clsx(
            'p-4 rounded-xl shadow-lg',
            colors.bg,
            'transform transition-transform duration-300 group-hover:scale-110 group-hover:rotate-3'
          )}>
            <Icon className="w-7 h-7 text-white" />
          </div>
        </div>
      </div>

      {/* Decorative corner element */}
      <div className={clsx(
        'absolute -right-8 -bottom-8 w-32 h-32 rounded-full opacity-10',
        colors.bg,
        'transition-transform duration-500 group-hover:scale-150'
      )}></div>
    </div>
  )
}
