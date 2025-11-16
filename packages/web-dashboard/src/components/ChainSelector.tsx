'use client'

import { useState } from 'react'
import { Network, ChevronDown, Check, Info } from 'lucide-react'
import { useAvailableChains, useCurrentChain, type ChainInfo } from '@/hooks/useMonitoring'

export default function ChainSelector() {
  const [isOpen, setIsOpen] = useState(false)
  const { data: chainsData, isLoading: chainsLoading } = useAvailableChains()
  const { data: currentChain, isLoading: currentLoading } = useCurrentChain()

  const chains = chainsData?.chains || []
  const isLoading = chainsLoading || currentLoading

  const handleChainSelect = (chain: ChainInfo) => {
    // For now, just close the dropdown
    // Chain switching logic will be implemented in the next task
    setIsOpen(false)
    console.log('Selected chain:', chain.name)
  }

  return (
    <div className="relative">
      <button
        onClick={() => setIsOpen(!isOpen)}
        disabled={isLoading}
        className="flex items-center gap-2 px-4 py-2 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
      >
        <Network className="w-4 h-4 text-gray-600" />
        <div className="text-left">
          <p className="text-sm font-medium text-gray-900">
            {isLoading ? 'Loading...' : currentChain?.display_name || 'Unknown Chain'}
          </p>
          <p className="text-xs text-gray-500">
            {isLoading ? '' : currentChain?.name || ''}
          </p>
        </div>
        <ChevronDown className={`w-4 h-4 text-gray-600 transition-transform ${isOpen ? 'rotate-180' : ''}`} />
      </button>

      {isOpen && !isLoading && (
        <div className="absolute top-full mt-2 w-96 bg-white border border-gray-200 rounded-lg shadow-lg z-50">
          <div className="p-3 border-b border-gray-200">
            <h3 className="text-sm font-semibold text-gray-900">Select Chain</h3>
            <p className="text-xs text-gray-600 mt-1">
              Choose which blockchain network to monitor
            </p>
          </div>
          <div className="max-h-96 overflow-y-auto">
            {chains.map((chain) => {
              const isActive = currentChain?.name === chain.name
              return (
                <button
                  key={chain.name}
                  onClick={() => handleChainSelect(chain)}
                  className={`w-full px-4 py-3 text-left hover:bg-gray-50 transition-colors border-b border-gray-100 last:border-b-0 ${
                    isActive ? 'bg-blue-50' : ''
                  }`}
                >
                  <div className="flex items-start justify-between gap-3">
                    <div className="flex-1">
                      <div className="flex items-center gap-2">
                        <h4 className={`text-sm font-medium ${isActive ? 'text-blue-900' : 'text-gray-900'}`}>
                          {chain.display_name}
                        </h4>
                        {isActive && <Check className="w-4 h-4 text-blue-600" />}
                      </div>
                      <p className="text-xs text-gray-600 mt-1">{chain.description}</p>
                      <p className="text-xs font-mono text-gray-500 mt-1.5 truncate">
                        {chain.endpoint}
                      </p>
                    </div>
                  </div>
                </button>
              )
            })}
          </div>
          <div className="p-3 border-t border-gray-200 bg-gray-50">
            <div className="flex items-start gap-2">
              <Info className="w-4 h-4 text-blue-600 mt-0.5 flex-shrink-0" />
              <p className="text-xs text-gray-700">
                Switching chains requires restarting the monitoring engine. This feature will be available soon.
              </p>
            </div>
          </div>
        </div>
      )}

      {isOpen && (
        <div
          className="fixed inset-0 z-40"
          onClick={() => setIsOpen(false)}
        />
      )}
    </div>
  )
}
