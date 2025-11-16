// API Route: GET /api/demo-alerts
// Generates realistic demo alerts showcasing all detection capabilities
// ONLY works in development mode

import { NextResponse } from 'next/server';

// Comprehensive demo alerts showcasing all platform features
const DEMO_ALERTS = [
  // FLASH LOAN ATTACK (Critical)
  {
    id: 'demo-flash-loan-1',
    timestamp: Math.floor(Date.now() / 1000) - 180, // 3 min ago
    chain: 'polkadot',
    chain_name: 'Polkadot',
    severity: 'critical',
    pattern: 'Flash_Loan_Attack',
    description: 'Flash loan attack detected: Large borrow (150,000 DOT) followed by immediate swap and repay within single block',
    confidence: 0.92,
    evidence: [
      'Borrowed 150,000 DOT from liquidity pool',
      'Executed swap causing 8% price impact',
      'Repaid loan + 0.3% fee in same transaction',
      'Net profit: 12,000 DOT (~$84,000 USD)',
    ],
    transaction_hash: '0x7f9fade1c0d57a7af66ab4ead79fade1c0d57a7af66ab4ead7c2c2eb7b11a91385',
    block_number: 18234567,
    metadata: {
      pool: 'DOT/USDT Omnipool',
      attacker: '13UVJy...xDqSFe',
      amount_borrowed: '150000 DOT',
      profit: '12000 DOT',
    },
    recommended_actions: [
      'Review transaction on block explorer',
      'Investigate borrower address for additional suspicious activity',
      'Consider implementing circuit breaker for large flash loans',
      'Alert liquidity pool maintainers',
    ],
    acknowledged: false,
  },

  // MEV SANDWICH ATTACK (High)
  {
    id: 'demo-mev-sandwich-1',
    timestamp: Math.floor(Date.now() / 1000) - 420, // 7 min ago
    chain: 'hydration',
    chain_name: 'Hydration',
    severity: 'high',
    pattern: 'MEV_Sandwich',
    description: 'MEV sandwich attack: Attacker frontran user swap with 50,000 HDX buy order, then backran with sell order',
    confidence: 0.88,
    evidence: [
      'Block #12345: Buy 50,000 HDX (position 3)',
      'Block #12345: Victim swap 10,000 USDT → HDX (position 4)',
      'Block #12345: Sell 50,000 HDX (position 5)',
      'Price manipulation: +4.2%',
      'Victim slippage: 3.8%',
      'Attacker profit: ~1,800 HDX',
    ],
    transaction_hash: '0x3a9fade1c0d57a7af66ab4ead79fade1c0d57a7af66ab4ead7c2c2eb7b11a91123',
    block_number: 12345,
    metadata: {
      attacker: '14VJyP...zRqWHg',
      victim: '15ZKmQ...aLpVnX',
      pool: 'HDX/USDT Omnipool',
      profit_usd: '$540',
    },
    recommended_actions: [
      'Flag attacker address for monitoring',
      'Notify affected user',
      'Review mempool ordering mechanisms',
      'Consider private transaction pools',
    ],
    acknowledged: false,
  },

  // ORACLE MANIPULATION (Critical)
  {
    id: 'demo-oracle-manip-1',
    timestamp: Math.floor(Date.now() / 1000) - 600, // 10 min ago
    chain: 'polkadot',
    chain_name: 'Polkadot',
    severity: 'critical',
    pattern: 'Oracle_Manipulation',
    description: 'Price oracle manipulation: Multiple large orders placed to manipulate TWAP oracle price by 12%',
    confidence: 0.95,
    evidence: [
      'Series of 8 large orders within 2-minute window',
      'Total volume: 2.3M DOT ($16.1M USD)',
      'TWAP oracle price moved 12.4% from baseline',
      'Suspiciously followed by large leveraged position',
      'All orders from 3 coordinated addresses',
    ],
    transaction_hash: '0x9fade1c0d57a7af66ab4ead79fade1c0d57a7af66ab4ead7c2c2eb7b11a91456',
    block_number: 18234520,
    metadata: {
      addresses: '3 coordinated accounts',
      volume: '2.3M DOT',
      price_impact: '12.4%',
      leverage_position: '500K DOT',
    },
    recommended_actions: [
      'URGENT: Pause affected oracle feeds',
      'Investigate coordinated addresses',
      'Review oracle update mechanisms',
      'Alert dependent protocols (lending, derivatives)',
      'Consider implementing price bounds',
    ],
    acknowledged: false,
  },

  // HYPERBRIDGE CROSS-CHAIN ATTACK (High)
  {
    id: 'demo-crosschain-replay-1',
    timestamp: Math.floor(Date.now() / 1000) - 900, // 15 min ago
    chain: 'polkadot',
    chain_name: 'Polkadot',
    severity: 'high',
    pattern: 'CrossChainReplay',
    description: 'Cross-chain replay attack detected: Same message ID used across multiple parachains via Hyperbridge',
    confidence: 0.86,
    evidence: [
      'Message ID 0x7a8b9c... appears on Polkadot relay chain',
      'Same message ID detected on Asset Hub',
      'Duplicate ISMP state proof submission',
      'Nonce validation bypassed',
      'Potential double-spend: 25,000 DOT',
    ],
    transaction_hash: '0x2fade1c0d57a7af66ab4ead79fade1c0d57a7af66ab4ead7c2c2eb7b11a91789',
    block_number: 18234500,
    metadata: {
      message_id: '0x7a8b9c...',
      source_chain: 'Polkadot Relay',
      target_chain: 'Asset Hub',
      amount: '25000 DOT',
    },
    recommended_actions: [
      'Halt cross-chain message processing',
      'Verify ISMP state proof integrity',
      'Review message ID uniqueness constraints',
      'Contact Hyperbridge security team',
      'Implement message ID blacklist',
    ],
    acknowledged: false,
  },

  // HYDRATION LIQUIDITY DRAIN (Medium)
  {
    id: 'demo-liquidity-drain-1',
    timestamp: Math.floor(Date.now() / 1000) - 1200, // 20 min ago
    chain: 'hydration',
    chain_name: 'Hydration',
    severity: 'medium',
    pattern: 'LiquidityDrain',
    description: 'Gradual liquidity drain: 22% of HDX/DOT Omnipool liquidity removed over 45-minute period',
    confidence: 0.79,
    evidence: [
      'Initial liquidity: 1.2M HDX',
      'Current liquidity: 936K HDX (22% decrease)',
      'Spread across 15 withdrawal transactions',
      'No corresponding deposits',
      'Pool imbalance increasing',
      'Slippage impact: +6.8%',
    ],
    transaction_hash: '0x8fade1c0d57a7af66ab4ead79fade1c0d57a7af66ab4ead7c2c2eb7b11a91234',
    block_number: 12300,
    metadata: {
      pool: 'HDX/DOT Omnipool',
      initial_liquidity: '1.2M HDX',
      current_liquidity: '936K HDX',
      withdrawals: '15 transactions',
      addresses: '8 unique addresses',
    },
    recommended_actions: [
      'Monitor pool health metrics',
      'Check for exploit indicators',
      'Verify smart contract integrity',
      'Alert liquidity providers',
      'Consider temporary withdrawal limits',
    ],
    acknowledged: false,
  },

  // VOLUME ANOMALY (High)
  {
    id: 'demo-volume-anomaly-1',
    timestamp: Math.floor(Date.now() / 1000) - 1800, // 30 min ago
    chain: 'kusama',
    chain_name: 'Kusama',
    severity: 'high',
    pattern: 'Volume_Anomaly',
    description: 'Abnormal trading volume spike: 850% increase in 5-minute window, possible pump-and-dump coordination',
    confidence: 0.82,
    evidence: [
      'Normal 5-min volume: ~50,000 KSM',
      'Detected volume: 425,000 KSM (850% increase)',
      'Price increased 18% in first 3 minutes',
      'Large sell orders placed at peak price',
      'Volume normalized after 7 minutes',
      'Social media activity spike detected',
    ],
    transaction_hash: '0x2fade1c0d57a7af66ab4ead79fade1c0d57a7af66ab4ead7c2c2eb7b11a91789',
    block_number: 21345678,
    metadata: {
      pair: 'KSM/USDT',
      volume_increase: '850%',
      price_peak: '+18%',
      coordinated_wallets: '12+',
      telegram_mentions: '+300%',
    },
    recommended_actions: [
      'Monitor for follow-up activity',
      'Flag coordinated wallet addresses',
      'Review social media for pump signals',
      'Alert exchange integrations',
      'Implement circuit breaker if continues',
    ],
    acknowledged: false,
  },

  // COLLATERAL MANIPULATION (Medium)
  {
    id: 'demo-collateral-manip-1',
    timestamp: Math.floor(Date.now() / 1000) - 2400, // 40 min ago
    chain: 'hydration',
    chain_name: 'Hydration',
    severity: 'medium',
    pattern: 'CollateralManipulation',
    description: 'Collateral ratio manipulation: User artificially inflating collateral value before large borrow',
    confidence: 0.74,
    evidence: [
      'Deposited 100K HDX as collateral',
      'Price pumped via coordinated buys (+15%)',
      'Borrowed max against inflated collateral',
      'Immediately sold borrowed assets',
      'Collateral value dropping rapidly',
    ],
    transaction_hash: '0x6fade1c0d57a7af66ab4ead79fade1c0d57a7af66ab4ead7c2c2eb7b11a91567',
    block_number: 12250,
    metadata: {
      user: '16KmPq...bNpYxZ',
      collateral: '100K HDX',
      borrowed: '80K USDT',
      price_manipulation: '+15%',
      liquidation_risk: 'High',
    },
    recommended_actions: [
      'Monitor collateral health',
      'Flag position for liquidation',
      'Review collateral price feed integrity',
      'Consider using TWAP for collateral pricing',
    ],
    acknowledged: false,
  },

  // FRONTRUNNING ATTACK (Low)
  {
    id: 'demo-frontrun-1',
    timestamp: Math.floor(Date.now() / 1000) - 3000, // 50 min ago
    chain: 'kusama',
    chain_name: 'Kusama',
    severity: 'low',
    pattern: 'FrontRunning',
    description: 'Frontrunning detected: Transaction executed immediately before victim with higher priority',
    confidence: 0.68,
    evidence: [
      'Victim tx in mempool: swap 5K KSM → USDT',
      'Attacker tx submitted 200ms later',
      'Higher transaction fee: +25%',
      'Executed in same block (position 12 vs 13)',
      'Victim experienced 2.1% additional slippage',
    ],
    transaction_hash: '0x567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234',
    block_number: 21345650,
    metadata: {
      attacker: '17VJkP...qWzHgL',
      victim: '18KmQx...pVnXyZ',
      victim_slippage: '2.1%',
      attacker_profit: '~150 KSM',
    },
    recommended_actions: [
      'Educate users about MEV protection',
      'Implement private transaction pools',
      'Consider commit-reveal schemes',
      'Monitor attacker address',
    ],
    acknowledged: false,
  },
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
