export type Note = {
    id: string;
    title: string;
    body: string;
    updatedAt: Date;
    isDirty: boolean;
};

class NoteState {
    // 1. The Data
    notes = $state<Note[]>([]);
    selectedId = $state<string | null>(null);
    shouldFocusTitle = $state(false);
    
    // ✅ Auto-save timer
    private autoSaveTimer: number | null = null;
    private readonly AUTO_SAVE_DELAY = 120000; // 2 minutes in milliseconds

    // 2. Computed Values
    activeNote = $derived(
        this.notes.find((n) => n.id === this.selectedId) || null
    );

    constructor() {
        // Initial Dummy Data
        this.notes = [
            { 
                id: '1', 
                title: 'Architecture Plan', 
                body: 'Use Svelte 5 Runes for state.', 
                updatedAt: new Date(),
                isDirty: false
            }
        ];
    }

    // 3. Actions
    select(id: string) {
        this.selectedId = id;
        this.shouldFocusTitle = false;
    }

    create() {
        const newNote: Note = {
            id: crypto.randomUUID(),
            title: '',
            body: '',
            updatedAt: new Date(),
            isDirty: false
        };
        this.notes = [newNote, ...this.notes];
        this.selectedId = newNote.id;
        this.shouldFocusTitle = true;
    }

    delete(id: string) {
        this.notes = this.notes.filter(n => n.id !== id);
        if (this.selectedId === id) {
            this.selectedId = null;
        }
    }

    updateTitle(newTitle: string) {
        if (this.activeNote) {
            this.activeNote.title = newTitle;
            this.activeNote.updatedAt = new Date();
            this.activeNote.isDirty = true;  // Red dot appears
            
            this.scheduleAutoSave();  // ✅ Schedule background save
        }
    }

    updateBody(newBody: string) {
        if (this.activeNote) {
            this.activeNote.body = newBody;
            this.activeNote.updatedAt = new Date();
            this.activeNote.isDirty = true;  // Red dot appears
            
            this.scheduleAutoSave();  // ✅ Schedule background save
        }
    }

    // ✅ Auto-save: Saves data but keeps red dot
    private scheduleAutoSave() {
        // Clear existing timer
        if (this.autoSaveTimer) {
            clearTimeout(this.autoSaveTimer);
        }
        
        // Schedule new auto-save
        this.autoSaveTimer = setTimeout(() => {
            if (this.activeNote) {
                this.autoSaveToBackend(this.activeNote);
                // Note: isDirty stays TRUE - red dot remains!
            }
        }, this.AUTO_SAVE_DELAY) as unknown as number;
    }

    // ✅ Background save (silent, no UI change)
    private autoSaveToBackend(note: Note) {
        console.log(`[Auto-save] Saved note "${note.title || 'Untitled'}" to backend`);
        // TODO: Call Tauri backend to persist to disk
        // await invoke('save_note', { note });
    }

    // ✅ Manual save: Clears red dot (user acknowledgment)
    save(id: string) {
        const note = this.notes.find(n => n.id === id);
        if (note) {
            note.isDirty = false;  // ✅ Clear red dot
            this.autoSaveToBackend(note);  // Also save immediately
            console.log(`[Manual save] User saved note "${note.title || 'Untitled'}"`);
        }
    }

    // ✅ Save active note
    saveActive() {
        if (this.activeNote) {
            this.save(this.activeNote.id);
        }
    }
}

// Export a global singleton instance
export const noteStore = new NoteState();
