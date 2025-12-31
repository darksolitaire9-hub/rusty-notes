// Import Tauri's invoke function to call Rust commands
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

class NoteState {
    // 1. The Data
    notes = $state<Note[]>([]);
    selectedId = $state<string | null>(null);
    shouldFocusTitle = $state(false);
    
    // Auto-save timer
    private autoSaveTimer: number | null = null;
    private readonly AUTO_SAVE_DELAY = 120000; // 2 minutes

    // 2. Computed Values
    activeNote = $derived(
        this.notes.find((n) => n.id === this.selectedId) || null
    );

    // 3. Load all notes from disk on startup
    async loadAllNotes() {
        try {
            // Ask Rust for list of note IDs
            const ids = await invoke<string[]>('list_notes');
            
            // Load each note
            for (const id of ids) {
                const [title, body] = await invoke<[string, string]>('load_note', { id });
                
                this.notes.push({
                    id,
                    title,
                    body,
                    updatedAt: new Date(),
                    isDirty: false
                });
            }
            
            console.log(`Loaded ${ids.length} notes from disk`);
        } catch (error) {
            console.error('Failed to load notes:', error);
        }
    }

    // 4. Actions
    select(id: string) {
        this.selectedId = id;
        this.shouldFocusTitle = false;
    }

    async create() {
        const newNote: Note = {
            id: crypto.randomUUID(),
            title: '',
            body: '',
            updatedAt: new Date(),
            isDirty: true  // New notes need to be saved
        };
        this.notes = [newNote, ...this.notes];
        this.selectedId = newNote.id;
        this.shouldFocusTitle = true;
    }

    async delete(id: string) {
        try {
            // Delete from disk first
            await invoke('delete_note', { id });
            
            // Then remove from UI
            this.notes = this.notes.filter(n => n.id !== id);
            if (this.selectedId === id) {
                this.selectedId = null;
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

    // Auto-save: Saves data but keeps red dot
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

    // Background save (silent, no UI change)
    private async autoSaveToBackend(note: Note) {
        try {
            // Call Rust to save to disk
            await invoke('save_note', {
                id: note.id,
                title: note.title,
                body: note.body
            });
            
            console.log(`[Auto-save] Saved note "${note.title || 'Untitled'}" to disk`);
        } catch (error) {
            console.error('Auto-save failed:', error);
        }
    }

    // Manual save: Clears red dot (user acknowledgment)
    async save(id: string) {
        const note = this.notes.find(n => n.id === id);
        if (note) {
            try {
                // Save to disk
                await invoke('save_note', {
                    id: note.id,
                    title: note.title,
                    body: note.body
                });
                
                // Clear red dot
                note.isDirty = false;
                console.log(`[Manual save] User saved note "${note.title || 'Untitled'}"`);
            } catch (error) {
                console.error('Save failed:', error);
            }
        }
    }

    // Save active note
   async saveActive() {
    if (!this.activeNote) return;
    
    const note = this.activeNote;
    
    // If no file path, ask user where to save
    if (!note.filePath) {
        const filePath = await save({
            defaultPath: note.title || 'Untitled.html',
            filters: [{
                name: 'HTML',
                extensions: ['html']
            }]
        });
        
        if (!filePath) return; // User cancelled
        
        note.filePath = filePath;
    }
    
    // Save to the file path
    try {
        await writeTextFile(note.filePath, note.body);
        
        // Also save to backend
        await invoke('save_note', {
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

// Export a global singleton instance
export const noteStore = new NoteState();
