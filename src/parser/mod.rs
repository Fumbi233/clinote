pub mod bundle;
pub mod headings;
pub mod normalize;
pub mod sectionize;
pub mod warnings;

use crate::config::Config;
use crate::models::{BundleMode, NoteFormat, ParseWarning, SectionCandidate, StructuredNote};
use crate::util;
use anyhow::Result;

#[derive(Debug, Clone, Copy)]
pub struct ParseOptions {
    pub apply_heuristics: bool,
}

pub fn split_bundle(
    text: &str,
    mode: BundleMode,
    config: &Config,
) -> (Vec<String>, Vec<ParseWarning>) {
    bundle::split_bundle(text, mode, config)
}

pub fn extract_candidates(
    text: &str,
    format: NoteFormat,
    config: &Config,
    options: ParseOptions,
) -> (Vec<SectionCandidate>, Vec<ParseWarning>) {
    let normalized = normalize::normalize_text(text);
    let lines: Vec<String> = normalized.lines().map(|l| l.to_string()).collect();
    let headings = headings::scan_headings(&lines, config);
    sectionize::extract_sections(&lines, &headings, format, config, options.apply_heuristics)
}

pub fn build_note(
    candidates: Vec<SectionCandidate>,
    format: NoteFormat,
    source_file: Option<String>,
    note_index: usize,
    mut warnings: Vec<ParseWarning>,
) -> StructuredNote {
    let mut sections = Vec::new();
    for candidate in candidates {
        if candidate.content.trim().is_empty() {
            warnings.push(warnings::warning(
                "empty_section",
                format!("Section {} has no content", candidate.name),
                candidate.start_line,
                candidate.end_line,
                crate::models::WarningSeverity::Info,
            ));
        }
        sections.push(crate::models::Section {
            name: candidate.name,
            content: candidate.content.trim().to_string(),
            confidence: candidate.confidence,
        });
    }

    StructuredNote {
        id: format!("note-{}-{}", note_index, util::now_iso()),
        format,
        source_file,
        note_index,
        sections,
        warnings,
        metadata: crate::models::Metadata {
            generated_at: util::now_iso(),
            tool_version: env!("CARGO_PKG_VERSION").to_string(),
        },
    }
}

pub fn parse_note(
    text: &str,
    format: NoteFormat,
    config: &Config,
    source_file: Option<String>,
    note_index: usize,
    options: ParseOptions,
) -> StructuredNote {
    let (candidates, warnings) = extract_candidates(text, format, config, options);
    build_note(candidates, format, source_file, note_index, warnings)
}

pub fn parse_notes(
    text: &str,
    format: NoteFormat,
    config: &Config,
    source_file: Option<String>,
    note_offset: usize,
    options: ParseOptions,
) -> Vec<StructuredNote> {
    let (notes, bundle_warnings) = split_bundle(text, config.bundle.mode_default, config);
    notes
        .into_iter()
        .enumerate()
        .map(|(idx, note_text)| {
            let (mut candidates, mut warnings) =
                extract_candidates(&note_text, format, config, options);
            warnings.extend(bundle_warnings.clone());
            build_note(
                candidates.drain(..).collect(),
                format,
                source_file.clone(),
                note_offset + idx + 1,
                warnings,
            )
        })
        .collect()
}

pub fn write_notes_to_file(path: &std::path::Path, content: &str) -> Result<()> {
    util::write_string(path, content)
}
