<script lang="ts">
  import { noteStore } from '$lib/state/notes.svelte';
  import { Button } from '$lib/components/ui/button';
  import { Save } from 'lucide-svelte';
  import type { Note } from '$lib/state/notes.svelte';

  let { note } = $props<{ note: Note }>();

  // Auto-focus title when creating new note
  function focusOnMount(node: HTMLInputElement) {
    if (noteStore.shouldFocusTitle) {
      setTimeout(() => {
        node.focus();
        noteStore.shouldFocusTitle = false;
      }, 50);
    }
  }
</script>

<div class="border-b border-border px-8 py-8 z-20 relative bg-muted/30 backdrop-blur">
  <div class="flex items-center gap-4">
    <!-- Title Input -->
    <div class="flex-1">
      {#key note.id}
        <input 
          use:focusOnMount
          type="text" 
          value={note.title}
          oninput={(e) => noteStore.updateTitle(e.currentTarget.value)}
          placeholder="Untitled"
          class="w-full bg-transparent text-4xl font-extrabold text-foreground focus:outline-none placeholder:text-muted-foreground/40 tracking-tight leading-relaxed py-2"
        />
      {/key}
    </div>

    <!-- Save Button (only shows when dirty) -->
    {#if note.isDirty}
      <Button 
        onclick={() => noteStore.saveActive()}
        variant="default"
        size="sm"
        class="shrink-0 gap-2"
        title="Save changes (Ctrl+S)"
      >
        <Save class="h-4 w-4" />
        Save
      </Button>
    {/if}
  </div>
</div>
