import { invoke } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';
import { writeTextFile } from '@tauri-apps/plugin-fs';

export type Note = {
  id: string;
  title: string;
  body: string;
  updatedAt: Date;
  isDirty: boolean;
  filePath?: string;
};

// Type returned from Rust (Note with timestamps as numbers)
type BackendNote = {
  id: string;
  title: string;
  body: string;
  created_at: number;
  updated_at: number;
};

function fromBackend(n: BackendNote): Note {
  return {
    id: n.id,
    title: n.title,
    body: n.body,
    updatedAt: new Date(n.updated_at * 1000),
    isDirty: false
  };
}

class NoteState {
  notes = $state<Note[]>([]);
  selectedId = $state<string | null>(null);
  shouldFocusTitle = $state(false);

  private autoSaveTimer: number | null = null;
  private readonly AUTO_SAVE_DELAY = 120000; // 2 minutes

  activeNote = $derived(
    this.notes.find((n) => n.id === this.selectedId) || null
  );

  // 1. Load all notes from SQLite on startup
  async loadAllNotes() {
    try {
      const backendNotes = await invoke<BackendNote[]>('list_notes');

      this.notes = backendNotes.map(fromBackend);

      if (this.notes.length > 0) {
        this.selectedId = this.notes[0].id;
      }

      console.log(`Loaded ${this.notes.length} notes from database`);
    } catch (error) {
      console.error('Failed to load notes:', error);
    }
  }

  // 2. Select a note
  select(id: string) {
    this.selectedId = id;
    this.shouldFocusTitle = false;
  }

  // 3. Create a new note in DB immediately
  async create() {
    try {
      const backendNote = await invoke<BackendNote>('create_note', {
        title: '',
        body: ''
      });

      const newNote = fromBackend(backendNote);
      newNote.isDirty = false; // just saved

      this.notes = [newNote, ...this.notes];
      this.selectedId = newNote.id;
      this.shouldFocusTitle = true;

      console.log('Created new note in database', newNote.id);
    } catch (error) {
      console.error('Failed to create note:', error);
    }
  }

  // 4. Delete (DB + UI)
  async delete(id: string) {
    try {
      await invoke('delete_note', { id });

      this.notes = this.notes.filter((n) => n.id !== id);
      if (this.selectedId === id) {
        this.selectedId = this.notes.length > 0 ? this.notes[0].id : null;
      }

      console.log(`Deleted note ${id}`);
    } catch (error) {
      console.error('Failed to delete note:', error);
    }
  }

  updateTitle(newTitle: string) {
    if (this.activeNote) {
      this.activeNote.title = newTitle;
      this.activeNote.updatedAt = new Date();
      this.activeNote.isDirty = true;

      this.scheduleAutoSave();
    }
  }

  updateBody(newBody: string) {
    if (this.activeNote) {
      this.activeNote.body = newBody;
      this.activeNote.updatedAt = new Date();
      this.activeNote.isDirty = true;

      this.scheduleAutoSave();
    }
  }

  // Auto-save to DB after inactivity
  private scheduleAutoSave() {
    if (this.autoSaveTimer) {
      clearTimeout(this.autoSaveTimer);
    }

    this.autoSaveTimer = setTimeout(() => {
      if (this.activeNote) {
        this.autoSaveToBackend(this.activeNote);
      }
    }, this.AUTO_SAVE_DELAY) as unknown as number;
  }

  // Background save to SQLite
  private async autoSaveToBackend(note: Note) {
    try {
      await invoke<BackendNote>('update_note', {
        id: note.id,
        title: note.title,
        body: note.body
      });

      console.log(
        `[Auto-save] Saved note "${note.title || 'Untitled'}" to database`
      );
      // Keep isDirty to show red dot until manual save if you want
    } catch (error) {
      console.error('Auto-save failed:', error);
    }
  }

  // Manual save: save to DB and clear dirty flag
  async save(id: string) {
    const note = this.notes.find((n) => n.id === id);
    if (!note) return;

    try {
      await invoke<BackendNote>('update_note', {
        id: note.id,
        title: note.title,
        body: note.body
      });

      note.isDirty = false;
      console.log(
        `[Manual save] User saved note "${note.title || 'Untitled'}" to database`
      );
    } catch (error) {
      console.error('Save failed:', error);
    }
  }

  // Export active note as HTML file (unchanged pattern, but DB is source of truth)
  async saveActive() {
    if (!this.activeNote) return;

    const note = this.activeNote;

    if (!note.filePath) {
      const filePath = await save({
        defaultPath: note.title || 'Untitled.html',
        filters: [
          {
            name: 'HTML',
            extensions: ['html']
          }
        ]
      });

      if (!filePath) return;

      note.filePath = filePath;
    }

    try {
      await writeTextFile(note.filePath, note.body);

      await invoke<BackendNote>('update_note', {
        id: note.id,
        title: note.title,
        body: note.body
      });

      note.isDirty = false;
      console.log(`Saved note to ${note.filePath}`);
    } catch (error) {
      console.error('Save failed:', error);
    }
  }
}

export const noteStore = new NoteState();
