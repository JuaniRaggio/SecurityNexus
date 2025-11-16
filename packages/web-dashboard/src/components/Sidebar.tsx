'use client'

import Link from 'next/link'
import { usePathname } from 'next/navigation'
import { LayoutDashboard } from 'lucide-react'
import { FileSearch } from 'lucide-react'
import { Activity } from 'lucide-react'
import { Shield } from 'lucide-react'
import { AlertTriangle } from 'lucide-react'
import { Settings } from 'lucide-react'
import { Database } from 'lucide-react'
import { GitBranch } from 'lucide-react'
import { Droplet } from 'lucide-react'
import { BarChart3 } from 'lucide-react'
import clsx from 'clsx'

const navigation = [
  { name: 'Dashboard', href: '/', icon: LayoutDashboard },
  { name: 'Static Analysis', href: '/analysis', icon: FileSearch },
  { name: 'Monitoring', href: '/monitoring', icon: Activity },
  { name: 'Alerts', href: '/alerts', icon: AlertTriangle },
  { name: 'Hyperbridge', href: '/hyperbridge', icon: GitBranch },
  { name: 'Hydration', href: '/hydration', icon: Droplet },
  { name: 'Analytics', href: '/analytics', icon: BarChart3 },
  { name: 'Privacy Layer', href: '/privacy', icon: Shield },
  { name: 'Data', href: '/data', icon: Database },
  { name: 'Settings', href: '/settings', icon: Settings },
]

export default function Sidebar() {
  const pathname = usePathname()

  return (
    <div className="w-64 bg-gray-900 text-white">
      <div className="flex items-center gap-2 px-6 py-5 border-b border-gray-800">
        <Shield className="w-8 h-8 text-blue-400" />
        <div>
          <h1 className="text-xl font-bold">Security Nexus</h1>
          <p className="text-xs text-gray-400">Polkadot Security</p>
        </div>
      </div>

      <nav className="mt-6 px-3">
        {navigation.map((item) => {
          const isActive = pathname === item.href
          const Icon = item.icon

          return (
            <Link
              key={item.name}
              href={item.href}
              className={clsx(
                'flex items-center gap-3 px-3 py-3 rounded-lg mb-1 transition-colors',
                isActive
                  ? 'bg-blue-600 text-white'
                  : 'text-gray-300 hover:bg-gray-800 hover:text-white'
              )}
            >
              <Icon className="w-5 h-5" />
              <span className="font-medium">{item.name}</span>
            </Link>
          )
        })}
      </nav>

      <div className="absolute bottom-0 w-64 p-4 border-t border-gray-800">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-full bg-gradient-to-r from-blue-500 to-purple-500" />
          <div className="flex-1">
            <p className="text-sm font-medium">Security Team</p>
            <p className="text-xs text-gray-400">admin@secnexus.io</p>
          </div>
        </div>
      </div>
    </div>
  )
}
