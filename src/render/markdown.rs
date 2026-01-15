use crate::models::StructuredNote;

pub fn render_notes(notes: &[StructuredNote]) -> String {
    let mut out = Vec::new();
    for (idx, note) in notes.iter().enumerate() {
        out.push(format!("# Structured Note {}", idx + 1));
        out.push(format!("Format: {:?}", note.format));
        if let Some(source) = &note.source_file {
            out.push(format!("Source: {}", source));
        }
        out.push(String::new());
        for section in &note.sections {
            out.push(format!("## {}", section.name));
            if section.content.is_empty() {
                out.push("(empty)".to_string());
            } else {
                out.push(section.content.clone());
            }
            out.push(String::new());
        }
        if idx + 1 < notes.len() {
            out.push("---".to_string());
            out.push(String::new());
        }
    }
    out.join("\n")
}
