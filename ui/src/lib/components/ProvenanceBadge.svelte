<script lang="ts">
  import type { Provenance } from '../contract';
  import { provenanceLabel } from '../contract';

  let { p }: { p: Provenance } = $props();

  const cls = $derived(
    p.kind === 'mock' ? 'is-mock' : p.kind === 'degraded' ? 'is-warn' : 'is-on',
  );
  const icon = $derived(
    p.kind === 'mock' ? 'ph-flask'
      : p.kind === 'degraded' ? 'ph-warning'
      : p.kind === 'cached' ? 'ph-clock-countdown'
      : 'ph-broadcast',
  );
</script>

<span class="pill {cls}" title={p.kind === 'degraded' ? p.reason : ''}>
  <i class="ph {icon}"></i>
  {provenanceLabel(p)}
</span>
