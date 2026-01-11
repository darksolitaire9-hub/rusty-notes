// src/lib/features/notes/store/notes-persistence.svelte.ts

import type { Note } from '../notes-types';

/**
 * Handles auto-save and manual save operations
 * Debounces auto-save to prevent excessive writes
 */
export class NotesPersistence {
  autoSaveTimer = $state<ReturnType<typeof setTimeout> | null>(null);
  readonly AUTO_SAVE_DELAY = 2 * 60 * 1000; // 2 minutes

  /**
   * Schedule an auto-save with debouncing
   * Cancels previous timer and starts a new one
   */
  scheduleAutoSave(note: Note, saveFn: () => Promise<void>) {
    if (this.autoSaveTimer) {
      clearTimeout(this.autoSaveTimer);
    }

    this.autoSaveTimer = setTimeout(async () => {
      try {
        await saveFn();
        note.isDirty = false;
        console.log(`Auto-save "${note.title || 'Untitled'}"`);
      } catch (error) {
        console.error('Auto-save failed:', error);
        // Keep isDirty flag on error
      } finally {
        this.autoSaveTimer = null;
      }
    }, this.AUTO_SAVE_DELAY);
  }

  /**
   * Immediately save a note
   * Cancels any pending auto-save
   */
  async manualSave(note: Note, saveFn: () => Promise<void>) {
    // Cancel pending auto-save
    this.cancel();

    try {
      await saveFn();
      note.isDirty = false;
      console.log(`Manual save "${note.title || 'Untitled'}"`);
    } catch (error) {
      console.error('Save failed:', error);
      throw error; // Re-throw to allow service layer to handle
    } finally {
      this.autoSaveTimer = null;
    }
  }

  /**
   * Cancel any pending auto-save
   */
  cancel() {
    if (this.autoSaveTimer) {
      clearTimeout(this.autoSaveTimer);
      this.autoSaveTimer = null;
    }
  }

  /**
   * Check if there's a pending auto-save
   */
  get hasPendingSave(): boolean {
    return this.autoSaveTimer !== null;
  }
}

export const notesPersistence = new NotesPersistence();
