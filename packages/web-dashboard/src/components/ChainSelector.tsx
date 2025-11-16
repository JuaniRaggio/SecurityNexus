'use client'

import { useState } from 'react'
import { Network, ChevronDown, Check, Info, AlertTriangle, CheckCircle, X, RefreshCw } from 'lucide-react'
import { useAvailableChains, useCurrentChain, useSwitchChain, type ChainInfo } from '@/hooks/useMonitoring'

export default function ChainSelector() {
  const [isOpen, setIsOpen] = useState(false)
  const [showConfirmModal, setShowConfirmModal] = useState(false)
  const [selectedChain, setSelectedChain] = useState<ChainInfo | null>(null)
  const [switchResult, setSwitchResult] = useState<{ success: boolean; message: string } | null>(null)

  const { data: chainsData, isLoading: chainsLoading } = useAvailableChains()
  const { data: currentChain, isLoading: currentLoading } = useCurrentChain()
  const switchChainMutation = useSwitchChain()

  const chains = chainsData?.chains || []
  const isLoading = chainsLoading || currentLoading

  const handleChainSelect = (chain: ChainInfo) => {
    // Don't allow switching to the same chain
    if (currentChain?.name === chain.name) {
      setIsOpen(false)
      return
    }

    setSelectedChain(chain)
    setShowConfirmModal(true)
    setIsOpen(false)
  }

  const handleConfirmSwitch = async () => {
    if (!selectedChain) return

    try {
      const result = await switchChainMutation.mutateAsync({
        chain_name: selectedChain.name,
      })

      setSwitchResult({
        success: true,
        message: result.message,
      })
      setShowConfirmModal(false)
    } catch (error) {
      setSwitchResult({
        success: false,
        message: error instanceof Error ? error.message : 'Failed to switch chain',
      })
      setShowConfirmModal(false)
    }
  }

  const handleCancelSwitch = () => {
    setShowConfirmModal(false)
    setSelectedChain(null)
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
                Click on a chain to switch. The monitoring engine will need to be restarted.
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

      {/* Confirmation Modal */}
      {showConfirmModal && selectedChain && (
        <>
          <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center p-4">
            <div className="bg-white rounded-lg shadow-xl max-w-md w-full">
              <div className="p-6">
                <div className="flex items-start gap-4">
                  <div className="w-10 h-10 rounded-full bg-blue-100 flex items-center justify-center flex-shrink-0">
                    <RefreshCw className="w-5 h-5 text-blue-600" />
                  </div>
                  <div className="flex-1">
                    <h3 className="text-lg font-semibold text-gray-900 mb-2">
                      Switch to {selectedChain.display_name}?
                    </h3>
                    <p className="text-sm text-gray-600 mb-4">
                      This will save the configuration to switch from{' '}
                      <span className="font-medium">{currentChain?.display_name}</span> to{' '}
                      <span className="font-medium">{selectedChain.display_name}</span>.
                    </p>
                    <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-3 mb-4">
                      <div className="flex items-start gap-2">
                        <AlertTriangle className="w-4 h-4 text-yellow-600 mt-0.5 flex-shrink-0" />
                        <div>
                          <p className="text-sm font-medium text-yellow-900">Restart Required</p>
                          <p className="text-xs text-yellow-700 mt-1">
                            You will need to manually restart the monitoring engine for changes to take effect.
                          </p>
                        </div>
                      </div>
                    </div>
                    <div className="bg-gray-50 rounded-lg p-3 text-xs font-mono text-gray-600">
                      {selectedChain.endpoint}
                    </div>
                  </div>
                </div>
              </div>
              <div className="bg-gray-50 px-6 py-4 flex gap-3 justify-end rounded-b-lg">
                <button
                  onClick={handleCancelSwitch}
                  className="px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-200 rounded-lg transition-colors"
                  disabled={switchChainMutation.isPending}
                >
                  Cancel
                </button>
                <button
                  onClick={handleConfirmSwitch}
                  disabled={switchChainMutation.isPending}
                  className="px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
                >
                  {switchChainMutation.isPending ? (
                    <>
                      <RefreshCw className="w-4 h-4 animate-spin" />
                      Switching...
                    </>
                  ) : (
                    'Switch Chain'
                  )}
                </button>
              </div>
            </div>
          </div>
        </>
      )}

      {/* Result Modal */}
      {switchResult && (
        <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center p-4">
          <div className="bg-white rounded-lg shadow-xl max-w-md w-full">
            <div className="p-6">
              <div className="flex items-start gap-4">
                <div
                  className={`w-10 h-10 rounded-full flex items-center justify-center flex-shrink-0 ${
                    switchResult.success ? 'bg-green-100' : 'bg-red-100'
                  }`}
                >
                  {switchResult.success ? (
                    <CheckCircle className="w-5 h-5 text-green-600" />
                  ) : (
                    <AlertTriangle className="w-5 h-5 text-red-600" />
                  )}
                </div>
                <div className="flex-1">
                  <h3 className="text-lg font-semibold text-gray-900 mb-2">
                    {switchResult.success ? 'Configuration Saved' : 'Switch Failed'}
                  </h3>
                  <p className="text-sm text-gray-600 mb-4">{switchResult.message}</p>
                  {switchResult.success && (
                    <div className="bg-blue-50 border border-blue-200 rounded-lg p-3">
                      <p className="text-sm font-medium text-blue-900 mb-1">Next Steps:</p>
                      <ol className="text-xs text-blue-700 space-y-1 list-decimal list-inside">
                        <li>Stop the current monitoring engine (Ctrl+C)</li>
                        <li>Restart it using the same command</li>
                        <li>The new chain will be loaded automatically</li>
                      </ol>
                    </div>
                  )}
                </div>
                <button
                  onClick={() => setSwitchResult(null)}
                  className="text-gray-400 hover:text-gray-600 transition-colors"
                >
                  <X className="w-5 h-5" />
                </button>
              </div>
            </div>
            <div className="bg-gray-50 px-6 py-4 flex justify-end rounded-b-lg">
              <button
                onClick={() => setSwitchResult(null)}
                className="px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors"
              >
                Close
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}
