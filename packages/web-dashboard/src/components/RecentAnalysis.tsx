'use client'

import { FileSearch, CheckCircle, XCircle, AlertCircle } from 'lucide-react'
import clsx from 'clsx'
import { useHistory } from '@/hooks/useAnalysis'
import { formatDistanceToNow } from 'date-fns'

type AnalysisStatus = 'passed' | 'failed' | 'warning'

interface AnalysisDisplay {
  id: string
  pallet: string
  status: AnalysisStatus
  vulnerabilities: number
  critical: number
  high: number
  medium: number
  timestamp: string
}

const statusConfig = {
  passed: {
    icon: CheckCircle,
    color: 'text-green-600',
    bg: 'bg-green-100',
    label: 'Passed',
  },
  failed: {
    icon: XCircle,
    color: 'text-red-600',
    bg: 'bg-red-100',
    label: 'Failed',
  },
  warning: {
    icon: AlertCircle,
    color: 'text-yellow-600',
    bg: 'bg-yellow-100',
    label: 'Warning',
  },
}

export default function RecentAnalysis() {
  const { data: historyData, isLoading } = useHistory(10);

  const analyses: AnalysisDisplay[] = (historyData?.history || []).map((item) => {
    const severityCounts = item.result.vulnerabilities.reduce(
      (acc, vuln) => {
        acc[vuln.severity] = (acc[vuln.severity] || 0) + 1;
        return acc;
      },
      {} as Record<string, number>
    );

    const critical = severityCounts.critical || 0;
    const high = severityCounts.high || 0;
    const medium = severityCounts.medium || 0;
    const total = item.result.vulnerabilities.length;

    let status: AnalysisStatus = 'passed';
    if (critical > 0 || high > 0) {
      status = 'failed';
    } else if (medium > 0) {
      status = 'warning';
    }

    return {
      id: item.id,
      pallet: item.filename,
      status,
      vulnerabilities: total,
      critical,
      high,
      medium,
      timestamp: formatDistanceToNow(new Date(item.uploadedAt), { addSuffix: true }),
    };
  });

  return (
    <div className="bg-white rounded-lg shadow">
      <div className="p-6 border-b border-gray-200">
        <h2 className="text-xl font-bold text-gray-900">Recent Analysis</h2>
        <p className="text-sm text-gray-600 mt-1">SAFT Enhanced pallet scans</p>
      </div>

      <div className="overflow-x-auto">
        <table className="w-full">
          <thead className="bg-gray-50 border-b border-gray-200">
            <tr>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Pallet
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Status
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Vulnerabilities
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Time
              </th>
              <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                Actions
              </th>
            </tr>
          </thead>
          <tbody className="bg-white divide-y divide-gray-200">
            {isLoading ? (
              <tr>
                <td colSpan={5} className="px-6 py-8 text-center text-gray-500">
                  Loading analysis history...
                </td>
              </tr>
            ) : analyses.length === 0 ? (
              <tr>
                <td colSpan={5} className="px-6 py-8 text-center text-gray-500">
                  No analyses yet. Upload a pallet file to get started.
                </td>
              </tr>
            ) : (
              analyses.map((analysis) => {
                const config = statusConfig[analysis.status]
                const Icon = config.icon

                return (
                  <tr key={analysis.id} className="hover:bg-gray-50">
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="flex items-center gap-3">
                        <FileSearch className="w-5 h-5 text-gray-400" />
                        <span className="font-medium text-gray-900">
                          {analysis.pallet}
                        </span>
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span
                        className={clsx(
                          'inline-flex items-center gap-1 px-3 py-1 rounded-full text-sm font-medium',
                          config.bg,
                          config.color
                        )}
                      >
                        <Icon className="w-4 h-4" />
                        {config.label}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="flex items-center gap-4 text-sm">
                        {analysis.critical > 0 && (
                          <span className="text-red-600 font-medium">
                            {analysis.critical} Critical
                          </span>
                        )}
                        {analysis.high > 0 && (
                          <span className="text-orange-600 font-medium">
                            {analysis.high} High
                          </span>
                        )}
                        {analysis.medium > 0 && (
                          <span className="text-yellow-600 font-medium">
                            {analysis.medium} Medium
                          </span>
                        )}
                        {analysis.vulnerabilities === 0 && (
                          <span className="text-gray-500">None</span>
                        )}
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                      {analysis.timestamp}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-right text-sm">
                      <button className="text-blue-600 hover:text-blue-700 font-medium">
                        View Report
                      </button>
                    </td>
                  </tr>
                )
              })
            )}
          </tbody>
        </table>
      </div>
    </div>
  )
}
