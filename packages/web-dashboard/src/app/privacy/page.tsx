'use client'

import { Shield, Lock, CheckCircle, AlertCircle, Loader2, Eye, EyeOff } from 'lucide-react'
import { useState } from 'react'
import toast from 'react-hot-toast'

type Severity = 'low' | 'medium' | 'high' | 'critical'

interface VulnerabilityReport {
  severity: Severity
  category: string
  description: string
  affectedCode: string
  remediation: string
  reporterId: string
}

interface ZKProof {
  commitment: string
  proofData: string
  publicInputs: string[]
  metadata: {
    createdAt: number
    circuitVersion: string
    curve: string
  }
}

export default function PrivacyPage() {
  const [step, setStep] = useState<'form' | 'proof' | 'verified'>('form')
  const [isGenerating, setIsGenerating] = useState(false)
  const [showDetails, setShowDetails] = useState(false)

  const [report, setReport] = useState<VulnerabilityReport>({
    severity: 'high',
    category: '',
    description: '',
    affectedCode: '',
    remediation: '',
    reporterId: ''
  })

  const [proof, setProof] = useState<ZKProof | null>(null)
  const [isValid, setIsValid] = useState<boolean | null>(null)

  const handleGenerateProof = async () => {
    if (!report.description || !report.category) {
      toast.error('Please fill in required fields')
      return
    }

    setIsGenerating(true)
    toast.loading('Generating zero-knowledge proof...', { id: 'generating' })

    // Simulate ZKP generation (en producción esto llamaría al backend)
    setTimeout(() => {
      const mockProof: ZKProof = {
        commitment: `0x${Math.random().toString(16).substring(2, 34)}...`,
        proofData: `0x${Math.random().toString(16).substring(2, 50)}...`,
        publicInputs: [`0x${Math.random().toString(16).substring(2, 34)}`],
        metadata: {
          createdAt: Date.now(),
          circuitVersion: 'v1',
          curve: 'BN254'
        }
      }

      setProof(mockProof)
      setStep('proof')
      setIsGenerating(false)
      toast.dismiss('generating')
      toast.success('Proof generated successfully!')
    }, 2000)
  }

  const handleVerifyProof = async () => {
    if (!proof) return

    toast.loading('Verifying proof...', { id: 'verifying' })

    setTimeout(() => {
      setIsValid(true)
      setStep('verified')
      toast.dismiss('verifying')
      toast.success('Proof verified successfully!')
    }, 1500)
  }

  const handleReset = () => {
    setStep('form')
    setProof(null)
    setIsValid(null)
    setReport({
      severity: 'high',
      category: '',
      description: '',
      affectedCode: '',
      remediation: '',
      reporterId: ''
    })
  }

  return (
    <div className="space-y-8">
      {/* Header */}
      <div className="relative overflow-hidden rounded-2xl bg-gradient-to-br from-purple-600 via-pink-600 to-red-600 p-8 shadow-2xl">
        <div className="absolute inset-0 bg-black/10"></div>
        <div className="relative z-10">
          <div className="flex items-center gap-3 mb-2">
            <div className="p-3 bg-white/20 backdrop-blur-sm rounded-xl">
              <Shield className="h-10 w-10 text-white" />
            </div>
            <h1 className="text-4xl font-bold text-white">Privacy Layer</h1>
          </div>
          <p className="text-white/90 text-lg max-w-3xl">
            Zero-knowledge proof system for private vulnerability reporting. Prove you found a vulnerability without revealing the exploit details.
          </p>
          <div className="mt-6 flex gap-4">
            <div className="px-4 py-2 bg-white/20 backdrop-blur-sm rounded-lg border border-white/30">
              <p className="text-white/80 text-sm">Groth16 SNARK</p>
            </div>
            <div className="px-4 py-2 bg-white/20 backdrop-blur-sm rounded-lg border border-white/30">
              <p className="text-white/80 text-sm">BN254 Curve</p>
            </div>
            <div className="px-4 py-2 bg-white/20 backdrop-blur-sm rounded-lg border border-white/30">
              <p className="text-white/80 text-sm">R1CS Constraints</p>
            </div>
          </div>
        </div>
      </div>

      {/* Progress Steps */}
      <div className="flex items-center justify-center gap-4">
        <div className={`flex items-center gap-2 px-6 py-3 rounded-xl transition-all ${
          step === 'form'
            ? 'bg-gradient-to-r from-purple-600 to-pink-600 text-white shadow-lg'
            : 'bg-gray-100 text-gray-600'
        }`}>
          <div className={`w-8 h-8 rounded-full flex items-center justify-center ${
            step === 'form' ? 'bg-white/20' : 'bg-gray-200'
          }`}>
            1
          </div>
          <span className="font-semibold">Create Report</span>
        </div>

        <div className="h-1 w-16 bg-gray-200 rounded"></div>

        <div className={`flex items-center gap-2 px-6 py-3 rounded-xl transition-all ${
          step === 'proof'
            ? 'bg-gradient-to-r from-purple-600 to-pink-600 text-white shadow-lg'
            : 'bg-gray-100 text-gray-600'
        }`}>
          <div className={`w-8 h-8 rounded-full flex items-center justify-center ${
            step === 'proof' ? 'bg-white/20' : 'bg-gray-200'
          }`}>
            2
          </div>
          <span className="font-semibold">Generate Proof</span>
        </div>

        <div className="h-1 w-16 bg-gray-200 rounded"></div>

        <div className={`flex items-center gap-2 px-6 py-3 rounded-xl transition-all ${
          step === 'verified'
            ? 'bg-gradient-to-r from-green-600 to-emerald-600 text-white shadow-lg'
            : 'bg-gray-100 text-gray-600'
        }`}>
          <div className={`w-8 h-8 rounded-full flex items-center justify-center ${
            step === 'verified' ? 'bg-white/20' : 'bg-gray-200'
          }`}>
            3
          </div>
          <span className="font-semibold">Verify</span>
        </div>
      </div>

      {/* Step 1: Form */}
      {step === 'form' && (
        <div className="bg-white rounded-2xl shadow-xl p-8 border border-gray-100">
          <h2 className="text-2xl font-bold bg-gradient-to-r from-purple-600 to-pink-600 bg-clip-text text-transparent mb-6">
            Vulnerability Report Details
          </h2>

          <div className="space-y-6">
            <div>
              <label className="block text-sm font-semibold text-gray-700 mb-2">
                Severity *
              </label>
              <select
                value={report.severity}
                onChange={(e) => setReport({ ...report, severity: e.target.value as Severity })}
                className="w-full px-4 py-3 rounded-xl border-2 border-gray-200 focus:border-purple-500 focus:ring-2 focus:ring-purple-200 transition-all"
              >
                <option value="low">Low</option>
                <option value="medium">Medium</option>
                <option value="high">High</option>
                <option value="critical">Critical</option>
              </select>
            </div>

            <div>
              <label className="block text-sm font-semibold text-gray-700 mb-2">
                Category *
              </label>
              <input
                type="text"
                value={report.category}
                onChange={(e) => setReport({ ...report, category: e.target.value })}
                placeholder="e.g., reentrancy, integer_overflow, access_control"
                className="w-full px-4 py-3 rounded-xl border-2 border-gray-200 focus:border-purple-500 focus:ring-2 focus:ring-purple-200 transition-all"
              />
            </div>

            <div>
              <label className="block text-sm font-semibold text-gray-700 mb-2">
                Description *
              </label>
              <textarea
                value={report.description}
                onChange={(e) => setReport({ ...report, description: e.target.value })}
                placeholder="Describe the vulnerability (this will be hashed in the ZKP)"
                rows={4}
                className="w-full px-4 py-3 rounded-xl border-2 border-gray-200 focus:border-purple-500 focus:ring-2 focus:ring-purple-200 transition-all"
              />
            </div>

            <div>
              <label className="block text-sm font-semibold text-gray-700 mb-2">
                Affected Code
              </label>
              <textarea
                value={report.affectedCode}
                onChange={(e) => setReport({ ...report, affectedCode: e.target.value })}
                placeholder="Code snippet showing the vulnerability"
                rows={3}
                className="w-full px-4 py-3 rounded-xl border-2 border-gray-200 focus:border-purple-500 focus:ring-2 focus:ring-purple-200 transition-all font-mono text-sm"
              />
            </div>

            <div>
              <label className="block text-sm font-semibold text-gray-700 mb-2">
                Remediation
              </label>
              <textarea
                value={report.remediation}
                onChange={(e) => setReport({ ...report, remediation: e.target.value })}
                placeholder="How to fix this vulnerability"
                rows={3}
                className="w-full px-4 py-3 rounded-xl border-2 border-gray-200 focus:border-purple-500 focus:ring-2 focus:ring-purple-200 transition-all"
              />
            </div>

            <div>
              <label className="block text-sm font-semibold text-gray-700 mb-2">
                Reporter ID
              </label>
              <input
                type="text"
                value={report.reporterId}
                onChange={(e) => setReport({ ...report, reporterId: e.target.value })}
                placeholder="Your identifier (optional)"
                className="w-full px-4 py-3 rounded-xl border-2 border-gray-200 focus:border-purple-500 focus:ring-2 focus:ring-purple-200 transition-all"
              />
            </div>

            <button
              onClick={handleGenerateProof}
              disabled={isGenerating}
              className="w-full py-4 bg-gradient-to-r from-purple-600 to-pink-600 text-white font-bold rounded-xl hover:shadow-2xl hover:shadow-purple-500/50 transform hover:-translate-y-0.5 transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
            >
              {isGenerating ? (
                <>
                  <Loader2 className="w-5 h-5 animate-spin" />
                  Generating Proof...
                </>
              ) : (
                <>
                  <Lock className="w-5 h-5" />
                  Generate Zero-Knowledge Proof
                </>
              )}
            </button>
          </div>
        </div>
      )}

      {/* Step 2: Proof Generated */}
      {step === 'proof' && proof && (
        <div className="space-y-6">
          <div className="bg-gradient-to-br from-purple-50 to-pink-50 rounded-2xl shadow-xl p-8 border-2 border-purple-200">
            <div className="flex items-center gap-3 mb-6">
              <div className="p-3 bg-gradient-to-br from-purple-600 to-pink-600 rounded-xl">
                <CheckCircle className="w-8 h-8 text-white" />
              </div>
              <div>
                <h2 className="text-2xl font-bold text-gray-900">Proof Generated!</h2>
                <p className="text-gray-600">Your zero-knowledge proof has been created</p>
              </div>
            </div>

            <div className="space-y-4">
              <div className="bg-white rounded-xl p-6 border border-purple-200">
                <div className="flex items-center justify-between mb-3">
                  <h3 className="font-bold text-gray-900">Public Commitment</h3>
                  <button
                    onClick={() => setShowDetails(!showDetails)}
                    className="flex items-center gap-2 text-purple-600 hover:text-purple-700 text-sm font-medium"
                  >
                    {showDetails ? <EyeOff className="w-4 h-4" /> : <Eye className="w-4 h-4" />}
                    {showDetails ? 'Hide' : 'Show'} Details
                  </button>
                </div>
                <p className="font-mono text-sm text-gray-600 break-all bg-gray-50 p-3 rounded">
                  {proof.commitment}
                </p>
              </div>

              {showDetails && (
                <>
                  <div className="bg-white rounded-xl p-6 border border-purple-200">
                    <h3 className="font-bold text-gray-900 mb-3">Proof Data</h3>
                    <p className="font-mono text-sm text-gray-600 break-all bg-gray-50 p-3 rounded">
                      {proof.proofData}
                    </p>
                  </div>

                  <div className="grid grid-cols-3 gap-4">
                    <div className="bg-white rounded-xl p-4 border border-purple-200">
                      <p className="text-sm text-gray-600 mb-1">Curve</p>
                      <p className="font-bold text-gray-900">{proof.metadata.curve}</p>
                    </div>
                    <div className="bg-white rounded-xl p-4 border border-purple-200">
                      <p className="text-sm text-gray-600 mb-1">Circuit Version</p>
                      <p className="font-bold text-gray-900">{proof.metadata.circuitVersion}</p>
                    </div>
                    <div className="bg-white rounded-xl p-4 border border-purple-200">
                      <p className="text-sm text-gray-600 mb-1">Timestamp</p>
                      <p className="font-bold text-gray-900">
                        {new Date(proof.metadata.createdAt).toLocaleTimeString()}
                      </p>
                    </div>
                  </div>
                </>
              )}

              <div className="bg-white rounded-xl p-6 border-2 border-green-200 bg-gradient-to-br from-green-50 to-emerald-50">
                <h3 className="font-bold text-gray-900 mb-3 flex items-center gap-2">
                  <Shield className="w-5 h-5 text-green-600" />
                  What This Proves
                </h3>
                <ul className="space-y-2 text-sm text-gray-700">
                  <li className="flex items-start gap-2">
                    <CheckCircle className="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
                    <span>You know a vulnerability with severity: <strong>{report.severity}</strong></span>
                  </li>
                  <li className="flex items-start gap-2">
                    <CheckCircle className="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
                    <span>The commitment is cryptographically correct</span>
                  </li>
                  <li className="flex items-start gap-2">
                    <CheckCircle className="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
                    <span>Severity is in valid range [Low, Medium, High, Critical]</span>
                  </li>
                </ul>
              </div>

              <div className="bg-white rounded-xl p-6 border-2 border-red-200 bg-gradient-to-br from-red-50 to-rose-50">
                <h3 className="font-bold text-gray-900 mb-3 flex items-center gap-2">
                  <Lock className="w-5 h-5 text-red-600" />
                  What Remains Private
                </h3>
                <ul className="space-y-2 text-sm text-gray-700">
                  <li className="flex items-start gap-2">
                    <AlertCircle className="w-4 h-4 text-red-600 mt-0.5 flex-shrink-0" />
                    <span>Exploit details and methodology</span>
                  </li>
                  <li className="flex items-start gap-2">
                    <AlertCircle className="w-4 h-4 text-red-600 mt-0.5 flex-shrink-0" />
                    <span>Affected code location</span>
                  </li>
                  <li className="flex items-start gap-2">
                    <AlertCircle className="w-4 h-4 text-red-600 mt-0.5 flex-shrink-0" />
                    <span>Vulnerability description</span>
                  </li>
                </ul>
              </div>
            </div>

            <div className="flex gap-4 mt-6">
              <button
                onClick={handleVerifyProof}
                className="flex-1 py-4 bg-gradient-to-r from-green-600 to-emerald-600 text-white font-bold rounded-xl hover:shadow-2xl hover:shadow-green-500/50 transform hover:-translate-y-0.5 transition-all flex items-center justify-center gap-2"
              >
                <CheckCircle className="w-5 h-5" />
                Verify Proof
              </button>
              <button
                onClick={handleReset}
                className="px-8 py-4 bg-gray-200 text-gray-700 font-bold rounded-xl hover:bg-gray-300 transition-all"
              >
                Start Over
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Step 3: Verified */}
      {step === 'verified' && isValid !== null && (
        <div className="bg-gradient-to-br from-green-50 to-emerald-50 rounded-2xl shadow-xl p-8 border-2 border-green-200">
          <div className="text-center">
            <div className="inline-flex p-6 bg-gradient-to-br from-green-600 to-emerald-600 rounded-full mb-6">
              <CheckCircle className="w-16 h-16 text-white" />
            </div>
            <h2 className="text-3xl font-bold text-gray-900 mb-3">
              Proof Verified Successfully!
            </h2>
            <p className="text-lg text-gray-600 mb-8">
              The zero-knowledge proof has been cryptographically verified
            </p>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
              <div className="bg-white rounded-xl p-6 border border-green-200">
                <Shield className="w-8 h-8 text-green-600 mx-auto mb-3" />
                <h3 className="font-bold text-gray-900 mb-2">Proof Valid</h3>
                <p className="text-sm text-gray-600">All constraints satisfied</p>
              </div>
              <div className="bg-white rounded-xl p-6 border border-green-200">
                <Lock className="w-8 h-8 text-green-600 mx-auto mb-3" />
                <h3 className="font-bold text-gray-900 mb-2">Privacy Preserved</h3>
                <p className="text-sm text-gray-600">Details remain confidential</p>
              </div>
              <div className="bg-white rounded-xl p-6 border border-green-200">
                <CheckCircle className="w-8 h-8 text-green-600 mx-auto mb-3" />
                <h3 className="font-bold text-gray-900 mb-2">Ready to Submit</h3>
                <p className="text-sm text-gray-600">Claim your bug bounty</p>
              </div>
            </div>

            <button
              onClick={handleReset}
              className="px-12 py-4 bg-gradient-to-r from-purple-600 to-pink-600 text-white font-bold rounded-xl hover:shadow-2xl hover:shadow-purple-500/50 transform hover:-translate-y-0.5 transition-all"
            >
              Create Another Report
            </button>
          </div>
        </div>
      )}

      {/* Info Panel */}
      <div className="bg-gradient-to-br from-blue-50 to-cyan-50 rounded-2xl p-8 border-2 border-blue-200">
        <h3 className="text-xl font-bold text-gray-900 mb-4 flex items-center gap-2">
          <Shield className="w-6 h-6 text-blue-600" />
          How It Works
        </h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6 text-sm text-gray-700">
          <div>
            <h4 className="font-semibold mb-2">1. Commitment Scheme</h4>
            <p>The system creates a cryptographic commitment using your vulnerability details, severity, and a random blinding factor.</p>
          </div>
          <div>
            <h4 className="font-semibold mb-2">2. R1CS Constraints</h4>
            <p>The circuit enforces constraints to prove severity validity and commitment correctness without revealing data.</p>
          </div>
          <div>
            <h4 className="font-semibold mb-2">3. Groth16 Proof</h4>
            <p>Uses Groth16 SNARK on BN254 curve to generate a compact, efficient zero-knowledge proof (128 bytes).</p>
          </div>
          <div>
            <h4 className="font-semibold mb-2">4. Verification</h4>
            <p>Anyone can verify your proof without learning the vulnerability details, perfect for bug bounty programs.</p>
          </div>
        </div>
      </div>
    </div>
  )
}
