<script lang="ts">
  import { noteStore } from '$lib/state/notes.svelte';
  import TipexEditor from './TipexEditor.svelte';

  let note = $derived(noteStore.activeNote);

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

<div class="flex flex-col h-full w-full bg-background text-foreground">
  
  {#if note}
    <!-- Title Input - Expanded with plenty of room -->
    <div class="border-b border-border px-8 py-8 z-20 relative bg-muted/30 backdrop-blur">
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

    <!-- Editor Area (Clean, No Fixed Toolbar) -->
    <div class="flex-1 overflow-hidden relative z-10">
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
