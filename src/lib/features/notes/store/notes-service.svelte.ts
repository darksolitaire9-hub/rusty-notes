// src/lib/features/notes/store/notes-service.svelte.ts

import { notesLocalState } from './notes-local-state.svelte';
import { noteAPIAdapter } from './notes-api-adapter';

/**
 * Main orchestrator for notes feature
 * Coordinates between local state, API adapter, and persistence
 * Single entry point for all note operations
 */
class NoteService {
  // ========================================================================
  // STATE ACCESSORS (delegates to local state)
  // ========================================================================

  get notes() {
    return notesLocalState.notes;
  }

  get activeNote() {
    return notesLocalState.activeNote;
  }

  get isLoading() {
    return notesLocalState.isLoading;
  }

  get error() {
    return notesLocalState.error;
  }

  get shouldFocusTitle() {
    return notesLocalState.shouldFocusTitle;
  }

  // ========================================================================
  // FOCUS CONTROL
  // ========================================================================

  requestTitleFocus() {
    notesLocalState.shouldFocusTitle = true;
  }

  clearTitleFocus() {
    notesLocalState.shouldFocusTitle = false;
  }

  // ========================================================================
  // INITIALIZATION
  // ========================================================================

  async load() {
    notesLocalState.isLoading = true;
    notesLocalState.error = null;

    try {
      notesLocalState.notes = await noteAPIAdapter.loadAll();

      // Keep current selection if it still exists
      if (notesLocalState.selectedId) {
        const exists = notesLocalState.notes.find((n) => n.id === notesLocalState.selectedId);
        if (!exists) {
          notesLocalState.clearSelection();
        }
      }
    } catch (err) {
      notesLocalState.error = err instanceof Error ? err.message : 'Failed to load notes';
      console.error('Failed to load notes:', err);
    } finally {
      notesLocalState.isLoading = false;
    }
  }

  // ========================================================================
  // CRUD OPERATIONS
  // ========================================================================

  async create() {
    notesLocalState.isLoading = true;
    notesLocalState.error = null;

    try {
      const newNote = await noteAPIAdapter.createEmpty();
      notesLocalState.notes = [newNote, ...notesLocalState.notes];
      notesLocalState.selectedId = newNote.id;
      this.requestTitleFocus(); 
    } catch (err) {
      notesLocalState.error = err instanceof Error ? err.message : 'Failed to create note';
      console.error('Failed to create note:', err);
    } finally {
      notesLocalState.isLoading = false;
    }
  }

  async update(id: string, title: string, body: string) {
    try {
      const note = notesLocalState.notes.find((n) => n.id === id);
      if (!note) throw new Error('Note not found');

      // Update local state first
      note.title = title;
      note.body = body;
      note.isDirty = false;
      note.updatedAt = new Date();

      // Persist to API
      await noteAPIAdapter.update(note);

      // Update activeNote if it's the current one
      if (notesLocalState.selectedId === id) {
        notesLocalState.clearSelection();
        notesLocalState.select(id);
      }
    } catch (err) {
      notesLocalState.error = err instanceof Error ? err.message : 'Failed to update note';
      console.error('Failed to update note:', err);
      throw err;
    }
  }

  /**
   * Delete a note - Rust backend checks settings.delete_behavior
   * Backend will either soft-delete (trash) or permanently delete
   */
  async delete(id: string) {
    notesLocalState.isLoading = true;
    notesLocalState.error = null;

    try {
      // Rust DeleteService checks settings and handles appropriately
      await noteAPIAdapter.delete(id);
      
      // Remove from UI
      notesLocalState.removeFromUI(id);
    } catch (err) {
      notesLocalState.error = err instanceof Error ? err.message : 'Failed to delete note';
      console.error('Failed to delete note:', err);
    } finally {
      notesLocalState.isLoading = false;
    }
  }

  /**
   * Restore a soft-deleted note from trash
   */
  async restore(id: string) {
    notesLocalState.isLoading = true;
    notesLocalState.error = null;

    try {
      await noteAPIAdapter.restore(id);
      await this.load(); // Refresh list
    } catch (err) {
      notesLocalState.error = err instanceof Error ? err.message : 'Failed to restore note';
      console.error('Failed to restore note:', err);
    } finally {
      notesLocalState.isLoading = false;
    }
  }

  /**
   * List soft-deleted notes (for trash view)
   */
  async loadDeleted() {
    notesLocalState.isLoading = true;
    notesLocalState.error = null;

    try {
      notesLocalState.notes = await noteAPIAdapter.listDeleted();
    } catch (err) {
      notesLocalState.error = err instanceof Error ? err.message : 'Failed to load deleted notes';
      console.error('Failed to load deleted notes:', err);
    } finally {
      notesLocalState.isLoading = false;
    }
  }

  // ========================================================================
  // SELECTION
  // ========================================================================

  select(id: string) {
    notesLocalState.select(id);
  }

  // ========================================================================
  // SEARCH
  // ========================================================================

  async search(query: string) {
    if (!query.trim()) {
      await this.load();
      return;
    }

    notesLocalState.isLoading = true;
    notesLocalState.error = null;

    try {
      notesLocalState.notes = await noteAPIAdapter.search(query);
    } catch (err) {
      notesLocalState.error = err instanceof Error ? err.message : 'Failed to search notes';
      console.error('Failed to search notes:', err);
    } finally {
      notesLocalState.isLoading = false;
    }
  }

  // ========================================================================
  // ACTIVE NOTE MUTATIONS
  // ========================================================================

  /**
   * Update title of active note (local state only, marks as dirty)
   */
  updateTitle(newTitle: string) {
    if (!notesLocalState.activeNote) return;

    const note = notesLocalState.notes.find((n) => n.id === notesLocalState.selectedId);
    if (note) {
      note.title = newTitle;
      note.isDirty = true;
    }

    // Force activeNote reactivity by reassigning
    notesLocalState.select(notesLocalState.selectedId!);
  }

  /**
   * Update body of active note (local state only, marks as dirty)
   */
  updateBody(newBody: string) {
    if (!notesLocalState.activeNote) return;

    const note = notesLocalState.notes.find((n) => n.id === notesLocalState.selectedId);
    if (note) {
      note.body = newBody;
      note.isDirty = true;
    }

    // Force activeNote reactivity by reassigning
    notesLocalState.select(notesLocalState.selectedId!);
  }

  /**
   * Save the currently active note to API
   */
  async saveActive() {
    if (!notesLocalState.activeNote) return;

    const { id, title, body } = notesLocalState.activeNote;
    await this.update(id, title, body);
  }

  // ========================================================================
  // DIRTY STATE
  // ========================================================================

  markDirty(id: string) {
    const note = notesLocalState.notes.find((n) => n.id === id);
    if (note) {
      note.isDirty = true;
    }
  }

  // ========================================================================
  // ERROR HANDLING
  // ========================================================================

  clearError() {
    notesLocalState.error = null;
  }

  // ========================================================================
  // CLEANUP
  // ========================================================================

  destroy() {
    // Cleanup if needed
  }
}

export const noteService = new NoteService();
