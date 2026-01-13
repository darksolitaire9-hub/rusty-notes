<script lang="ts">
  import { Button } from '$lib/shared/components/ui/button';
  import { PanelLeftOpen, PanelLeftClose } from '@lucide/svelte';

  interface Props {
    isCollapsed: boolean;
    onClick: () => void;
  }

  let { isCollapsed, onClick }: Props = $props();
</script>

<Button
  variant="ghost"
  size="icon"
  class="h-8 w-8 shrink-0"
  onclick={onClick}
  aria-label={isCollapsed ? 'Expand sidebar' : 'Collapse sidebar'}
>
  {#if isCollapsed}
    <PanelLeftOpen class="h-4 w-4" />
  {:else}
    <PanelLeftClose class="h-4 w-4" />
  {/if}
</Button>
