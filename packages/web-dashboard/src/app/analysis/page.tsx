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
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold text-gray-900">Static Analysis</h1>
        <p className="text-gray-600 mt-2">
          Upload and analyze FRAME pallets for security vulnerabilities
        </p>
      </div>

      {/* Results Display */}
      {currentResult && (
        <AnalysisResults
          result={currentResult}
          onClose={() => setCurrentResult(null)}
        />
      )}

      {/* Upload Area */}
      <div className="bg-white rounded-lg shadow p-8">
        <h2 className="text-xl font-semibold text-gray-900 mb-4">
          Upload Pallet for Analysis
        </h2>

        <div
          className={`border-2 border-dashed rounded-lg p-12 text-center transition-colors ${
            dragActive
              ? 'border-blue-500 bg-blue-50'
              : 'border-gray-300 hover:border-gray-400'
          } ${isPending ? 'opacity-50 pointer-events-none' : ''}`}
          onDragEnter={() => setDragActive(true)}
          onDragLeave={() => setDragActive(false)}
          onDragOver={handleDragOver}
          onDrop={handleDrop}
        >
          {isPending ? (
            <Loader2 className="w-16 h-16 text-blue-600 mx-auto mb-4 animate-spin" />
          ) : (
            <Upload className="w-16 h-16 text-gray-400 mx-auto mb-4" />
          )}
          <h3 className="text-lg font-medium text-gray-900 mb-2">
            {isPending ? 'Analyzing pallet...' : 'Drop your pallet files here'}
          </h3>
          <p className="text-gray-600 mb-4">
            {isPending ? 'This may take a few seconds' : 'or click to browse for Rust files (.rs)'}
          </p>
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
            className="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isPending ? 'Analyzing...' : 'Choose File'}
          </button>
        </div>

        <div className="mt-6 grid grid-cols-1 md:grid-cols-3 gap-4">
          <div className="text-center p-4 bg-gray-50 rounded-lg">
            <FileSearch className="w-8 h-8 text-blue-600 mx-auto mb-2" />
            <h4 className="font-medium text-gray-900">Comprehensive Scan</h4>
            <p className="text-sm text-gray-600 mt-1">
              All vulnerability types
            </p>
          </div>
          <div className="text-center p-4 bg-gray-50 rounded-lg">
            <History className="w-8 h-8 text-green-600 mx-auto mb-2" />
            <h4 className="font-medium text-gray-900">Quick Analysis</h4>
            <p className="text-sm text-gray-600 mt-1">Results in seconds</p>
          </div>
          <div className="text-center p-4 bg-gray-50 rounded-lg">
            <FileSearch className="w-8 h-8 text-purple-600 mx-auto mb-2" />
            <h4 className="font-medium text-gray-900">Detailed Reports</h4>
            <p className="text-sm text-gray-600 mt-1">
              Actionable remediation
            </p>
          </div>
        </div>
      </div>

      {/* Analysis History */}
      <div className="bg-white rounded-lg shadow">
        <div className="p-6 border-b border-gray-200">
          <h2 className="text-xl font-semibold text-gray-900">
            Recent Analyses
          </h2>
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
                    className="w-full text-left p-4 bg-gray-50 hover:bg-gray-100 rounded-lg transition-colors border border-gray-200"
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
