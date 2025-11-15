import { NextResponse } from 'next/server'

const MONITORING_ENGINE_URL = process.env.MONITORING_ENGINE_URL || 'http://localhost:8080'

export async function POST(
  request: Request,
  { params }: { params: { id: string } }
) {
  try {
    const response = await fetch(
      `${MONITORING_ENGINE_URL}/api/alerts/${params.id}/acknowledge`,
      {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
      }
    )

    if (!response.ok) {
      return NextResponse.json(
        { error: `Monitoring engine returned ${response.status}` },
        { status: response.status }
      )
    }

    const data = await response.json()
    return NextResponse.json(data)
  } catch (error) {
    console.error('Error acknowledging alert:', error)

    return NextResponse.json(
      {
        error: error instanceof Error ? error.message : 'Failed to acknowledge alert',
        details: 'Make sure the monitoring engine is running'
      },
      { status: 503 }
    )
  }
}
