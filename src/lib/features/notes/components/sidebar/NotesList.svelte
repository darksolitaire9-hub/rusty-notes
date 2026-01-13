<script lang="ts">
  import { FileText, CircleAlert } from '@lucide/svelte';
  import type { Note } from '$lib/features/notes/notes-types';
  import NoteItem from './NoteItem.svelte';

  interface Props {
    notes: Note[];
    activeNoteId: string | null;
    isCollapsed: boolean;
    isLoading: boolean;
    error: string | null;
    onSelectNote: (noteId: string) => void;
    onDeleteNote: (noteId: string, event: MouseEvent | KeyboardEvent) => void;
    onClearError: () => void;
  }

  let { 
    notes, 
    activeNoteId, 
    isCollapsed, 
    isLoading, 
    error, 
    onSelectNote, 
    onDeleteNote,
    onClearError
  }: Props = $props();
</script>

<!-- Error Banner - shown at top when there's an error -->
{#if error && !isCollapsed}
  <div class="mx-2 mt-2 p-2 bg-destructive/10 border border-destructive/20 rounded-md flex items-start gap-2">
    <CircleAlert class="h-4 w-4 text-destructive shrink-0 mt-0.5" />
    <div class="flex-1 min-w-0">
      <p class="text-xs text-destructive">{error}</p>
    </div>
    <button
      onclick={onClearError}
      class="text-destructive hover:text-destructive/80 shrink-0"
      aria-label="Dismiss error"
    >
      ✕
    </button>
  </div>
{/if}

<!-- Loading State -->
{#if isLoading}
  <div class="flex-1 flex items-center justify-center p-4">
    <div class="text-center space-y-2">
      <!-- Spinner -->
      <div class="animate-spin h-8 w-8 border-2 border-primary border-t-transparent rounded-full mx-auto"></div>
      {#if !isCollapsed}
        <p class="text-sm text-muted-foreground">Loading notes...</p>
      {/if}
    </div>
  </div>

<!-- Empty State - no notes yet -->
{:else if notes.length === 0}
  <div class="flex-1 flex items-center justify-center p-4">
    {#if !isCollapsed}
      <div class="text-center space-y-2">
        <FileText class="h-12 w-12 text-muted-foreground/50 mx-auto" />
        <p class="text-sm text-muted-foreground">No notes yet</p>
        <p class="text-xs text-muted-foreground/70">Click "New" to get started</p>
      </div>
    {:else}
      <!-- Collapsed empty state - just icon -->
      <FileText class="h-8 w-8 text-muted-foreground/50" />
    {/if}
  </div>

<!-- List of Notes -->
{:else}
  <div class="flex-1 overflow-y-auto p-2 space-y-1">
    {#each notes as note (note.id)}
      <NoteItem
        {note}
        isActive={activeNoteId === note.id}
        {isCollapsed}
        onSelect={() => onSelectNote(note.id)}
        onDelete={(e) => onDeleteNote(note.id, e)}
      />
    {/each}
  </div>
{/if}
