<script lang="ts">
  import { noteStore } from '$lib/state/notes.svelte';
  import TipexEditor from './TipexEditor.svelte';
  import NoteHeader from './NoteHeader.svelte';

  let note = $derived(noteStore.activeNote);

  // Keyboard shortcut for save (Ctrl+S / Cmd+S)
  function handleKeydown(e: KeyboardEvent) {
    if ((e.ctrlKey || e.metaKey) && e.key === 's') {
      e.preventDefault();
      noteStore.saveActive();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="flex flex-col h-full w-full bg-background text-foreground">
  
  {#if note}
    <!-- Note Header (Title + Save Button) -->
    <NoteHeader {note} />

    <!-- Editor Area -->
    <div class="flex-1 overflow-hidden relative z-10 bg-background">
      {#key note.id}
        <TipexEditor 
          value={note.body} 
          onChange={(html) => noteStore.updateBody(html)} 
        />
      {/key}
    </div>

  {:else}
    <!-- Empty State -->
    <div class="flex h-full w-full items-center justify-center text-muted-foreground">
      <div class="text-center">
        <p class="text-xl mb-4">No note selected</p>
        <button 
          onclick={() => noteStore.create()}
          class="px-4 py-2 bg-primary text-primary-foreground rounded hover:opacity-90 transition-all"
        >
          Create New Note
        </button>
      </div>
    </div>
  {/if}

</div>
