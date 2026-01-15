use crate::config::Config;
use crate::models::HeadingLine;
use crate::util;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

static INLINE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?P<h>[A-Za-z0-9 /&.-]{1,40}):\s*(?P<rest>.+)$").unwrap());
static COLON_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?P<h>[A-Za-z0-9 /&.-]{2,40}):\s*$").unwrap());
static ALL_CAPS_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[A-Z][A-Z0-9 /&-]{1,40}$").unwrap());

static HEADING_MAP: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut map = HashMap::new();
    let pairs = vec![
        ("SUBJECTIVE", "Subjective"),
        ("OBJECTIVE", "Objective"),
        ("ASSESSMENT", "Assessment"),
        ("DIAGNOSIS", "Assessment"),
        ("DX", "Assessment"),
        ("PLAN", "Plan"),
        ("S", "Subjective"),
        ("O", "Objective"),
        ("A", "Assessment"),
        ("P", "Plan"),
        ("CHIEF COMPLAINT", "Chief Complaint"),
        ("CC", "Chief Complaint"),
        ("HPI", "HPI"),
        ("HISTORY OF PRESENT ILLNESS", "HPI"),
        ("PMH", "PMH"),
        ("PAST MEDICAL HISTORY", "PMH"),
        ("HX", "PMH"),
        ("MEDS", "Medications"),
        ("MEDICATIONS", "Medications"),
        ("ALLERGIES", "Allergies"),
        ("ALLERGY", "Allergies"),
        ("ROS", "ROS"),
        ("REVIEW OF SYSTEMS", "ROS"),
        ("PHYSICAL EXAM", "Physical Exam"),
        ("PHYSICAL EXAMINATION", "Physical Exam"),
        ("PE", "Physical Exam"),
        ("ADMISSION DX", "Admission Dx"),
        ("ADMISSION DIAGNOSIS", "Admission Dx"),
        ("ADMIT DX", "Admission Dx"),
        ("DISCHARGE DX", "Discharge Dx"),
        ("DISCHARGE DIAGNOSIS", "Discharge Dx"),
        ("HOSPITAL COURSE", "Hospital Course"),
        ("COURSE", "Hospital Course"),
        ("FOLLOW UP", "Follow-up"),
        ("FOLLOW-UP", "Follow-up"),
        ("FOLLOWUP", "Follow-up"),
        ("DISPOSITION", "Disposition"),
        ("DISPO", "Disposition"),
        ("INSTRUCTIONS", "Instructions"),
        ("DISCHARGE INSTRUCTIONS", "Instructions"),
    ];
    for (k, v) in pairs {
        map.insert(k.to_string(), v.to_string());
    }
    map
});

pub fn scan_headings(lines: &[String], config: &Config) -> Vec<HeadingLine> {
    let mut headings = Vec::new();
    for (idx, line) in lines.iter().enumerate() {
        if let Some((heading, inline)) = detect_heading(line, config) {
            headings.push(HeadingLine {
                line_num: idx + 1,
                raw: line.clone(),
                heading,
                inline_content: inline,
            });
        }
    }
    headings
}

pub fn detect_heading(line: &str, config: &Config) -> Option<(String, Option<String>)> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }

    if let Some(caps) = ALL_CAPS_RE.captures(trimmed) {
        let raw = caps.get(0).map(|m| m.as_str()).unwrap_or("");
        if let Some(mapped) = canonicalize_heading(raw, config) {
            return Some((mapped, None));
        }
    }

    if let Some(caps) = COLON_RE.captures(trimmed) {
        let raw = caps.name("h").map(|m| m.as_str()).unwrap_or("");
        if let Some(mapped) = canonicalize_heading(raw, config) {
            return Some((mapped, None));
        }
    }

    if let Some(caps) = INLINE_RE.captures(trimmed) {
        let raw = caps.name("h").map(|m| m.as_str()).unwrap_or("");
        let rest = caps.name("rest").map(|m| m.as_str()).unwrap_or("");
        if let Some(mapped) = canonicalize_heading(raw, config) {
            return Some((mapped, Some(rest.trim().to_string())));
        }
    }

    None
}

pub fn canonicalize_heading(raw: &str, config: &Config) -> Option<String> {
    if let Some(mapped) = config.resolve_heading_alias(raw) {
        return Some(mapped);
    }
    let key = util::normalize_heading_key(raw);
    HEADING_MAP.get(&key).cloned()
}
