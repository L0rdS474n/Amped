// TypeScript mirror of the Amped plugin data contract (crates/amped-plugin-api).
// Keeping these aligned with the Rust/WIT types means that when M2/M4 wire real
// plugin data through Tauri IPC, the shape already matches — no translation layer.

/** Mirrors WIT `variant provenance`. Every piece of widget data is source-tagged. */
export type Provenance =
  | { kind: 'live' }
  | { kind: 'mock' }
  | { kind: 'cached'; ageSeconds: number }
  | { kind: 'degraded'; reason: string };

/** Mirrors WIT `record widget-data`. `payload` is decoded JSON on this side. */
export interface WidgetData<T = unknown> {
  widgetId: string;
  provenance: Provenance;
  fetchedAtMs: number;
  payload: T;
}

/** Display-level Easee charger state. NOTE: the exact Easee `chargerOpMode`
 *  integer→state mapping is [TO VERIFY] before M5 live data; these labels are
 *  used only for MOCK rendering. */
export type ChargerState =
  | 'charging'
  | 'ready'
  | 'awaiting'
  | 'completed'
  | 'disconnected'
  | 'error';

export interface ChargerStatus {
  chargerId: string;
  name: string;
  state: ChargerState;
  powerKw: number;
  sessionEnergyKwh: number;
  lifetimeEnergyKwh: number;
  online: boolean;
  cableLocked: boolean;
}

/** Subset of PluginManifest surfaced to the dashboard. */
export interface PluginInfo {
  id: string;
  name: string;
  version: string;
  author: string;
  description: string;
  permissions: { network: string[]; secrets: string[]; filesystem: string[] };
}

export function provenanceLabel(p: Provenance): string {
  switch (p.kind) {
    case 'live': return 'Live';
    case 'mock': return 'DEMO';
    case 'cached': return `Cachad ${Math.round(p.ageSeconds)}s`;
    case 'degraded': return 'Avbruten';
  }
}
