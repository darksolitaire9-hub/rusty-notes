<script lang="ts">
  import { noteService } from '$lib/features/notes/store/notes-service.svelte';
  import { onDestroy } from 'svelte';
  import SidebarHeader from './SidebarHeader.svelte';
  import NotesList from './NotesList.svelte';

  // ========================================================================
  // STATE MANAGEMENT (Svelte 5 Runes)
  // ========================================================================
  // All state is local and reactive using $state()
  // No external stores needed - state flows down as props
  
  let isCollapsed = $state(false);

  // ========================================================================
  // LIFECYCLE
  // ========================================================================
  
  onDestroy(() => {
    noteService.destroy();
  });

  // ========================================================================
  // EVENT HANDLERS
  // ========================================================================
  
  /**
   * Toggles sidebar collapsed/expanded state
   */
  function handleToggle() {
    isCollapsed = !isCollapsed;
  }

  /**
   * Creates a new note via noteService
   */
  async function handleCreate() {
    await noteService.create();
  }

  /**
   * Selects a note
   */
  function handleNoteSelect(noteId: string) {
    noteService.select(noteId);
  }

  /**
   * Deletes note immediately - Rust backend respects settings:
   * - MoveToTrash → soft delete (recoverable from trash)
   * - Permanent → permanent delete (not recoverable)
   */
  async function handleDelete(noteId: string, event: MouseEvent | KeyboardEvent) {
    event.stopPropagation();
    await noteService.delete(noteId);
  }

  /**
   * Clears error message from noteService
   */
  function handleClearError() {
    noteService.clearError();
  }
</script>

<!-- ========================================================================
     SIDEBAR LAYOUT
     - Flex column for vertical stacking
     - Conditional width classes based on collapse state
     - Smooth transitions via CSS
     ======================================================================== -->
<aside
  class="flex-none border-r border-border bg-muted/10 flex flex-col h-full transition-all duration-300 ease-in-out"
  class:w-64={!isCollapsed}
  class:w-16={isCollapsed}
>
  <!-- Header: Toggle + New button -->
  <SidebarHeader
    {isCollapsed}
    isLoading={noteService.isLoading}
    onToggle={handleToggle}
    onCreate={handleCreate}
  />

  <!-- Main content: Error banner + loading/empty/list states -->
  <NotesList
    notes={noteService.notes}
    activeNoteId={noteService.activeNote?.id ?? null}
    {isCollapsed}
    isLoading={noteService.isLoading}
    error={noteService.error}
    onSelectNote={handleNoteSelect}
    onDeleteNote={handleDelete}
    onClearError={handleClearError}
  />

  <!-- Footer: Note count -->
  {#if !noteService.isLoading}
    <div class="p-3 border-t border-border text-xs text-muted-foreground text-center">
      {noteService.notes.length} {noteService.notes.length === 1 ? 'note' : 'notes'}
    </div>
  {/if}
</aside>
