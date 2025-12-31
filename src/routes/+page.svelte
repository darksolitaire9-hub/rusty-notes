<script lang="ts">
  import { onMount } from 'svelte';
  import { noteStore } from '$lib/state/notes.svelte';
  import Sidebar from "$lib/components/notes/Sidebar.svelte"; 
  import NoteEditor from "$lib/components/editor/NoteEditor.svelte";
  
  // Load notes from disk when app starts
  onMount(async () => {
    console.log('Loading notes from disk...');
    await noteStore.loadAllNotes();
    
    // If no notes exist, create a first one
    if (noteStore.notes.length === 0) {
      console.log('No notes found, creating first note');
      await noteStore.create();
    } else {
      console.log(`Loaded ${noteStore.notes.length} notes`);
    }
  });
</script>

<!-- 
  This div fills the space. Sidebar and Editor side-by-side.
-->
<div class="flex h-full w-full">
  
  <!-- Left Column: Sidebar -->
  <Sidebar />

  <!-- Right Column: Editor -->
  <main class="flex-1 h-full overflow-hidden">
    <NoteEditor />
  </main>

</div>
