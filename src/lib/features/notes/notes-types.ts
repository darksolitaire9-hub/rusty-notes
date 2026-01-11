// src/lib/features/notes/types.ts

/**
 * Rust DTO - matches  Rust Note schema exactly
 */
export interface NoteDTO {
  id: string;
  title: string;
  body: string;
  created_at: number; // Unix timestamp (i64 in Rust)
  updated_at: number; // Unix timestamp (i64 in Rust)
  file_path: string;
  is_deleted?: number; // SQLite boolean (0 or 1)
}

/**
 * Attachment DTO - matches your Rust Attachment schema
 */
export interface AttachmentDTO {
  id: string;
  note_id: string;
  type: string; // renamed from attachment_type in Rust via serde
  file_name: string;
  file_path: string;
  mime_type: string | null;
  size_bytes: number | null;
  created_at: number;
}

/**
 * Combined DTO - matches NoteWithAttachments
 */
export interface NoteWithAttachmentsDTO {
  id: string;
  title: string;
  body: string;
  created_at: number;
  updated_at: number;
  file_path: string;
  is_deleted?: number;
  attachments: AttachmentDTO[];
}

/**
 * Frontend domain model for a Note
 */
export interface Note {
  id: string;
  title: string;
  body: string;
  createdAt: Date;
  updatedAt: Date;
  filePath: string;
  isDirty: boolean; // Frontend-only flag for unsaved changes
  isDeleted?: boolean;
}

/**
 * Frontend domain model for Attachment
 */
export interface Attachment {
  id: string;
  noteId: string;
  type: string;
  fileName: string;
  filePath: string;
  mimeType: string | null;
  sizeBytes: number | null;
  createdAt: Date;
}

/**
 * Combined frontend model
 */
export interface NoteWithAttachments extends Note {
  attachments: Attachment[];
}

/**
 * Convert Rust DTO to frontend Note model
 */
export function toNote(dto: NoteDTO): Note {
  return {
    id: dto.id,
    title: dto.title,
    body: dto.body,
    createdAt: new Date(dto.created_at * 1000), // Convert Unix timestamp to Date
    updatedAt: new Date(dto.updated_at * 1000),
    filePath: dto.file_path,
    isDirty: false,
    isDeleted: dto.is_deleted === 1
  };
}

/**
 * Convert Rust AttachmentDTO to frontend Attachment model
 */
export function toAttachment(dto: AttachmentDTO): Attachment {
  return {
    id: dto.id,
    noteId: dto.note_id,
    type: dto.type,
    fileName: dto.file_name,
    filePath: dto.file_path,
    mimeType: dto.mime_type,
    sizeBytes: dto.size_bytes,
    createdAt: new Date(dto.created_at * 1000)
  };
}

/**
 * Convert NoteWithAttachmentsDTO to frontend model
 */
export function toNoteWithAttachments(dto: NoteWithAttachmentsDTO): NoteWithAttachments {
  const note = toNote(dto);
  return {
    ...note,
    attachments: dto.attachments.map(toAttachment)
  };
}