use crate::config::Config;
use crate::models::{HeadingLine, NoteFormat, ParseWarning, SectionCandidate, WarningSeverity};
use crate::parser::headings;
use crate::parser::warnings;
use crate::util;
use once_cell::sync::Lazy;
use regex::Regex;

static FALLBACK_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)^(?P<h>[A-Za-z /&.-]{2,40})\s*[:\-]\s*(?P<rest>.+)$").unwrap());

pub fn extract_sections(
    lines: &[String],
    headings_found: &[HeadingLine],
    format: NoteFormat,
    config: &Config,
    apply_heuristics: bool,
) -> (Vec<SectionCandidate>, Vec<ParseWarning>) {
    let mut warnings_list = Vec::new();
    let mut headings = headings_found.to_vec();
    let mut used_fallback = false;

    if headings.is_empty() {
        if apply_heuristics {
            headings = fallback_headings(lines, config);
            if !headings.is_empty() {
                used_fallback = true;
                warnings_list.push(warnings::warning(
                    "fallback_heuristics",
                    "Fallback heuristics applied to find headings".to_string(),
                    1,
                    lines.len().max(1),
                    WarningSeverity::Info,
                ));
            }
        }
        if headings.is_empty() {
            warnings_list.push(warnings::warning(
                "no_headings",
                "No headings detected; content grouped as Narrative".to_string(),
                1,
                lines.len().max(1),
                WarningSeverity::Warning,
            ));
            let content = lines.join("\n").trim().to_string();
            let candidate = SectionCandidate {
                name: "Narrative".to_string(),
                raw_heading: "Narrative".to_string(),
                content,
                start_line: 1,
                end_line: lines.len().max(1),
                confidence: 0.4,
            };
            return (vec![candidate], warnings_list);
        }
    }

    headings.sort_by_key(|h| h.line_num);

    let section_order = config.section_order(format);
    let mut candidates = Vec::new();

    for (idx, heading) in headings.iter().enumerate() {
        let start_line = heading.line_num;
        let end_line = if idx + 1 < headings.len() {
            headings[idx + 1].line_num.saturating_sub(1)
        } else {
            lines.len().max(1)
        };

        let mut content_lines = Vec::new();
        if let Some(inline) = &heading.inline_content {
            content_lines.push(inline.clone());
        }
        let content_start = heading.line_num + 1;
        for line_idx in content_start..=end_line {
            if let Some(line) = lines.get(line_idx - 1) {
                content_lines.push(line.clone());
            }
        }

        let (name, mapped) = map_heading(&heading.heading, &section_order);
        if !mapped {
            warnings_list.push(warnings::warning(
                "unmapped_heading",
                format!("Heading '{}' not in target format", heading.heading),
                start_line,
                end_line,
                WarningSeverity::Info,
            ));
        }

        let confidence = if used_fallback { 0.6 } else { 0.85 };
        let candidate = SectionCandidate {
            name,
            raw_heading: heading.heading.clone(),
            content: content_lines.join("\n").trim().to_string(),
            start_line,
            end_line,
            confidence,
        };
        candidates.push(candidate);
    }

    let mut ordered = Vec::new();
    for name in section_order {
        let key = util::normalize_heading_key(&name);
        for candidate in &candidates {
            if util::normalize_heading_key(&candidate.name) == key {
                ordered.push(candidate.clone());
            }
        }
    }
    for candidate in candidates {
        if util::normalize_heading_key(&candidate.name) == util::normalize_heading_key("Narrative")
        {
            ordered.push(candidate);
        }
    }

    (ordered, warnings_list)
}

fn map_heading(heading: &str, section_order: &[String]) -> (String, bool) {
    let heading_key = util::normalize_heading_key(heading);
    for name in section_order {
        if util::normalize_heading_key(name) == heading_key {
            return (name.clone(), true);
        }
    }
    ("Narrative".to_string(), false)
}

fn fallback_headings(lines: &[String], config: &Config) -> Vec<HeadingLine> {
    let mut headings = Vec::new();
    for (idx, line) in lines.iter().enumerate() {
        if let Some(caps) = FALLBACK_RE.captures(line.trim()) {
            let raw = caps.name("h").map(|m| m.as_str()).unwrap_or("");
            let rest = caps.name("rest").map(|m| m.as_str()).unwrap_or("");
            if let Some(mapped) = headings::canonicalize_heading(raw, config) {
                headings.push(HeadingLine {
                    line_num: idx + 1,
                    raw: line.clone(),
                    heading: mapped,
                    inline_content: Some(rest.trim().to_string()),
                });
            }
        }
    }
    headings
}
