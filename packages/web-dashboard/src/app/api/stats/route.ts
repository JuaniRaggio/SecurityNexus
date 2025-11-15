// API Route: GET /api/stats
// Returns dashboard statistics

import { NextResponse } from 'next/server';
import storage from '@/lib/storage';
import type { StatsResponse } from '@/types/api';

export const runtime = 'nodejs';
export const dynamic = 'force-dynamic';

export async function GET() {
  try {
    const stats = storage.getStats();

    return NextResponse.json<StatsResponse>(
      {
        success: true,
        stats,
      },
      { status: 200 }
    );
  } catch (error) {
    console.error('Stats error:', error);

    return NextResponse.json<StatsResponse>(
      {
        success: false,
        stats: {
          totalPalletsAnalyzed: 0,
          activeAlerts: 0,
          securityScore: 0,
          chainsMonitored: 0,
        },
      },
      { status: 500 }
    );
  }
}
