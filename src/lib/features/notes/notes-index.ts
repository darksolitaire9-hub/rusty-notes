// src/lib/features/notes/index.ts

// Export service (main reactive singleton)
export { noteService } from './store/notes-service.svelte';

export type {
  Note,
  NoteDTO,
  Attachment,
  AttachmentDTO,
  NoteWithAttachments,
  NoteWithAttachmentsDTO
} from './notes-types';

export { NotesAPI } from './api/notes-tauri-commands-wrapper';
