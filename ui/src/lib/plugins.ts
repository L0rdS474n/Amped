// Plugin loader: real manifests from the Tauri backend when running inside the
// desktop shell, mock data when previewing in a plain browser. The dashboard is
// identical either way — only the data source differs.

import type { PluginInfo } from './contract';
import { mockPlugins } from './mock';

export interface PluginLoad {
  plugins: PluginInfo[];
  source: 'tauri' | 'mock';
}

/** True when running inside the Tauri webview (vs a plain browser tab). */
function inTauri(): boolean {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
}

export async function loadPlugins(): Promise<PluginLoad> {
  if (!inTauri()) {
    return { plugins: mockPlugins, source: 'mock' };
  }
  try {
    const { invoke } = await import('@tauri-apps/api/core');
    const plugins = await invoke<PluginInfo[]>('list_plugins');
    return { plugins, source: 'tauri' };
  } catch (err) {
    // Backend command failed — degrade gracefully to mock rather than blanking.
    console.error('list_plugins failed, falling back to mock:', err);
    return { plugins: mockPlugins, source: 'mock' };
  }
}
