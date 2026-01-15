use crate::models::StructuredNote;
use anyhow::Result;

pub fn render_notes(notes: &[StructuredNote]) -> Result<String> {
    if notes.len() == 1 {
        Ok(serde_json::to_string_pretty(&notes[0])?)
    } else {
        Ok(serde_json::to_string_pretty(&notes)?)
    }
}
