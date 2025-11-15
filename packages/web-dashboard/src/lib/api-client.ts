// API Client for SAFT Enhanced Dashboard
// Provides functions to interact with Next.js API routes

import type {
  AnalyzeResponse,
  StatsResponse,
  HistoryResponse,
} from '@/types/api';

const API_BASE = '/api';

export class APIError extends Error {
  constructor(
    message: string,
    public statusCode?: number,
    public response?: unknown
  ) {
    super(message);
    this.name = 'APIError';
  }
}

async function handleResponse<T>(response: Response): Promise<T> {
  if (!response.ok) {
    const errorData = await response.json().catch(() => ({}));
    throw new APIError(
      errorData.error || `HTTP ${response.status}: ${response.statusText}`,
      response.status,
      errorData
    );
  }

  return response.json();
}

/**
 * Upload and analyze a Rust pallet file
 */
export async function analyzePallet(file: File): Promise<AnalyzeResponse> {
  const formData = new FormData();
  formData.append('file', file);

  const response = await fetch(`${API_BASE}/analyze`, {
    method: 'POST',
    body: formData,
  });

  return handleResponse<AnalyzeResponse>(response);
}

/**
 * Get dashboard statistics
 */
export async function getStats(): Promise<StatsResponse> {
  const response = await fetch(`${API_BASE}/stats`, {
    cache: 'no-store',
  });

  return handleResponse<StatsResponse>(response);
}

/**
 * Get analysis history
 */
export async function getHistory(limit: number = 20): Promise<HistoryResponse> {
  const response = await fetch(`${API_BASE}/history?limit=${limit}`, {
    cache: 'no-store',
  });

  return handleResponse<HistoryResponse>(response);
}

/**
 * Get recent security alerts
 */
export async function getAlerts(limit: number = 10): Promise<{
  success: boolean;
  alerts: Array<{
    id: string;
    severity: 'critical' | 'high' | 'medium' | 'low';
    title: string;
    description: string;
    file: string;
    timestamp: Date;
  }>;
}> {
  const response = await fetch(`${API_BASE}/alerts?limit=${limit}`, {
    cache: 'no-store',
  });

  return handleResponse(response);
}

/**
 * Health check - verify SAFT binary availability
 */
export async function healthCheck(): Promise<{
  status: string;
  saft_binary: string;
  error?: string;
}> {
  const response = await fetch(`${API_BASE}/analyze`, {
    cache: 'no-store',
  });

  return handleResponse(response);
}
