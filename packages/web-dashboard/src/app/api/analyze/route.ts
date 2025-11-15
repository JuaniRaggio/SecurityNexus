// API Route: POST /api/analyze
// Handles pallet file upload and analysis

import { NextRequest, NextResponse } from 'next/server';
import { analyzePallet, checkSAFTBinary } from '@/lib/saft-client';
import storage from '@/lib/storage';
import type { AnalyzeResponse } from '@/types/api';

export const runtime = 'nodejs';
export const dynamic = 'force-dynamic';

// Increase body size limit for file uploads (10MB)
export const bodyParser = {
  sizeLimit: '10mb',
};

export async function POST(request: NextRequest) {
  try {
    // Check if SAFT binary is available
    const binaryCheck = await checkSAFTBinary();
    if (!binaryCheck.exists) {
      return NextResponse.json<AnalyzeResponse>(
        {
          success: false,
          analysisId: '',
          error: binaryCheck.error || 'SAFT binary not found',
        },
        { status: 500 }
      );
    }

    // Parse request body
    const formData = await request.formData();
    const file = formData.get('file') as File | null;

    if (!file) {
      return NextResponse.json<AnalyzeResponse>(
        {
          success: false,
          analysisId: '',
          error: 'No file provided',
        },
        { status: 400 }
      );
    }

    // Validate file extension
    if (!file.name.endsWith('.rs')) {
      return NextResponse.json<AnalyzeResponse>(
        {
          success: false,
          analysisId: '',
          error: 'Only Rust (.rs) files are supported',
        },
        { status: 400 }
      );
    }

    // Read file content
    const content = await file.text();

    // Validate file size (max 1MB)
    if (content.length > 1024 * 1024) {
      return NextResponse.json<AnalyzeResponse>(
        {
          success: false,
          analysisId: '',
          error: 'File too large (max 1MB)',
        },
        { status: 400 }
      );
    }

    // Analyze the pallet
    const result = await analyzePallet(file.name, content);

    // Store in history
    const analysisId = storage.addAnalysis(file.name, result);

    // Return success response
    return NextResponse.json<AnalyzeResponse>(
      {
        success: true,
        analysisId,
        result,
      },
      { status: 200 }
    );
  } catch (error) {
    console.error('Analysis error:', error);

    return NextResponse.json<AnalyzeResponse>(
      {
        success: false,
        analysisId: '',
        error: error instanceof Error ? error.message : 'Analysis failed',
      },
      { status: 500 }
    );
  }
}

// Health check endpoint
export async function GET() {
  const binaryCheck = await checkSAFTBinary();

  return NextResponse.json(
    {
      status: binaryCheck.exists ? 'healthy' : 'unavailable',
      saft_binary: binaryCheck.path,
      error: binaryCheck.error,
    },
    { status: binaryCheck.exists ? 200 : 503 }
  );
}
