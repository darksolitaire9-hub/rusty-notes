// src/lib/features/notes/store/notes-local-state.svelte.ts

import type { Note } from '../notes-types';

/**
 * Local UI state management using Svelte 5 runes
 * Handles note selection, focus state, and UI-only flags
 */
export class NotesLocalState {
  notes = $state<Note[]>([]);
  selectedId = $state<string | null>(null);
  shouldFocusTitle = $state(false);
  isLoading = $state(false);
  error = $state<string | null>(null);

  get activeNote(): Note | null {
    return this.notes.find((n) => n.id === this.selectedId) || null;
  }

  get hasNotes(): boolean {
    return this.notes.length > 0;
  }

  select(id: string) {
    this.selectedId = id;
    this.shouldFocusTitle = false;
  }

  removeFromUI(id: string) {
    this.notes = this.notes.filter((n) => n.id !== id);
    // Auto-select next note if the deleted note was active
    if (this.selectedId === id) {
      this.selectedId = this.hasNotes ? this.notes[0].id : null;
    }
  }

  clearSelection() {
    this.selectedId = null;
    this.shouldFocusTitle = false;
  }

  sortByUpdatedAt() {
    this.notes = [...this.notes].sort(
      (a, b) => b.updatedAt.getTime() - a.updatedAt.getTime()
    );
  }
}

export const notesLocalState = new NotesLocalState();
