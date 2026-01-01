<script lang="ts">
  import { onMount } from 'svelte';
  import { noteStore } from '$lib/state/notes.svelte';
  import Sidebar from '$lib/components/notes/Sidebar.svelte'; 
  import NoteEditor from '$lib/components/editor/NoteEditor.svelte';

  onMount(async () => {
    console.log('Loading notes from disk...');
    await noteStore.loadAllNotes();
    
    if (noteStore.notes.length === 0) {
      console.log('No notes found, creating first note');
      await noteStore.create();
    } else {
      console.log(`Loaded ${noteStore.notes.length} notes`);
    }
  });
</script>

<div class="flex h-full w-full">
  <Sidebar />
  <main class="flex-1 h-full overflow-hidden">
    <NoteEditor />
  </main>
</div>
