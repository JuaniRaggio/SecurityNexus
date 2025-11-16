import { NextResponse } from 'next/server'

// Monitoring Engine API endpoint
const MONITORING_ENGINE_URL = process.env.MONITORING_ENGINE_URL || 'http://localhost:8080'

export interface MonitoringStats {
  is_running: boolean
  blocks_processed: number
  transactions_analyzed: number
  alerts_triggered: number
  chain_name: string
  endpoint: string
  reconnect_attempts: number
}

export interface HealthStatus {
  status: string
  version: string
  uptime_seconds: number
}

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url)
  const endpoint = searchParams.get('endpoint') || 'stats'
  const useDemoAlerts = searchParams.get('demo') === 'true' || process.env.NEXT_PUBLIC_DEMO_MODE === 'true'

  try {
    const response = await fetch(`${MONITORING_ENGINE_URL}/api/${endpoint}`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
      // Don't cache monitoring data - we want real-time updates
      cache: 'no-store',
    })

    if (!response.ok) {
      return NextResponse.json(
        { error: `Monitoring engine returned ${response.status}` },
        { status: response.status }
      )
    }

    let data = await response.json()

    // In development mode, merge demo alerts with real alerts
    if (process.env.NODE_ENV === 'development' && useDemoAlerts &&
        (endpoint === 'alerts' || endpoint === 'alerts/unacknowledged')) {
      try {
        const demoResponse = await fetch('http://localhost:3000/api/demo-alerts')
        if (demoResponse.ok) {
          const demoAlerts = await demoResponse.json()
          // Merge demo alerts with real alerts
          data = Array.isArray(data) ? [...demoAlerts, ...data] : demoAlerts
        }
      } catch (demoError) {
        console.error('Error fetching demo alerts:', demoError)
      }
    }

    return NextResponse.json(data, {
      headers: {
        'Cache-Control': 'no-store, max-age=0',
      },
    })
  } catch (error) {
    console.error('Error fetching monitoring data:', error)

    // Return mock data if monitoring engine is not available
    if (endpoint === 'stats') {
      return NextResponse.json({
        is_running: false,
        blocks_processed: 0,
        transactions_analyzed: 0,
        alerts_triggered: 0,
        chain_name: 'Not connected',
        endpoint: 'Not connected',
        reconnect_attempts: 0,
        error: 'Monitoring engine not available',
      })
    }

    if (endpoint === 'detectors') {
      return NextResponse.json({
        detectors: [
          { name: 'Flash Loan Detector', enabled: true, detections: 0, last_detection: null },
          { name: 'MEV Detector', enabled: true, detections: 0, last_detection: null },
          { name: 'Volume Anomaly Detector', enabled: true, detections: 0, last_detection: null },
          { name: 'FrontRunning Detector', enabled: true, detections: 0, last_detection: null },
        ],
      })
    }

    // In development, return demo alerts if monitoring engine is down
    if (process.env.NODE_ENV === 'development' &&
        (endpoint === 'alerts' || endpoint === 'alerts/unacknowledged')) {
      try {
        const demoResponse = await fetch('http://localhost:3000/api/demo-alerts')
        if (demoResponse.ok) {
          const demoAlerts = await demoResponse.json()
          return NextResponse.json(demoAlerts)
        }
      } catch (demoError) {
        console.error('Error fetching demo alerts:', demoError)
      }
    }

    return NextResponse.json(
      {
        error: error instanceof Error ? error.message : 'Failed to fetch monitoring data',
        details: 'Make sure the monitoring engine is running'
      },
      { status: 503 }
    )
  }
}

export async function POST(request: Request) {
  const { searchParams } = new URL(request.url)
  const endpoint = searchParams.get('endpoint') || ''

  try {
    const response = await fetch(`${MONITORING_ENGINE_URL}/api/${endpoint}`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (!response.ok) {
      return NextResponse.json(
        { error: `Monitoring engine returned ${response.status}` },
        { status: response.status }
      )
    }

    const data = await response.json()

    return NextResponse.json(data)
  } catch (error) {
    console.error('Error posting to monitoring engine:', error)

    return NextResponse.json(
      {
        error: error instanceof Error ? error.message : 'Failed to post to monitoring engine',
        details: 'Make sure the monitoring engine is running'
      },
      { status: 503 }
    )
  }
}
