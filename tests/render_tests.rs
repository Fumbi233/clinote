use clinote::models::{CsvLayout, Metadata, NoteFormat, Section, StructuredNote};
use clinote::render::{self, OutputFormat};

fn sample_note() -> StructuredNote {
    StructuredNote {
        id: "note-1".to_string(),
        format: NoteFormat::Soap,
        source_file: Some("input.txt".to_string()),
        note_index: 1,
        sections: vec![Section {
            name: "Subjective".to_string(),
            content: "Synthetic subjective content".to_string(),
            confidence: 0.9,
        }],
        warnings: Vec::new(),
        metadata: Metadata {
            generated_at: "2024-01-01T00:00:00Z".to_string(),
            tool_version: "0.1.0".to_string(),
        },
    }
}

#[test]
fn renders_markdown() {
    let note = sample_note();
    let output = render::render_notes(&[note], OutputFormat::Md, CsvLayout::Wide).unwrap();
    assert!(output.contains("## Subjective"));
}

#[test]
fn renders_json() {
    let note = sample_note();
    let output = render::render_notes(&[note], OutputFormat::Json, CsvLayout::Wide).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert!(parsed.get("sections").is_some());
}

#[test]
fn renders_csv_wide() {
    let note = sample_note();
    let output = render::render_notes(&[note], OutputFormat::Csv, CsvLayout::Wide).unwrap();
    assert!(output.contains("Subjective"));
}
