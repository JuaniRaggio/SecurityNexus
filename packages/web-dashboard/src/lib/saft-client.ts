// SAFT Enhanced binary client
// Executes the SAFT binary and parses the JSON output

import { exec } from 'child_process';
import { promisify } from 'util';
import { writeFile, unlink, mkdir } from 'fs/promises';
import { existsSync } from 'fs';
import path from 'path';
import type { SAFTAnalysisResult } from '@/types/api';

const execAsync = promisify(exec);

// Configuration
const SAFT_BINARY_PATH = process.env.SAFT_BINARY_PATH ||
  path.join(process.cwd(), '../../target/release/saft');
const TEMP_UPLOAD_DIR = process.env.TEMP_UPLOAD_DIR || '/tmp/saft-uploads';

/**
 * Ensure temp directory exists
 */
async function ensureTempDir(): Promise<void> {
  if (!existsSync(TEMP_UPLOAD_DIR)) {
    await mkdir(TEMP_UPLOAD_DIR, { recursive: true });
  }
}

/**
 * Check if SAFT binary exists and is executable
 */
export async function checkSAFTBinary(): Promise<{ exists: boolean; path: string; error?: string }> {
  try {
    if (!existsSync(SAFT_BINARY_PATH)) {
      return {
        exists: false,
        path: SAFT_BINARY_PATH,
        error: `SAFT binary not found at ${SAFT_BINARY_PATH}. Please build it with: cargo build --release --package saft-enhanced`,
      };
    }

    // Try to execute version command
    await execAsync(`${SAFT_BINARY_PATH} --version`);

    return {
      exists: true,
      path: SAFT_BINARY_PATH,
    };
  } catch (error) {
    return {
      exists: false,
      path: SAFT_BINARY_PATH,
      error: error instanceof Error ? error.message : 'Unknown error checking SAFT binary',
    };
  }
}

/**
 * Analyze a Rust source file using SAFT Enhanced
 */
export async function analyzePallet(
  filename: string,
  content: string
): Promise<SAFTAnalysisResult> {
  await ensureTempDir();

  // Generate unique temp filename
  const tempFilename = `${Date.now()}-${Math.random().toString(36).substr(2, 9)}.rs`;
  const tempFilePath = path.join(TEMP_UPLOAD_DIR, tempFilename);

  try {
    // Write content to temp file
    await writeFile(tempFilePath, content, 'utf-8');

    // Execute SAFT binary with JSON output
    const command = `${SAFT_BINARY_PATH} analyze "${tempFilePath}" --format json`;

    const { stdout } = await execAsync(command, {
      maxBuffer: 10 * 1024 * 1024, // 10MB buffer
      timeout: 30000, // 30 second timeout
    });

    // Parse JSON output
    let analysisResult: SAFTAnalysisResult[];

    try {
      analysisResult = JSON.parse(stdout);
    } catch (parseError) {
      console.error('Failed to parse SAFT output:', stdout);
      throw new Error('Failed to parse SAFT analysis output');
    }

    // SAFT returns an array, we take the first element
    if (!analysisResult || analysisResult.length === 0) {
      throw new Error('SAFT returned empty result');
    }

    const result = analysisResult[0];

    // Update the file path to the original filename
    result.file = filename;

    return result;
  } catch (error) {
    console.error('SAFT analysis error:', error);

    if (error instanceof Error) {
      if (error.message.includes('timeout')) {
        throw new Error('Analysis timed out. The file may be too large or complex.');
      }
      throw new Error(`Analysis failed: ${error.message}`);
    }

    throw new Error('Unknown error during analysis');
  } finally {
    // Clean up temp file
    try {
      await unlink(tempFilePath);
    } catch (cleanupError) {
      console.error('Failed to clean up temp file:', cleanupError);
    }
  }
}

/**
 * Get SAFT version information
 */
export async function getSAFTVersion(): Promise<string> {
  try {
    const { stdout } = await execAsync(`${SAFT_BINARY_PATH} --version`);
    return stdout.trim();
  } catch (error) {
    return 'Unknown';
  }
}

/**
 * List available SAFT rules
 */
export async function listSAFTRules(): Promise<string[]> {
  try {
    const { stdout } = await execAsync(`${SAFT_BINARY_PATH} rules`);
    return stdout.trim().split('\n').filter(line => line.length > 0);
  } catch (error) {
    return [];
  }
}
