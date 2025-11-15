// API Route: GET /api/alerts
// Returns recent security alerts from vulnerability scans

import { NextRequest, NextResponse } from 'next/server';
import storage from '@/lib/storage';
import type { Alert } from '@/types/api';

export const runtime = 'nodejs';
export const dynamic = 'force-dynamic';

export async function GET(request: NextRequest) {
  try {
    const { searchParams } = new URL(request.url);
    const limitParam = searchParams.get('limit');
    const limit = limitParam ? parseInt(limitParam, 10) : 10;

    const alerts = storage.getRecentAlerts(limit);

    return NextResponse.json<{ success: boolean; alerts: Alert[] }>(
      {
        success: true,
        alerts,
      },
      { status: 200 }
    );
  } catch (error) {
    console.error('Alerts error:', error);

    return NextResponse.json<{ success: boolean; alerts: Alert[] }>(
      {
        success: false,
        alerts: [],
      },
      { status: 500 }
    );
  }
}
