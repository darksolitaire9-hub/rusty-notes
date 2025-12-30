export type Note = {
    id: string;
    title: string;
    body: string;
    updatedAt: Date;
};

class NoteState {
    // 1. The Data
    notes = $state<Note[]>([]);
    selectedId = $state<string | null>(null);
    shouldFocusTitle = $state(false); // Flag to control focus

    // 2. Computed Values
    activeNote = $derived(
        this.notes.find((n) => n.id === this.selectedId) || null
    );

    constructor() {
        // Initial Dummy Data
        this.notes = [
            { id: '1', title: 'Architecture Plan', body: 'Use Svelte 5 Runes for state.', updatedAt: new Date() }
        ];
    }

    // 3. Actions
    select(id: string) {
        this.selectedId = id;
        this.shouldFocusTitle = false; // Don't focus title when just switching notes
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
        this.shouldFocusTitle = true; // Signal the editor to focus
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
