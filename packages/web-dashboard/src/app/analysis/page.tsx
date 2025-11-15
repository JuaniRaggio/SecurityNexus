'use client'

import { FileSearch, Upload, History } from 'lucide-react'
import { useState } from 'react'

export default function AnalysisPage() {
  const [dragActive, setDragActive] = useState(false)

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold text-gray-900">Static Analysis</h1>
        <p className="text-gray-600 mt-2">
          Upload and analyze FRAME pallets for security vulnerabilities
        </p>
      </div>

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
          }`}
          onDragEnter={() => setDragActive(true)}
          onDragLeave={() => setDragActive(false)}
          onDrop={() => setDragActive(false)}
        >
          <Upload className="w-16 h-16 text-gray-400 mx-auto mb-4" />
          <h3 className="text-lg font-medium text-gray-900 mb-2">
            Drop your pallet files here
          </h3>
          <p className="text-gray-600 mb-4">
            or click to browse for Rust files (.rs)
          </p>
          <button className="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors">
            Choose Files
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
              JSON, Text, SARIF formats
            </p>
          </div>
        </div>
      </div>

      {/* Analysis Queue */}
      <div className="bg-white rounded-lg shadow">
        <div className="p-6 border-b border-gray-200">
          <h2 className="text-xl font-semibold text-gray-900">
            Analysis History
          </h2>
        </div>
        <div className="p-6">
          <p className="text-gray-500 text-center py-8">
            No recent analyses. Upload a pallet to get started.
          </p>
        </div>
      </div>
    </div>
  )
}
