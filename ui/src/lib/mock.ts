// MOCK data source — clearly synthetic. Nothing here comes from a real Easee
// charger or any live API. Every consumer renders a DEMO badge. Real data
// arrives at M4 (WASM mock plugin via the host) and M5 (live, gated on §1b).

import type { ChargerStatus, PluginInfo, WidgetData } from './contract';

export const MOCK_NOTICE =
  'Demodata — syntetiska värden, inte från en riktig Easee-laddare.';

export function mockCharger(): WidgetData<ChargerStatus> {
  return {
    widgetId: 'charger_status',
    provenance: { kind: 'mock' },
    fetchedAtMs: 0, // deterministic; no wall-clock in mock
    payload: {
      chargerId: 'EH000MOCK',
      name: 'Carport',
      state: 'charging',
      powerKw: 11.0,
      sessionEnergyKwh: 24.6,
      lifetimeEnergyKwh: 3187.4,
      online: true,
      cableLocked: true,
    },
  };
}

/** A deliberately degraded reading, to demonstrate graceful failure handling. */
export function mockChargerDegraded(): WidgetData<ChargerStatus> {
  return {
    widgetId: 'charger_status',
    provenance: { kind: 'degraded', reason: 'Laddaren är offline (moln-anslutning förlorad)' },
    fetchedAtMs: 0,
    payload: {
      chargerId: 'EH000MOCK',
      name: 'Carport',
      state: 'disconnected',
      powerKw: 0,
      sessionEnergyKwh: 24.6,
      lifetimeEnergyKwh: 3187.4,
      online: false,
      cableLocked: false,
    },
  };
}

/** Installed plugins — mirrors what amped-registry will load from
 *  plugins/easee/amped.plugin.toml. Hard-coded for the browser MVP; M3 wires
 *  this to a Tauri command that reads the real manifests. */
export const mockPlugins: PluginInfo[] = [
  {
    id: 'com.easee.charger',
    name: 'Easee Charger',
    version: '0.1.0',
    author: 'Amped',
    description: 'Read-only status för Easee-laddare (effekt, sessionsenergi, läge).',
    permissions: {
      network: ['api.easee.com'],
      secrets: ['easee.username', 'easee.password', 'easee.refresh_token'],
      filesystem: [],
    },
  },
];
