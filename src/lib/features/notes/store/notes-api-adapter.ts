// src/lib/features/notes/store/notes-api-adapter.ts

import { NotesAPI } from '../notes-index';
import { toNote } from '../notes-types';
import type { Note } from '../notes-types';

/**
 * Adapter layer between Tauri API and domain models
 * Converts DTOs to domain objects and handles errors
 */
export class NoteAPIAdapter {
  async loadAll(): Promise<Note[]> {
    try {
      const backend = await NotesAPI.list();
      return backend.map(toNote);
    } catch (error) {
      console.error('API: Failed to load notes', error);
      throw new Error('Failed to load notes from database');
    }
  }

  async createEmpty(): Promise<Note> {
    try {
      const backend = await NotesAPI.create('Untitled', '');
      const note = toNote(backend);
      note.isDirty = false;
      return note;
    } catch (error) {
      console.error('API: Failed to create note', error);
      throw new Error('Failed to create new note');
    }
  }

  async update(note: Note): Promise<void> {
    try {
      await NotesAPI.update(note.id, note.title, note.body);
    } catch (error) {
      console.error('API: Failed to update note', note.id, error);
      throw new Error('Failed to save note');
    }
  }

  async delete(id: string): Promise<void> {
    try {
      await NotesAPI.delete(id);
    } catch (error) {
      console.error('API: Failed to delete note', id, error);
      throw new Error('Failed to delete note');
    }
  }

  async softDelete(id: string): Promise<void> {
    try {
      await NotesAPI.softDelete(id);
    } catch (error) {
      console.error('API: Failed to soft delete note', id, error);
      throw new Error('Failed to move note to trash');
    }
  }

  async restore(id: string): Promise<void> {
    try {
      await NotesAPI.restore(id);
    } catch (error) {
      console.error('API: Failed to restore note', id, error);
      throw new Error('Failed to restore note');
    }
  }

  async search(query: string): Promise<Note[]> {
    try {
      const backend = await NotesAPI.search(query);
      return backend.map(toNote);
    } catch (error) {
      console.error('API: Failed to search notes', error);
      throw new Error('Failed to search notes');
    }
  }
}

export const noteAPIAdapter = new NoteAPIAdapter();
