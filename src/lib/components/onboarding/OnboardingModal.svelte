<script lang="ts">
  import { modalOpen } from '$lib/state/ui';

  let { children, open = true, onComplete } = $props();

  $effect(() => {
    modalOpen.set(open);
  });
</script>

{#if open}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/40 backdrop-blur-sm">
    <div class="w-[480px] max-w-[90vw] rounded-xl bg-card text-card-foreground shadow-2xl border border-border p-6">
      {@render children({ onComplete })}
    </div>
  </div>
{/if}
