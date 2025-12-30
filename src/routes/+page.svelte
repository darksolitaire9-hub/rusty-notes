<script lang="ts">
  import Sidebar from "$lib/components/notes/Sidebar.svelte"; 
  import NoteEditor from "$lib/components/editor/NoteEditor.svelte";
  
  // Note: We don't need to import noteStore here if your components 
  // are "smart" (reading the store internally).
</script>

<!-- 
  This div fills the empty space left by the NavBar in your layout.
  We use 'flex' to put Sidebar and Editor side-by-side.
-->
<div class="flex h-full w-full">
  
  <!-- Left Column: Sidebar -->
  <!-- If you updated Sidebar to use the store directly, you don't need props here.
       If you kept the "props" version, keep passing them like before. -->
  <Sidebar />

  <!-- Right Column: Editor -->
  <!-- 'flex-1' makes it fill the remaining width -->
  <main class="flex-1 h-full overflow-hidden">
    <NoteEditor />
  </main>

</div>
