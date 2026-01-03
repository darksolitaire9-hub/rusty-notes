<!-- src/lib/components/onboarding/StorageLocationField.svelte -->
<script lang="ts">
  import { open } from '@tauri-apps/plugin-dialog';

  let {
    value = null as string | null,
    disabled = false,
    onChange
  } = $props<{
    value?: string | null;
    disabled?: boolean;
    onChange?: (folder: string | null) => void;
  }>();

  let folderError = $state<string | null>(null);

  async function pickFolder() {
    try {
      const selected = await open({
        directory: true,
        multiple: false
      });

      if (typeof selected === 'string') {
        value = selected;
        folderError = null;
        onChange?.(selected);
      }
    } catch (e) {
      console.error(e);
      folderError = 'Failed to pick folder. Please try again.';
    }
  }
</script>

<section class="space-y-2">
  <h2 class="text-sm font-medium text-foreground">Storage location</h2>
  <p class="text-xs text-muted-foreground">
    Choose where to save your encrypted notes on this device.
  </p>

  <div class="flex items-center gap-2">
    <button
      type="button"
      class="rounded-md border border-border bg-card px-3 py-1.5 text-xs font-medium text-foreground shadow-sm hover:bg-accent disabled:opacity-60 disabled:cursor-not-allowed"
      onclick={pickFolder}
      disabled={disabled}
    >
      Choose folder…
    </button>

    {#if value}
      <span class="truncate text-xs text-muted-foreground max-w-[260px]">
        {value}
      </span>
    {:else}
      <span class="text-xs text-destructive">No folder selected</span>
    {/if}
  </div>

  {#if folderError}
    <p class="text-xs text-destructive">{folderError}</p>
  {/if}
</section>
