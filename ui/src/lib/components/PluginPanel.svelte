<script lang="ts">
  import type { PluginInfo } from '../contract';
  let { plugins, source = 'mock' }: { plugins: PluginInfo[]; source?: 'tauri' | 'mock' } = $props();
</script>

<section class="glass panel">
  <div class="head">
    <h2><i class="ph-light ph-puzzle-piece"></i> Installerade plugins</h2>
    <span class="src" title={source === 'tauri' ? 'Läst från riktiga manifest via Tauri' : 'Mockdata (browser-preview)'}>
      <i class="ph {source === 'tauri' ? 'ph-hard-drives' : 'ph-flask'}"></i>
      {source === 'tauri' ? 'manifest' : 'mock'}
    </span>
    <span class="pill">{plugins.length}</span>
  </div>

  {#if plugins.length === 0}
    <p class="empty">Inga plugins hittades.</p>
  {/if}

  <ul>
    {#each plugins as p (p.id)}
      <li>
        <span class="ic"><i class="ph-fill ph-charging-station"></i></span>
        <div class="meta">
          <div class="line1">
            <strong>{p.name}</strong>
            <span class="ver">v{p.version}</span>
            <span class="by">· {p.author}</span>
          </div>
          <p class="desc">{p.description}</p>
          <div class="perms">
            <span class="perm net"><i class="ph ph-globe-simple"></i> {p.permissions.network.join(', ') || 'inget nät'}</span>
            <span class="perm sec"><i class="ph ph-key"></i> {p.permissions.secrets.length} hemligheter (host-ägda)</span>
            <span class="perm fs"><i class="ph ph-folder-simple"></i> {p.permissions.filesystem.length === 0 ? 'ingen filåtkomst' : `${p.permissions.filesystem.length} sökvägar`}</span>
          </div>
        </div>
        <span class="pill is-mock" title="Körs som sandboxad WASM-komponent vid M2">Read-only</span>
      </li>
    {/each}
  </ul>
</section>

<style>
  .panel { padding: 20px; margin-top: var(--gap); }
  .head { display: flex; align-items: center; gap: 10px; margin-bottom: 14px; }
  h2 { font-size: 15px; font-weight: 600; color: var(--text-dim); display: flex; align-items: center; gap: 8px; margin-right: auto; }
  .src {
    display: inline-flex; align-items: center; gap: 5px;
    font-size: 11px; color: var(--text-faint);
    padding: 3px 9px; border-radius: 999px;
    background: var(--surface); border: 1px solid var(--border);
  }
  .empty { color: var(--text-faint); font-size: 13px; }
  ul { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 10px; }
  li {
    display: flex; align-items: flex-start; gap: 14px;
    padding: 14px; border-radius: var(--radius-sm);
    background: var(--surface); border: 1px solid var(--border);
  }
  .ic {
    display: grid; place-items: center; flex: none;
    width: 40px; height: 40px; border-radius: 11px;
    background: var(--flow-soft); color: var(--green-bright); font-size: 21px;
  }
  .meta { flex: 1; min-width: 0; }
  .line1 { display: flex; align-items: baseline; gap: 7px; flex-wrap: wrap; }
  .line1 strong { font-size: 15px; font-weight: 650; }
  .ver { font-size: 11px; color: var(--teal); font-variant-numeric: tabular-nums; }
  .by { font-size: 12px; color: var(--text-faint); }
  .desc { color: var(--text-dim); font-size: 13px; margin: 5px 0 9px; line-height: 1.45; }
  .perms { display: flex; flex-wrap: wrap; gap: 7px; }
  .perm {
    display: inline-flex; align-items: center; gap: 5px;
    font-size: 11px; padding: 3px 9px; border-radius: 999px;
    background: var(--surface-2); border: 1px solid var(--border); color: var(--text-dim);
  }
  .perm.sec { color: var(--amber); border-color: rgba(246,185,74,0.22); }
</style>
