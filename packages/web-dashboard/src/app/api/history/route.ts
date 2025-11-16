// API Route: GET /api/history
// Returns analysis history

import { NextRequest, NextResponse } from 'next/server';
import storage from '@/lib/storage';
import type { HistoryResponse, HistoryItem } from '@/types/api';

export const runtime = 'nodejs';
export const dynamic = 'force-dynamic';

// Demo history data when no real analyses exist
const DEMO_HISTORY: HistoryItem[] = [
  {
    id: 'demo-defi-vault',
    filename: 'defi_vault.rs',
    uploadedAt: new Date(Date.now() - 1000 * 60 * 30).toISOString(), // 30 min ago
    result: {
      success: true,
      analysis_time_ms: 2,
      vulnerabilities_found: 6,
      vulnerabilities: [
        {
          severity: 'critical',
          vulnerability_type: 'Reentrancy Attack',
          description: 'State changes after external call in withdraw function',
          location: {
            file: 'defi_vault.rs',
            line: 156,
            column: 9,
            function: 'withdraw'
          },
          code_snippet: 'self.user_balances.insert(caller.clone(), remaining);',
          impact: 'Attacker can drain vault by repeatedly calling withdraw before balance update',
          recommendation: 'Move balance update before external call (Checks-Effects-Interactions pattern)',
          cwe_id: 'CWE-841',
          references: [
            'https://docs.substrate.io/build/troubleshoot-your-code/#reentrancy',
            'https://github.com/crytic/building-secure-contracts/blob/master/development-guidelines/workflow.md'
          ]
        },
        {
          severity: 'critical',
          vulnerability_type: 'Integer Overflow',
          description: 'Unchecked arithmetic in reward calculation',
          location: {
            file: 'defi_vault.rs',
            line: 201,
            column: 17,
            function: 'calculate_rewards'
          },
          code_snippet: 'let rewards = balance * reward_rate * time_elapsed;',
          impact: 'Overflow could result in incorrect reward amounts or vault insolvency',
          recommendation: 'Use checked_mul() and checked_add() for all arithmetic operations',
          cwe_id: 'CWE-190',
          references: ['https://docs.substrate.io/build/troubleshoot-your-code/#arithmetic-overflow']
        },
        {
          severity: 'high',
          vulnerability_type: 'Access Control',
          description: 'Missing authorization check in emergency_withdraw',
          location: {
            file: 'defi_vault.rs',
            line: 245,
            column: 5,
            function: 'emergency_withdraw'
          },
          code_snippet: 'pub fn emergency_withdraw(origin: OriginFor<T>) -> DispatchResult {',
          impact: 'Any user can trigger emergency withdrawal, potentially draining the vault',
          recommendation: 'Add ensure_root() or ensure_signed() with admin check',
          cwe_id: 'CWE-862',
          references: ['https://docs.substrate.io/build/origins/']
        },
        {
          severity: 'high',
          vulnerability_type: 'Price Oracle Manipulation',
          description: 'Using spot price without TWAP protection',
          location: {
            file: 'defi_vault.rs',
            line: 178,
            column: 13,
            function: 'get_asset_price'
          },
          code_snippet: 'let price = Self::oracle_price(asset_id)?;',
          impact: 'Flash loan attacks can manipulate spot prices for unfair liquidations',
          recommendation: 'Implement TWAP (Time-Weighted Average Price) or use multiple oracles',
          cwe_id: 'CWE-837',
          references: ['https://www.paradigm.xyz/2020/11/so-you-want-to-use-a-price-oracle']
        },
        {
          severity: 'medium',
          vulnerability_type: 'Unbounded Storage Growth',
          description: 'No limits on user_positions storage map',
          location: {
            file: 'defi_vault.rs',
            line: 89,
            column: 5,
            function: 'StorageMap declaration'
          },
          code_snippet: 'pub type UserPositions<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Position>;',
          impact: 'Unlimited position creation could lead to storage bloat and high costs',
          recommendation: 'Add MaxPositionsPerUser config parameter and enforce limit',
          cwe_id: 'CWE-400',
          references: ['https://docs.substrate.io/build/runtime-storage/']
        },
        {
          severity: 'medium',
          vulnerability_type: 'Front-Running',
          description: 'Swap function vulnerable to front-running attacks',
          location: {
            file: 'defi_vault.rs',
            line: 312,
            column: 5,
            function: 'swap_assets'
          },
          code_snippet: 'pub fn swap_assets(origin: OriginFor<T>, amount_in: Balance) -> DispatchResult {',
          impact: 'Attackers can front-run swaps to extract MEV',
          recommendation: 'Implement slippage protection and deadline parameters',
          cwe_id: 'CWE-362',
          references: ['https://consensys.github.io/smart-contract-best-practices/attacks/frontrunning/']
        }
      ]
    }
  },
  {
    id: 'demo-token-pallet',
    filename: 'token_pallet.rs',
    uploadedAt: new Date(Date.now() - 1000 * 60 * 60 * 2).toISOString(), // 2 hours ago
    result: {
      success: true,
      analysis_time_ms: 1,
      vulnerabilities_found: 2,
      vulnerabilities: [
        {
          severity: 'high',
          vulnerability_type: 'Integer Overflow',
          description: 'Unchecked addition in transfer function',
          location: {
            file: 'token_pallet.rs',
            line: 89,
            column: 13,
            function: 'transfer'
          },
          code_snippet: 'balances[to] = balances[to] + amount;',
          impact: 'Could allow minting unlimited tokens through overflow',
          recommendation: 'Use saturating_add() or checked_add()',
          cwe_id: 'CWE-190',
          references: ['https://docs.substrate.io/build/troubleshoot-your-code/#arithmetic-overflow']
        },
        {
          severity: 'medium',
          vulnerability_type: 'Missing Event Emission',
          description: 'Transfer function does not emit event',
          location: {
            file: 'token_pallet.rs',
            line: 85,
            column: 5,
            function: 'transfer'
          },
          code_snippet: 'pub fn transfer(origin: OriginFor<T>, to: T::AccountId, amount: Balance) -> DispatchResult {',
          impact: 'Off-chain systems cannot track transfers properly',
          recommendation: 'Add Self::deposit_event(Event::Transfer { from, to, amount });',
          cwe_id: 'CWE-223',
          references: ['https://docs.substrate.io/build/events-and-errors/']
        }
      ]
    }
  },
  {
    id: 'demo-governance',
    filename: 'governance.rs',
    uploadedAt: new Date(Date.now() - 1000 * 60 * 60 * 5).toISOString(), // 5 hours ago
    result: {
      success: true,
      analysis_time_ms: 3,
      vulnerabilities_found: 0,
      vulnerabilities: []
    }
  }
];

export async function GET(request: NextRequest) {
  try {
    const { searchParams } = new URL(request.url);
    const limitParam = searchParams.get('limit');
    const limit = limitParam ? parseInt(limitParam, 10) : 20;

    let history = storage.getHistory(limit);

    // If no history and in development, return demo data
    if (history.length === 0 && process.env.NODE_ENV === 'development') {
      history = DEMO_HISTORY.slice(0, limit);
    }

    return NextResponse.json<HistoryResponse>(
      {
        success: true,
        history,
      },
      { status: 200 }
    );
  } catch (error) {
    console.error('History error:', error);

    return NextResponse.json<HistoryResponse>(
      {
        success: false,
        history: [],
      },
      { status: 500 }
    );
  }
}
