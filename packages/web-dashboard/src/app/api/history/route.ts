// API Route: GET /api/history
// Returns analysis history

import { NextRequest, NextResponse } from 'next/server';
import storage from '@/lib/storage';
import type { HistoryResponse } from '@/types/api';

export const runtime = 'nodejs';
export const dynamic = 'force-dynamic';

export async function GET(request: NextRequest) {
  try {
    const { searchParams } = new URL(request.url);
    const limitParam = searchParams.get('limit');
    const limit = limitParam ? parseInt(limitParam, 10) : 20;

    const history = storage.getHistory(limit);

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
