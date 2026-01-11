import { invoke } from '@tauri-apps/api/core';
import type { NoteDTO, NoteWithAttachmentsDTO } from '../notes-types';

/**
 * Tauri API wrapper for note operations
 */
export class NotesAPI {
  static async list(): Promise<NoteDTO[]> {
    return await invoke<NoteDTO[]>('list_notes');
  }

  static async listWithAttachments(): Promise<NoteWithAttachmentsDTO[]> {
    return await invoke<NoteWithAttachmentsDTO[]>('list_notes_with_attachments');
  }

  static async getById(id: string): Promise<NoteDTO | null> {
    return await invoke<NoteDTO | null>('get_note', { id });
  }

  static async getWithAttachments(id: string): Promise<NoteWithAttachmentsDTO | null> {
    return await invoke<NoteWithAttachmentsDTO | null>('get_note_with_attachments', { id });
  }

  static async create(title: string, body: string): Promise<NoteDTO> {
    return await invoke<NoteDTO>('create_note', { title, body });
  }

  static async update(id: string, title: string, body: string): Promise<void> {
    await invoke('update_note', { id, title, body });
  }

  /**
   * Soft delete - moves note to trash (is_deleted = 1)
   * Note is recoverable via restore()
   * 
   * Used when settings.delete_behavior = 'MoveToTrash'
   * Rust: TrashManager::move_to_trash()
   */
  static async softDelete(id: string): Promise<void> {
    await invoke('soft_delete_note', { id });
  }

  /**
   * Permanent delete - removes note file from disk
   * Note is NOT recoverable
   * 
   * Used when settings.delete_behavior = 'Permanent'
   * OR when user explicitly deletes from trash
   * Rust: PermanentDelete::delete()
   */
  static async delete(id: string): Promise<void> {
    await invoke('delete_note', { id });
  }

  /**
   * Restore a soft-deleted note
   * Only works for notes that were soft-deleted
   */
  static async restore(id: string): Promise<void> {
    await invoke('restore_note', { id });
  }

  /**
   * List all deleted notes (for trash view)
   * Only returns notes with is_deleted = 1
   */
  static async listDeleted(): Promise<NoteDTO[]> {
    return await invoke<NoteDTO[]>('list_deleted_notes');
  }

  static async search(query: string): Promise<NoteDTO[]> {
    return await invoke<NoteDTO[]>('search_notes', { query });
  }
}

