<script lang="ts">
  import type { WidgetData, ChargerStatus, ChargerState } from '../contract';
  import ProvenanceBadge from './ProvenanceBadge.svelte';

  let { data, jitter = 0 }: { data: WidgetData<ChargerStatus>; jitter?: number } = $props();

  const s = $derived(data.payload);
  const degraded = $derived(data.provenance.kind === 'degraded');

  const STATE: Record<ChargerState, { label: string; icon: string; tone: string }> = {
    charging:     { label: 'Laddar',      icon: 'ph-lightning',       tone: 'green' },
    ready:        { label: 'Redo',        icon: 'ph-check-circle',    tone: 'green' },
    awaiting:     { label: 'Väntar',      icon: 'ph-hourglass-medium', tone: 'amber' },
    completed:    { label: 'Klar',        icon: 'ph-flag-checkered',  tone: 'green' },
    disconnected: { label: 'Frånkopplad', icon: 'ph-plugs',           tone: 'faint' },
    error:        { label: 'Fel',         icon: 'ph-warning-circle',  tone: 'red' },
  };

  const view = $derived(STATE[s.state]);
  const power = $derived(Math.max(0, s.powerKw + (s.state === 'charging' ? jitter : 0)));
  const pct = $derived(Math.min(100, (power / 22) * 100));
</script>

<article class="glass card" class:degraded data-tone={view.tone}>
  <div class="head">
    <div class="title">
      <span class="ic"><i class="ph-fill ph-charging-station"></i></span>
      <div>
        <h3>{s.name}</h3>
        <p class="sub">Easee · {s.chargerId}</p>
      </div>
    </div>
    <ProvenanceBadge p={data.provenance} />
  </div>

  {#if degraded}
    <div class="degraded-note">
      <i class="ph ph-warning"></i>
      <span>{data.provenance.kind === 'degraded' ? data.provenance.reason : 'Otillgänglig'}</span>
    </div>
  {/if}

  <div class="state">
    <span class="state-ic" data-tone={view.tone}><i class="ph {view.icon}"></i></span>
    <span class="state-label">{view.label}</span>
    <span class="pill" class:is-on={s.online} class:is-warn={!s.online} style="margin-left:auto">
      <span class="dot"></span> {s.online ? 'Online' : 'Offline'}
    </span>
  </div>

  <div class="power">
    <span class="num">{power.toFixed(1)}</span>
    <span class="unit">kW</span>
  </div>
  <div class="flowbar"><span style="width:{degraded ? 0 : pct}%"></span></div>

  <div class="stats">
    <div>
      <span class="k"><i class="ph-light ph-battery-charging"></i> Sessionsenergi</span>
      <span class="v">{s.sessionEnergyKwh.toFixed(1)} <em>kWh</em></span>
    </div>
    <div>
      <span class="k"><i class="ph-light ph-gauge"></i> Livstid</span>
      <span class="v">{s.lifetimeEnergyKwh.toLocaleString('sv-SE')} <em>kWh</em></span>
    </div>
    <div>
      <span class="k"><i class="ph-light ph-lock-key"></i> Kabel</span>
      <span class="v">{s.cableLocked ? 'Låst' : 'Olåst'}</span>
    </div>
  </div>
</article>

<style>
  .card {
    grid-column: span 2;
    padding: 22px;
    display: flex;
    flex-direction: column;
    gap: 16px;
    box-shadow: var(--glow-green);
    transition: opacity 0.3s var(--ease);
  }
  .card.degraded { box-shadow: 0 0 0 1px rgba(246,185,74,0.25); opacity: 0.92; }
  .head { display: flex; align-items: flex-start; justify-content: space-between; gap: 12px; }
  .title { display: flex; align-items: center; gap: 13px; }
  .ic {
    display: grid; place-items: center;
    width: 46px; height: 46px;
    border-radius: 13px;
    background: var(--flow-soft);
    color: var(--green-bright);
    font-size: 24px;
  }
  h3 { font-size: 18px; font-weight: 650; }
  .sub { color: var(--text-faint); font-size: 12px; margin-top: 2px; }

  .degraded-note {
    display: flex; align-items: center; gap: 9px;
    padding: 9px 12px; border-radius: 12px;
    background: rgba(246,185,74,0.10);
    border: 1px solid rgba(246,185,74,0.25);
    color: var(--amber); font-size: 13px;
  }

  .state { display: flex; align-items: center; gap: 11px; }
  .state-ic { font-size: 20px; }
  .state-ic[data-tone="green"] { color: var(--green); }
  .state-ic[data-tone="amber"] { color: var(--amber); }
  .state-ic[data-tone="red"]   { color: var(--red); }
  .state-ic[data-tone="faint"] { color: var(--text-faint); }
  .state-label { font-size: 15px; font-weight: 600; }

  .power { display: flex; align-items: baseline; gap: 8px; margin-top: 2px; }
  .power .num {
    font-size: 54px; font-weight: 700; line-height: 1; letter-spacing: -0.02em;
    font-variant-numeric: tabular-nums;
    background: var(--flow); -webkit-background-clip: text; background-clip: text; color: transparent;
  }
  .power .unit { font-size: 18px; color: var(--text-dim); font-weight: 600; }

  .stats {
    display: grid; grid-template-columns: repeat(3, 1fr); gap: 12px;
    margin-top: 4px; padding-top: 16px; border-top: 1px solid var(--border);
  }
  .stats .k { display: flex; align-items: center; gap: 6px; color: var(--text-dim); font-size: 12px; }
  .stats .v { display: block; margin-top: 6px; font-size: 18px; font-weight: 650; font-variant-numeric: tabular-nums; }
  .stats em { font-style: normal; font-size: 12px; color: var(--text-faint); font-weight: 500; }

  @media (max-width: 720px) { .card { grid-column: span 1; } }
</style>
