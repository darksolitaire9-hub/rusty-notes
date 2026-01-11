use serde::{Deserialize, Serialize};

/// Core note metadata stored in the database.
/// - Text content lives here in `body` (for search, quick display).
/// - `file_path` points to the rendered/serialized note file on disk (HTML/Markdown, etc.).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    /// Stable unique ID for the note (UUID as string).
    pub id: String,

    /// User-facing title of the note.
    pub title: String,

    /// Main text content of the note (for search, previews, etc.).
    pub body: String,

    /// Unix timestamp (seconds) when the note was created.
    pub created_at: i64,

    /// Unix timestamp (seconds) when the note was last updated.
    pub updated_at: i64,

    /// Full filesystem path to the note’s file (e.g. HTML/Markdown).
    /// This is what your delete / trash logic will use instead of guessing.
    pub file_path: String,
}

/// File/asset attached to a note.
/// Each attachment is a separate record pointing back to `note_id`.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attachment {
    /// Stable unique ID for the attachment (string/UUID).
    pub id: String,

    /// Foreign key to the owning note.
    pub note_id: String,

    /// Logical type of the attachment (image, pdf, audio, etc.).
    /// Serialized as `type` to keep JSON/TOML nice.
    #[serde(rename = "type")]
    pub attachment_type: String,

    /// Human-readable file name (e.g. `invoice.pdf`).
    /// Useful for display; not required to be unique.
    pub file_name: String,

    /// Full filesystem path to the attachment file on disk.
    /// Delete / move logic uses this directly.
    pub file_path: String,

    /// Optional MIME type (e.g. `image/png`, `application/pdf`).
    pub mime_type: Option<String>,

    /// Optional file size in bytes, captured at save/import time.
    pub size_bytes: Option<i64>,

    /// Unix timestamp (seconds) when the attachment was created/added.
    pub created_at: i64,
}

/// Combined view for convenience in Rust / API:
/// - Flattens `Note` fields at the top level.
/// - Includes a list of attachments.
/// This is *not* a separate table; it’s a projection used in code.
#[derive(Debug, Serialize, Deserialize)]
pub struct NoteWithAttachments {
    /// All note fields (id, title, body, file_path, timestamps, etc.).
    #[serde(flatten)]
    pub note: Note,

    /// All attachments linked to this note.
    pub attachments: Vec<Attachment>,
}
