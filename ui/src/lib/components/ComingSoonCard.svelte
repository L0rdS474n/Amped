<script lang="ts">
  let { icon, title, desc }: { icon: string; title: string; desc: string } = $props();
</script>

<article class="glass card">
  <div class="head">
    <span class="ic"><i class="ph-light {icon}"></i></span>
    <span class="pill"><i class="ph ph-seal"></i> Planerad</span>
  </div>
  <h3>{title}</h3>
  <p>{desc}</p>
</article>

<style>
  .card {
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 10px;
    opacity: 0.62;
    transition: opacity 0.3s var(--ease), transform 0.3s var(--ease);
  }
  .card:hover { opacity: 0.85; transform: translateY(-2px); }
  .head { display: flex; align-items: center; justify-content: space-between; }
  .ic {
    display: grid; place-items: center;
    width: 40px; height: 40px;
    border-radius: 12px;
    background: var(--surface-2);
    color: var(--teal);
    font-size: 22px;
  }
  h3 { font-size: 16px; font-weight: 600; margin-top: 4px; }
  p { color: var(--text-dim); font-size: 13px; line-height: 1.5; }
</style>
