// API Route: POST /api/demo-alerts
// Generates demo alerts for testing/demo purposes
// ONLY works in development mode

import { NextResponse } from 'next/server';

const DEMO_ALERTS = [
  {
    id: `alert-${Date.now()}-1`,
    timestamp: Math.floor(Date.now() / 1000),
    chain: "Kusama",
    severity: "critical",
    pattern: "FLASH_LOAN",
    description: "Potential flash loan attack: borrow + 3 DeFi interactions + repay in single transaction",
    transaction_hash: "0x1a2b3c4d5e6f7890abcdef1234567890abcdef1234567890abcdef1234567890",
    block_number: 12345678,
    metadata: {
      dex_interactions: "3",
      balance_change: "150%",
      protocol: "Karura DEX"
    },
    recommended_actions: [
      "Review transaction for unusual patterns",
      "Check Karura protocol for vulnerabilities",
      "Monitor related addresses for similar activity"
    ],
    acknowledged: false
  },
  {
    id: `alert-${Date.now()}-2`,
    timestamp: Math.floor(Date.now() / 1000) - 120,
    chain: "Kusama",
    severity: "high",
    pattern: "SANDWICH_ATTACK",
    description: "Sandwich attack: Attacker surrounds victim transaction",
    transaction_hash: "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
    block_number: 12345675,
    metadata: {
      victim_address: "0x789...def",
      attacker_address: "0xabc...123",
      estimated_profit: "500 KSM"
    },
    recommended_actions: [
      "Warn users about this attacker address",
      "Consider implementing MEV protection",
      "Report to Kusama security team"
    ],
    acknowledged: false
  },
  {
    id: `alert-${Date.now()}-3`,
    timestamp: Math.floor(Date.now() / 1000) - 300,
    chain: "Kusama",
    severity: "medium",
    pattern: "FRONT_RUNNING",
    description: "Frontrunning: Transaction executed before victim",
    transaction_hash: "0x567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234",
    block_number: 12345670,
    metadata: {
      victim_gas_price: "100 Gwei",
      attacker_gas_price: "150 Gwei"
    },
    recommended_actions: [
      "Implement private transaction pools",
      "Use commit-reveal schemes"
    ],
    acknowledged: false
  },
  {
    id: `alert-${Date.now()}-4`,
    timestamp: Math.floor(Date.now() / 1000) - 600,
    chain: "Kusama",
    severity: "low",
    pattern: "VOLUME_ANOMALY",
    description: "Unusual trading volume spike detected",
    transaction_hash: "0x234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
    block_number: 12345665,
    metadata: {
      normal_volume: "1000 KSM/hour",
      current_volume: "5000 KSM/hour",
      increase: "400%"
    },
    recommended_actions: [
      "Monitor for potential pump and dump",
      "Check for coordinated trading"
    ],
    acknowledged: false
  }
];

export async function GET() {
  // Only allow in development
  if (process.env.NODE_ENV !== 'development') {
    return NextResponse.json(
      { error: 'Demo alerts only available in development mode' },
      { status: 403 }
    );
  }

  return NextResponse.json(DEMO_ALERTS);
}

export async function POST() {
  // Only allow in development
  if (process.env.NODE_ENV !== 'development') {
    return NextResponse.json(
      { error: 'Demo alerts only available in development mode' },
      { status: 403 }
    );
  }

  return NextResponse.json({
    success: true,
    alerts: DEMO_ALERTS,
    message: 'Demo alerts generated'
  });
}
