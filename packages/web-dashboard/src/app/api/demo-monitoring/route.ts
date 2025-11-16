import { NextRequest, NextResponse } from 'next/server'

// Mock data generator for demo purposes
function generateMockAttackTrends(hours: number = 24) {
  const trends = []
  const now = new Date()

  // Generate data for the last N hours
  for (let i = 0; i < hours; i += 6) {
    const timestamp = new Date(now.getTime() - i * 60 * 60 * 1000)

    // Hyperbridge patterns
    if (Math.random() > 0.5) {
      trends.push({
        hour: timestamp.toISOString(),
        attack_pattern: 'CrossChainReplay',
        count: Math.floor(Math.random() * 5) + 1,
        avg_confidence: 0.75 + Math.random() * 0.2
      })
    }

    if (Math.random() > 0.6) {
      trends.push({
        hour: timestamp.toISOString(),
        attack_pattern: 'StateProofForgery',
        count: Math.floor(Math.random() * 3) + 1,
        avg_confidence: 0.8 + Math.random() * 0.15
      })
    }

    // Hydration DeFi patterns
    if (Math.random() > 0.4) {
      trends.push({
        hour: timestamp.toISOString(),
        attack_pattern: 'OmnipoolManipulation',
        count: Math.floor(Math.random() * 8) + 1,
        avg_confidence: 0.7 + Math.random() * 0.25
      })
    }

    if (Math.random() > 0.7) {
      trends.push({
        hour: timestamp.toISOString(),
        attack_pattern: 'LiquidityDrain',
        count: Math.floor(Math.random() * 4) + 1,
        avg_confidence: 0.65 + Math.random() * 0.3
      })
    }

    if (Math.random() > 0.8) {
      trends.push({
        hour: timestamp.toISOString(),
        attack_pattern: 'CollateralManipulation',
        count: Math.floor(Math.random() * 6) + 1,
        avg_confidence: 0.72 + Math.random() * 0.23
      })
    }
  }

  return trends
}

export async function GET(request: NextRequest) {
  const searchParams = request.nextUrl.searchParams
  const hours = parseInt(searchParams.get('hours') || '24')

  // Generate mock data
  const trends = generateMockAttackTrends(hours)

  return NextResponse.json(trends)
}
