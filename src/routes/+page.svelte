<script lang="ts">
  import Sidebar from "$lib/components/notes/Sidebar.svelte";
  import { noteStore } from "$lib/state/notes.svelte"; // Import the brain
</script>

<Sidebar 
  notes={noteStore.notes} 
  selectedId={noteStore.selectedId} 
  onCreate={() => noteStore.create()} 
  onSelect={(id) => noteStore.select(id)} 
  onDelete={(id) => noteStore.delete(id)} 
/>

