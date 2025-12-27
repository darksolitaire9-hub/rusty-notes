// src/lib/state/notes.svelte.ts

export type Note = {
    id: string;
    title: string;
    body: string;
    updatedAt: Date;
};

class NoteState {
    // 1. The Data (Private access recommended, but public is fine for simplicity)
    notes = $state<Note[]>([]);
    selectedId = $state<string | null>(null);

    // 2. Computed Values
    // This automatically updates whenever notes or selectedId changes
    activeNote = $derived(
        this.notes.find((n) => n.id === this.selectedId) || null
    );

    constructor() {
        // We can load initial data here later
        this.notes = [
            { id: '1', title: 'Architecture Plan', body: 'Use Svelte 5 Runes for state.', updatedAt: new Date() }
        ];
    }

    // 3. Actions (The API)
    select(id: string) {
        this.selectedId = id;
    }

    create() {
        const newNote: Note = {
            id: crypto.randomUUID(),
            title: '',
            body: '',
            updatedAt: new Date()
        };
        // Add to top of list
        this.notes = [newNote, ...this.notes];
        this.selectedId = newNote.id;
    }

    delete(id: string) {
        this.notes = this.notes.filter(n => n.id !== id);
        if (this.selectedId === id) {
            this.selectedId = null;
        }
    }

    updateBody(newBody: string) {
        if (this.activeNote) {
            this.activeNote.body = newBody;
            this.activeNote.updatedAt = new Date();
        }
    }
}

// Export a global singleton instance
export const noteStore = new NoteState();
