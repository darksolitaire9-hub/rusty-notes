use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub body: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attachment {
    pub id: String,
    pub note_id: String,
    #[serde(rename = "type")]
    pub attachment_type: String,
    pub file_name: String,
    pub file_path: String,
    pub mime_type: Option<String>,
    pub size_bytes: Option<i64>,
    pub created_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteWithAttachments {
    #[serde(flatten)]
    pub note: Note,
    pub attachments: Vec<Attachment>,
}
