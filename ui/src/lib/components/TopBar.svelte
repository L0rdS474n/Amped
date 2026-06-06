<script lang="ts">
  import { onMount } from 'svelte';

  let { demo = true }: { demo?: boolean } = $props();
  let time = $state('--:--');

  function tick() {
    time = new Date().toLocaleTimeString('sv-SE', { hour: '2-digit', minute: '2-digit' });
  }

  onMount(() => {
    tick();
    const id = setInterval(tick, 10_000);
    return () => clearInterval(id);
  });
</script>

<header>
  <div class="brand">
    <span class="logo"><i class="ph-fill ph-lightning"></i></span>
    <div class="word">
      <h1>Amped</h1>
      <p>Energidashboard</p>
    </div>
  </div>
  <div class="right">
    {#if demo}
      <span class="pill is-mock"><i class="ph ph-flask"></i> Demodata</span>
    {/if}
    <span class="pill"><i class="ph-light ph-clock"></i> {time}</span>
  </div>
</header>

<style>
  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    padding: 4px 2px 22px;
  }
  .brand { display: flex; align-items: center; gap: 14px; }
  .logo {
    display: grid;
    place-items: center;
    width: 46px; height: 46px;
    border-radius: 14px;
    background: var(--flow);
    color: #06120c;
    font-size: 26px;
    box-shadow: 0 10px 28px -8px rgba(46, 226, 122, 0.6);
  }
  h1 {
    font-size: 26px;
    font-weight: 700;
    letter-spacing: -0.02em;
    line-height: 1;
    background: var(--flow);
    -webkit-background-clip: text;
    background-clip: text;
    color: transparent;
  }
  .word p { color: var(--text-faint); font-size: 12.5px; margin-top: 3px; letter-spacing: 0.04em; }
  .right { display: flex; align-items: center; gap: 10px; }
  .right .pill i { font-size: 14px; }
</style>
