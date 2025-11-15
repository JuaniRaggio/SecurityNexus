'use client';

import { AlertTriangle, CheckCircle, XCircle, Info, FileSearch } from 'lucide-react';
import clsx from 'clsx';
import type { SAFTAnalysisResult } from '@/types/api';

interface AnalysisResultsProps {
  result: SAFTAnalysisResult;
  onClose?: () => void;
}

const severityConfig = {
  critical: {
    icon: XCircle,
    color: 'text-red-600',
    bg: 'bg-red-100',
    border: 'border-red-200',
    label: 'Critical',
  },
  high: {
    icon: AlertTriangle,
    color: 'text-orange-600',
    bg: 'bg-orange-100',
    border: 'border-orange-200',
    label: 'High',
  },
  medium: {
    icon: AlertTriangle,
    color: 'text-yellow-600',
    bg: 'bg-yellow-100',
    border: 'border-yellow-200',
    label: 'Medium',
  },
  low: {
    icon: Info,
    color: 'text-blue-600',
    bg: 'bg-blue-100',
    border: 'border-blue-200',
    label: 'Low',
  },
  info: {
    icon: Info,
    color: 'text-gray-600',
    bg: 'bg-gray-100',
    border: 'border-gray-200',
    label: 'Info',
  },
};

export default function AnalysisResults({ result, onClose }: AnalysisResultsProps) {
  const severityCounts = result.vulnerabilities.reduce(
    (acc, vuln) => {
      acc[vuln.severity] = (acc[vuln.severity] || 0) + 1;
      return acc;
    },
    {} as Record<string, number>
  );

  const hasVulnerabilities = result.vulnerabilities.length > 0;

  return (
    <div className="bg-white rounded-lg shadow">
      {/* Header */}
      <div className="p-6 border-b border-gray-200">
        <div className="flex items-start justify-between">
          <div className="flex items-center gap-3">
            <FileSearch className="w-8 h-8 text-blue-600" />
            <div>
              <h2 className="text-2xl font-bold text-gray-900">Analysis Results</h2>
              <p className="text-gray-600 mt-1">{result.file}</p>
            </div>
          </div>
          {onClose && (
            <button
              onClick={onClose}
              className="text-gray-400 hover:text-gray-600 transition-colors"
            >
              <XCircle className="w-6 h-6" />
            </button>
          )}
        </div>

        {/* Summary Stats */}
        <div className="mt-6 grid grid-cols-2 md:grid-cols-5 gap-4">
          <div className="text-center p-4 bg-gray-50 rounded-lg">
            <div className="text-2xl font-bold text-gray-900">
              {result.vulnerabilities.length}
            </div>
            <div className="text-sm text-gray-600 mt-1">Total Issues</div>
          </div>
          <div className="text-center p-4 bg-red-50 rounded-lg">
            <div className="text-2xl font-bold text-red-600">
              {severityCounts.critical || 0}
            </div>
            <div className="text-sm text-gray-600 mt-1">Critical</div>
          </div>
          <div className="text-center p-4 bg-orange-50 rounded-lg">
            <div className="text-2xl font-bold text-orange-600">
              {severityCounts.high || 0}
            </div>
            <div className="text-sm text-gray-600 mt-1">High</div>
          </div>
          <div className="text-center p-4 bg-yellow-50 rounded-lg">
            <div className="text-2xl font-bold text-yellow-600">
              {severityCounts.medium || 0}
            </div>
            <div className="text-sm text-gray-600 mt-1">Medium</div>
          </div>
          <div className="text-center p-4 bg-blue-50 rounded-lg">
            <div className="text-2xl font-bold text-blue-600">
              {severityCounts.low || 0}
            </div>
            <div className="text-sm text-gray-600 mt-1">Low</div>
          </div>
        </div>

        {!hasVulnerabilities && (
          <div className="mt-6 flex items-center gap-3 p-4 bg-green-50 border border-green-200 rounded-lg">
            <CheckCircle className="w-8 h-8 text-green-600" />
            <div>
              <h3 className="font-semibold text-green-900">All Clear!</h3>
              <p className="text-sm text-green-700 mt-1">
                No security vulnerabilities detected in this pallet.
              </p>
            </div>
          </div>
        )}
      </div>

      {/* Vulnerabilities List */}
      {hasVulnerabilities && (
        <div className="p-6">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">
            Detected Vulnerabilities
          </h3>
          <div className="space-y-4">
            {result.vulnerabilities.map((vuln, index) => {
              const config = severityConfig[vuln.severity];
              const Icon = config.icon;

              return (
                <div
                  key={index}
                  className={clsx(
                    'p-4 rounded-lg border-l-4',
                    config.bg,
                    config.border
                  )}
                >
                  <div className="flex items-start gap-3">
                    <Icon className={clsx('w-5 h-5 mt-0.5 flex-shrink-0', config.color)} />
                    <div className="flex-1 min-w-0">
                      <div className="flex items-start justify-between gap-4">
                        <h4 className="font-semibold text-gray-900">
                          {vuln.message}
                        </h4>
                        <span
                          className={clsx(
                            'text-xs font-medium px-2 py-1 rounded whitespace-nowrap',
                            config.bg,
                            config.color
                          )}
                        >
                          {config.label}
                        </span>
                      </div>
                      <p className="text-sm text-gray-700 mt-2">
                        {vuln.description}
                      </p>

                      {/* Location */}
                      {vuln.location && (
                        <div className="mt-3 p-3 bg-gray-50 rounded border border-gray-200">
                          <div className="flex items-center gap-2 text-xs text-gray-600">
                            <span className="font-medium">Location:</span>
                            <span>
                              {vuln.location.file}:{vuln.location.line}
                              {vuln.location.column && `:${vuln.location.column}`}
                            </span>
                          </div>
                        </div>
                      )}

                      {/* Remediation */}
                      {vuln.remediation && (
                        <div className="mt-3">
                          <h5 className="text-sm font-semibold text-gray-900">
                            Remediation:
                          </h5>
                          <p className="text-sm text-gray-700 mt-1">
                            {vuln.remediation}
                          </p>
                        </div>
                      )}

                      {/* References */}
                      {vuln.references && vuln.references.length > 0 && (
                        <div className="mt-3">
                          <h5 className="text-sm font-semibold text-gray-900">
                            References:
                          </h5>
                          <ul className="mt-1 space-y-1">
                            {vuln.references.map((ref, i) => (
                              <li key={i}>
                                <a
                                  href={ref}
                                  target="_blank"
                                  rel="noopener noreferrer"
                                  className="text-sm text-blue-600 hover:text-blue-700 underline"
                                >
                                  {ref}
                                </a>
                              </li>
                            ))}
                          </ul>
                        </div>
                      )}
                    </div>
                  </div>
                </div>
              );
            })}
          </div>
        </div>
      )}

      {/* Metadata */}
      <div className="p-6 border-t border-gray-200 bg-gray-50">
        <h3 className="text-sm font-semibold text-gray-900 mb-3">
          Analysis Metadata
        </h3>
        <div className="grid grid-cols-2 md:grid-cols-3 gap-4 text-sm">
          <div>
            <span className="text-gray-600">Analyzer:</span>
            <span className="ml-2 font-medium text-gray-900">
              SAFT Enhanced {result.metadata.analyzer_version}
            </span>
          </div>
          <div>
            <span className="text-gray-600">Total Issues:</span>
            <span className="ml-2 font-medium text-gray-900">
              {result.metadata.total_vulnerabilities}
            </span>
          </div>
          <div>
            <span className="text-gray-600">Duration:</span>
            <span className="ml-2 font-medium text-gray-900">
              {result.metadata.duration_ms}ms
            </span>
          </div>
        </div>
      </div>
    </div>
  );
}
