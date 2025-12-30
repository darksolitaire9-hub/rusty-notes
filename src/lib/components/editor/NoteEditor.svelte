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

<div class="flex flex-col h-full w-full bg-slate-950 text-slate-200">
  
  {#if note}
    <!-- Title Input -->
    <div class="border-b border-slate-800 px-8 py-6 z-20 relative bg-slate-950">
      {#key note.id}
        <input 
          use:focusOnMount
          type="text" 
          value={note.title}
          oninput={(e) => noteStore.updateTitle(e.currentTarget.value)}
          placeholder="Untitled"
          class="w-full bg-transparent text-4xl font-extrabold text-white focus:outline-none placeholder-slate-700 tracking-tight"
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
    <div class="flex h-full w-full items-center justify-center text-slate-500">
      <div class="text-center">
        <p class="text-xl mb-4">No note selected</p>
        <button 
          onclick={() => noteStore.create()}
          class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-500 transition-colors"
        >
          Create New Note
        </button>
      </div>
    </div>
  {/if}

</div>
