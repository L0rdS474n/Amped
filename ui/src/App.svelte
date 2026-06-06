<script lang="ts">
  import { onMount } from 'svelte';
  import TopBar from './lib/components/TopBar.svelte';
  import ChargerCard from './lib/components/ChargerCard.svelte';
  import ComingSoonCard from './lib/components/ComingSoonCard.svelte';
  import PluginPanel from './lib/components/PluginPanel.svelte';
  import { mockCharger, mockChargerDegraded, MOCK_NOTICE } from './lib/mock';
  import type { PluginInfo } from './lib/contract';
  import { loadPlugins } from './lib/plugins';

  // Toggle a degraded reading to demonstrate graceful failure handling.
  let degraded = $state(false);
  const charger = $derived(degraded ? mockChargerDegraded() : mockCharger());

  // Installed plugins: real manifests via Tauri (desktop) or mock (browser preview).
  let plugins = $state<PluginInfo[]>([]);
  let pluginSource = $state<'tauri' | 'mock'>('mock');
  onMount(async () => {
    const r = await loadPlugins();
    plugins = r.plugins;
    pluginSource = r.source;
  });

  // Subtle "flow": gently nudge the live power so the dashboard feels alive.
  // Still mock — the value oscillates by a few hundred watts only.
  let jitter = $state(0);
  $effect(() => {
    const id = setInterval(() => {
      jitter = Math.sin(Date.now() / 2200) * 0.4;
    }, 1200);
    return () => clearInterval(id);
  });

  const upcoming = [
    { icon: 'ph-sun',              title: 'Solproduktion', desc: 'Solceller: produktion, export och egenförbrukning.' },
    { icon: 'ph-battery-charging', title: 'Hembatteri',    desc: 'Laddning, urladdning och State of Charge.' },
    { icon: 'ph-chart-line-up',    title: 'Elpris',        desc: 'Spotpris per timme och billigaste laddfönster.' },
    { icon: 'ph-cloud-sun',        title: 'Väder',         desc: 'Sol- och vindprognos för energiplanering.' },
  ];
</script>

<main>
  <TopBar />

  <p class="notice"><i class="ph ph-info"></i> {MOCK_NOTICE}</p>

  <section class="grid">
    <ChargerCard data={charger} {jitter} />
    {#each upcoming as c (c.title)}
      <ComingSoonCard icon={c.icon} title={c.title} desc={c.desc} />
    {/each}
  </section>

  <PluginPanel {plugins} source={pluginSource} />

  <div class="controls">
    <button onclick={() => (degraded = !degraded)}>
      <i class="ph {degraded ? 'ph-arrow-counter-clockwise' : 'ph-warning'}"></i>
      {degraded ? 'Återställ mock' : 'Simulera laddarfel'}
    </button>
    <span class="hint">Visar hur dashboarden hanterar avbrott utan att krascha.</span>
  </div>

  <footer>
    Amped · M0–M1 + UI-skal · alla värden är <strong>demodata</strong> tills live-Easee (M5).
  </footer>
</main>

<style>
  main {
    max-width: 1180px;
    margin: 0 auto;
    padding: 28px 24px 48px;
  }
  .notice {
    display: flex; align-items: center; gap: 9px;
    color: var(--text-dim); font-size: 13px;
    margin-bottom: 18px;
  }
  .notice i { color: var(--teal); }
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
    gap: var(--gap);
  }
  .controls {
    display: flex; align-items: center; gap: 14px; flex-wrap: wrap;
    margin-top: 22px;
  }
  button {
    display: inline-flex; align-items: center; gap: 8px;
    padding: 10px 16px; border-radius: 12px;
    background: var(--surface-2); color: var(--text);
    border: 1px solid var(--border-strong);
    font-size: 13px; font-weight: 600; cursor: pointer;
    transition: transform 0.2s var(--ease), background 0.2s var(--ease);
  }
  button:hover { transform: translateY(-1px); background: rgba(46,226,122,0.10); }
  .hint { color: var(--text-faint); font-size: 12px; }
  footer {
    margin-top: 36px; padding-top: 18px;
    border-top: 1px solid var(--border);
    color: var(--text-faint); font-size: 12.5px; text-align: center;
  }
  footer strong { color: var(--green); font-weight: 600; }
</style>
