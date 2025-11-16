'use client'

import { FileSearch, Upload, History, Loader2 } from 'lucide-react'
import { useState, useRef, DragEvent, ChangeEvent } from 'react'
import { useAnalyzePallet, useHistory } from '@/hooks/useAnalysis'
import AnalysisResults from '@/components/AnalysisResults'
import toast from 'react-hot-toast'
import type { SAFTAnalysisResult } from '@/types/api'

export default function AnalysisPage() {
  const [dragActive, setDragActive] = useState(false)
  const [currentResult, setCurrentResult] = useState<SAFTAnalysisResult | null>(null)
  const fileInputRef = useRef<HTMLInputElement>(null)

  const { mutate: analyzePallet, isPending } = useAnalyzePallet()
  const { data: historyData } = useHistory(5)

  const handleFile = (file: File) => {
    if (!file.name.endsWith('.rs')) {
      toast.error('Only Rust (.rs) files are supported')
      return
    }

    if (file.size > 1024 * 1024) {
      toast.error('File too large (max 1MB)')
      return
    }

    toast.loading('Analyzing pallet...', { id: 'analyzing' })

    analyzePallet(file, {
      onSuccess: (data) => {
        toast.dismiss('analyzing')
        if (data.success && data.result) {
          setCurrentResult(data.result)
          toast.success('Analysis completed successfully!')
        } else {
          toast.error(data.error || 'Analysis failed')
        }
      },
      onError: (error) => {
        toast.dismiss('analyzing')
        toast.error(error instanceof Error ? error.message : 'Analysis failed')
      },
    })
  }

  const handleDrop = (e: DragEvent<HTMLDivElement>) => {
    e.preventDefault()
    setDragActive(false)

    const files = e.dataTransfer.files
    if (files.length > 0) {
      handleFile(files[0])
    }
  }

  const handleChange = (e: ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files
    if (files && files.length > 0) {
      handleFile(files[0])
    }
  }

  const handleDragOver = (e: DragEvent<HTMLDivElement>) => {
    e.preventDefault()
  }

  return (
    <div className="space-y-8">
      {/* Header with gradient */}
      <div className="relative overflow-hidden rounded-2xl bg-gradient-to-r from-blue-600 via-purple-600 to-pink-600 p-8 shadow-xl">
        <div className="absolute inset-0 bg-black/10"></div>
        <div className="relative z-10">
          <div className="flex items-center gap-3 mb-2">
            <FileSearch className="h-10 w-10 text-white" />
            <h1 className="text-4xl font-bold text-white">Static Analysis</h1>
          </div>
          <p className="text-white/90 text-lg">
            Upload and analyze FRAME pallets for security vulnerabilities using SAFT Enhanced
          </p>
        </div>
      </div>

      {/* Results Display */}
      {currentResult && (
        <AnalysisResults
          result={currentResult}
          onClose={() => setCurrentResult(null)}
        />
      )}

      {/* Upload Area */}
      <div className="bg-white rounded-2xl shadow-xl p-8 border border-gray-100">
        <h2 className="text-2xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent mb-6">
          Upload Pallet for Analysis
        </h2>

        <div
          className={`relative group border-2 border-dashed rounded-2xl p-16 text-center transition-all duration-300 ${
            dragActive
              ? 'border-blue-500 bg-gradient-to-br from-blue-50 to-purple-50 scale-[1.02]'
              : 'border-gray-300 hover:border-blue-400 hover:bg-gray-50'
          } ${isPending ? 'opacity-50 pointer-events-none' : ''}`}
          onDragEnter={() => setDragActive(true)}
          onDragLeave={() => setDragActive(false)}
          onDragOver={handleDragOver}
          onDrop={handleDrop}
        >
          {/* Background gradient effect */}
          <div className="absolute inset-0 bg-gradient-to-br from-blue-500/5 to-purple-500/5 rounded-2xl opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>

          <div className="relative z-10">
            {isPending ? (
              <div className="space-y-4">
                <div className="relative inline-block">
                  <Loader2 className="w-20 h-20 text-blue-600 animate-spin" />
                  <div className="absolute inset-0 bg-blue-500/20 rounded-full blur-xl animate-pulse"></div>
                </div>
                <h3 className="text-2xl font-bold text-gray-900">
                  Analyzing pallet...
                </h3>
                <p className="text-gray-600 text-lg">
                  Running comprehensive security scan
                </p>
              </div>
            ) : (
              <div className="space-y-6">
                <div className="relative inline-block">
                  <Upload className="w-20 h-20 text-blue-500 transform transition-transform duration-300 group-hover:scale-110" />
                  <div className="absolute inset-0 bg-blue-500/20 rounded-full blur-2xl opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                </div>
                <div>
                  <h3 className="text-2xl font-bold text-gray-900 mb-2">
                    Drop your pallet files here
                  </h3>
                  <p className="text-gray-600 text-lg">
                    or click to browse for Rust files (.rs)
                  </p>
                </div>
                <input
                  ref={fileInputRef}
                  type="file"
                  accept=".rs"
                  onChange={handleChange}
                  className="hidden"
                  disabled={isPending}
                />
                <button
                  onClick={() => fileInputRef.current?.click()}
                  disabled={isPending}
                  className="group/btn inline-flex items-center gap-2 px-8 py-4 bg-gradient-to-r from-blue-600 to-purple-600 text-white text-lg font-semibold rounded-xl hover:shadow-xl hover:shadow-blue-500/50 transform hover:-translate-y-0.5 transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  <FileSearch className="w-5 h-5 group-hover/btn:rotate-12 transition-transform" />
                  Choose File
                </button>
              </div>
            )}
          </div>
        </div>

        <div className="mt-8 grid grid-cols-1 md:grid-cols-3 gap-6">
          <div className="group text-center p-6 bg-gradient-to-br from-blue-50 to-blue-100/50 rounded-xl border border-blue-200 hover:shadow-lg transition-all duration-300 hover:-translate-y-1">
            <div className="inline-flex p-3 bg-gradient-to-br from-blue-500 to-blue-600 rounded-xl mb-4 group-hover:scale-110 transition-transform duration-300">
              <FileSearch className="w-8 h-8 text-white" />
            </div>
            <h4 className="font-bold text-gray-900 text-lg mb-2">Comprehensive Scan</h4>
            <p className="text-sm text-gray-600">
              Detects all vulnerability types including integer overflows, reentrancy, and XCM issues
            </p>
          </div>
          <div className="group text-center p-6 bg-gradient-to-br from-green-50 to-emerald-100/50 rounded-xl border border-green-200 hover:shadow-lg transition-all duration-300 hover:-translate-y-1">
            <div className="inline-flex p-3 bg-gradient-to-br from-green-500 to-emerald-600 rounded-xl mb-4 group-hover:scale-110 transition-transform duration-300">
              <History className="w-8 h-8 text-white" />
            </div>
            <h4 className="font-bold text-gray-900 text-lg mb-2">Quick Analysis</h4>
            <p className="text-sm text-gray-600">
              Lightning-fast results in seconds using advanced AST parsing
            </p>
          </div>
          <div className="group text-center p-6 bg-gradient-to-br from-purple-50 to-purple-100/50 rounded-xl border border-purple-200 hover:shadow-lg transition-all duration-300 hover:-translate-y-1">
            <div className="inline-flex p-3 bg-gradient-to-br from-purple-500 to-purple-600 rounded-xl mb-4 group-hover:scale-110 transition-transform duration-300">
              <FileSearch className="w-8 h-8 text-white" />
            </div>
            <h4 className="font-bold text-gray-900 text-lg mb-2">Detailed Reports</h4>
            <p className="text-sm text-gray-600">
              Actionable remediation steps with code examples
            </p>
          </div>
        </div>
      </div>

      {/* Analysis History */}
      <div className="bg-white rounded-2xl shadow-xl border border-gray-100">
        <div className="p-6 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
          <div className="flex items-center gap-3">
            <History className="w-6 h-6 text-blue-600" />
            <h2 className="text-2xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
              Recent Analyses
            </h2>
          </div>
        </div>
        <div className="p-6">
          {!historyData || historyData.history.length === 0 ? (
            <p className="text-gray-500 text-center py-8">
              No recent analyses. Upload a pallet to get started.
            </p>
          ) : (
            <div className="space-y-3">
              {historyData.history.map((item) => {
                const criticalCount = item.result.vulnerabilities.filter(
                  (v) => v.severity === 'critical'
                ).length
                const highCount = item.result.vulnerabilities.filter(
                  (v) => v.severity === 'high'
                ).length

                return (
                  <button
                    key={item.id}
                    onClick={() => setCurrentResult(item.result)}
                    className="w-full text-left p-5 bg-gradient-to-br from-gray-50 to-white hover:from-blue-50 hover:to-purple-50 rounded-xl transition-all duration-300 border border-gray-200 hover:border-blue-300 hover:shadow-lg group"
                  >
                    <div className="flex items-center justify-between">
                      <div className="flex items-center gap-3">
                        <FileSearch className="w-5 h-5 text-gray-400" />
                        <div>
                          <h4 className="font-medium text-gray-900">
                            {item.filename}
                          </h4>
                          <p className="text-sm text-gray-600">
                            {new Date(item.uploadedAt).toLocaleString()}
                          </p>
                        </div>
                      </div>
                      <div className="flex items-center gap-3">
                        {item.result.vulnerabilities.length === 0 ? (
                          <span className="text-sm text-green-600 font-medium">
                            No issues found
                          </span>
                        ) : (
                          <div className="flex items-center gap-2 text-sm">
                            {criticalCount > 0 && (
                              <span className="text-red-600 font-medium">
                                {criticalCount} Critical
                              </span>
                            )}
                            {highCount > 0 && (
                              <span className="text-orange-600 font-medium">
                                {highCount} High
                              </span>
                            )}
                          </div>
                        )}
                      </div>
                    </div>
                  </button>
                )
              })}
            </div>
          )}
        </div>
      </div>
    </div>
  )
}
