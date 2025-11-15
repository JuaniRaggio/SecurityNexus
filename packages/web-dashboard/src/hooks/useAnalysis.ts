// React Query hooks for SAFT Enhanced analysis

import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { analyzePallet, getStats, getHistory, getAlerts, healthCheck } from '@/lib/api-client';
import type { AnalyzeResponse } from '@/types/api';

/**
 * Query key factory for consistent cache management
 */
export const analysisKeys = {
  all: ['analysis'] as const,
  stats: () => [...analysisKeys.all, 'stats'] as const,
  history: (limit?: number) =>
    [...analysisKeys.all, 'history', limit] as const,
  alerts: (limit?: number) =>
    [...analysisKeys.all, 'alerts', limit] as const,
  health: () => [...analysisKeys.all, 'health'] as const,
};

/**
 * Hook to get dashboard statistics
 */
export function useStats() {
  return useQuery({
    queryKey: analysisKeys.stats(),
    queryFn: getStats,
    staleTime: 10000, // 10 seconds
    refetchInterval: 30000, // Refetch every 30 seconds
  });
}

/**
 * Hook to get analysis history
 */
export function useHistory(limit: number = 20) {
  return useQuery({
    queryKey: analysisKeys.history(limit),
    queryFn: () => getHistory(limit),
    staleTime: 5000, // 5 seconds
    refetchInterval: 15000, // Refetch every 15 seconds
  });
}

/**
 * Hook to get recent security alerts
 */
export function useAlerts(limit: number = 10) {
  return useQuery({
    queryKey: analysisKeys.alerts(limit),
    queryFn: () => getAlerts(limit),
    staleTime: 5000, // 5 seconds
    refetchInterval: 15000, // Refetch every 15 seconds
  });
}

/**
 * Hook to check SAFT binary health
 */
export function useHealthCheck() {
  return useQuery({
    queryKey: analysisKeys.health(),
    queryFn: healthCheck,
    staleTime: 60000, // 1 minute
    retry: 1,
  });
}

/**
 * Hook to analyze a pallet file
 */
export function useAnalyzePallet() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: analyzePallet,
    onSuccess: (_data: AnalyzeResponse) => {
      // Invalidate and refetch stats and history after successful analysis
      queryClient.invalidateQueries({ queryKey: analysisKeys.stats() });
      queryClient.invalidateQueries({ queryKey: analysisKeys.all });
    },
  });
}
