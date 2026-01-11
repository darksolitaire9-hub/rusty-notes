<script lang="ts">
  import { noteService } from "$lib/features/notes/notes-index";
  import NotesSidebar from "$lib/features/notes/components/sidebar/NotesSidebar.svelte";
  import NoteEditor from "$lib/features/notes/components/editor/NoteEditor.svelte";
  import { handleGlobalShortcut } from "$lib/shared/shortcuts/handlers";
  import { browser } from "$app/environment";

  // Load notes when the component mounts
  $effect(() => {
    if (!browser) return;
    noteService.load();
  });
</script>

<svelte:window onkeydown={handleGlobalShortcut} />

<div class="flex h-full w-full">
  <NotesSidebar />
  <main class="flex-1 h-full overflow-hidden">
    <NoteEditor />
  </main>
</div>