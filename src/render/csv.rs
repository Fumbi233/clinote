use crate::models::{CsvLayout, NoteFormat, StructuredNote};
use anyhow::Result;
use csv::Writer;
use std::collections::HashSet;

pub fn render_notes(notes: &[StructuredNote], layout: CsvLayout) -> Result<String> {
    match layout {
        CsvLayout::Wide => render_wide(notes),
        CsvLayout::Long => render_long(notes),
    }
}

fn render_wide(notes: &[StructuredNote]) -> Result<String> {
    let mut seen = HashSet::new();
    let mut section_names = Vec::new();
    for note in notes {
        for section in &note.sections {
            if seen.insert(section.name.clone()) {
                section_names.push(section.name.clone());
            }
        }
    }

    let mut wtr = Writer::from_writer(vec![]);
    let mut header = vec!["id", "format", "source_file", "note_index"]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    header.extend(section_names.iter().cloned());
    wtr.write_record(&header)?;

    for note in notes {
        let mut record = Vec::new();
        record.push(note.id.clone());
        record.push(format_label(note.format).to_string());
        record.push(note.source_file.clone().unwrap_or_default());
        record.push(note.note_index.to_string());

        for name in &section_names {
            let value = note
                .sections
                .iter()
                .find(|s| &s.name == name)
                .map(|s| s.content.clone())
                .unwrap_or_default();
            record.push(value);
        }
        wtr.write_record(&record)?;
    }

    let data = wtr.into_inner()?;
    Ok(String::from_utf8(data)?)
}

fn render_long(notes: &[StructuredNote]) -> Result<String> {
    let mut wtr = Writer::from_writer(vec![]);
    wtr.write_record([
        "note_id",
        "format",
        "source_file",
        "note_index",
        "section_name",
        "content",
    ])?;

    for note in notes {
        for section in &note.sections {
            wtr.write_record([
                note.id.as_str(),
                format_label(note.format),
                note.source_file.as_deref().unwrap_or(""),
                &note.note_index.to_string(),
                section.name.as_str(),
                section.content.as_str(),
            ])?;
        }
    }

    let data = wtr.into_inner()?;
    Ok(String::from_utf8(data)?)
}

fn format_label(format: NoteFormat) -> &'static str {
    match format {
        NoteFormat::Soap => "soap",
        NoteFormat::Hp => "hp",
        NoteFormat::Discharge => "discharge",
    }
}
