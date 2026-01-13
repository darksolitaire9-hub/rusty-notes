<script lang="ts">
  import { Button } from '$lib/shared/components/ui/button';
  import { Plus } from '@lucide/svelte';
  import SidebarToggleButton from './SidebarToggleButton.svelte';

  interface Props {
    isCollapsed: boolean;
    isLoading: boolean;
    onToggle: () => void;
    onCreate: () => void;
  }

  let { isCollapsed, isLoading, onToggle, onCreate }: Props = $props();
</script>

<div class="p-4 border-b border-border flex items-center justify-between gap-2">
  <!-- Toggle button - always visible -->
  <SidebarToggleButton {isCollapsed} onClick={onToggle} />

  <!-- New note button - changes layout based on collapse state -->
  {#if !isCollapsed}
    <!-- Expanded: Full button with icon and text -->
    <Button
      onclick={onCreate}
      variant="default"
      size="sm"
      class="shrink-0 gap-2"
      disabled={isLoading}
      aria-label="Create new note"
    >
      <Plus class="h-4 w-4" />
      New
    </Button>
  {:else}
    <!-- Collapsed: Icon-only button -->
    <Button
      onclick={onCreate}
      variant="ghost"
      size="icon"
      class="h-8 w-8 shrink-0"
      disabled={isLoading}
      aria-label="Create new note"
    >
      <Plus class="h-4 w-4" />
    </Button>
  {/if}
</div>
