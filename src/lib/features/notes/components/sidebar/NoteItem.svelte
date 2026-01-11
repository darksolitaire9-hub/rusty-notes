<script lang="ts">
  import { FileText, Trash2 } from 'lucide-svelte';
  import type { Note } from '$lib/features/notes/notes-types';

  interface Props {
    note: Note;
    isActive: boolean;
    isCollapsed: boolean;
    onSelect: () => void;
    onDelete: (event: MouseEvent | KeyboardEvent) => void;
  }

  let { note, isActive, isCollapsed, onSelect, onDelete }: Props = $props();

  /**
   * Formats a date relative to now (Today, Yesterday, X days ago, or date)
   */
  function formatDate(date: Date): string {
    const now = new Date();
    const diff = now.getTime() - date.getTime();
    const days = Math.floor(diff / (1000 * 60 * 60 * 24));

    if (days === 0) return 'Today';
    if (days === 1) return 'Yesterday';
    if (days < 7) return `${days} days ago`;
    return date.toLocaleDateString();
  }
</script>

<div
  role="button"
  tabindex="0"
  onclick={onSelect}
  onkeydown={(e) => {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      onSelect();
    }
  }}
  class="group relative flex w-full items-center gap-3 rounded-lg p-3 text-left
         transition-all hover:bg-accent cursor-pointer
         {isActive ? 'bg-accent text-accent-foreground' : ''}"
  title={isCollapsed ? note.title || 'Untitled' : ''}
>
  <!-- Note icon with unsaved changes indicator -->
  <div class="relative shrink-0">
    <FileText class="h-4 w-4 opacity-70" />
    {#if note.isDirty}
      <span
        class="absolute -top-1 -right-1 h-2 w-2 rounded-full bg-orange-500 animate-pulse"
        title="Unsaved changes"
      ></span>
    {/if}
  </div>

  <!-- Note details - only shown when sidebar is expanded -->
  {#if !isCollapsed}
    <div class="min-w-0 flex-1 overflow-hidden">
      <!-- Title row with delete button -->
      <div class="flex items-center justify-between gap-2">
        <span class="truncate text-sm font-medium">
          {note.title || 'Untitled'}
        </span>

        <!-- Delete button - only shown on active note -->
        {#if isActive}
          <button
            onclick={onDelete}
            onkeydown={(e) => {
              if (e.key === 'Enter' || e.key === ' ') {
                e.preventDefault();
                onDelete(e);
              }
            }}
            class="opacity-0 group-hover:opacity-100 transition-opacity
                   p-1 rounded-sm hover:bg-background/20 shrink-0"
            title="Delete note"
          >
            <Trash2
              class="h-3 w-3 transition-colors text-muted-foreground hover:text-destructive"
            />
          </button>
        {/if}
      </div>

      <!-- Note body preview -->
      <div class="line-clamp-1 text-xs text-muted-foreground truncate mt-0.5">
        {note.body || 'No content'}
      </div>

      <!-- Last updated timestamp -->
      <div class="text-xs text-muted-foreground/60 mt-1">
        {formatDate(note.updatedAt)}
      </div>
    </div>
  {/if}
</div>
