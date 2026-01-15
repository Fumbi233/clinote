use crate::config::Config;
use crate::models::{BundleMode, ParseWarning, WarningSeverity};
use crate::parser::warnings;
use once_cell::sync::Lazy;
use regex::Regex;

static DATE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\s*(\d{4}-\d{2}-\d{2}|\d{2}/\d{2}/\d{4})").unwrap());

pub fn split_bundle(
    text: &str,
    mode: BundleMode,
    config: &Config,
) -> (Vec<String>, Vec<ParseWarning>) {
    match mode {
        BundleMode::Off => (vec![text.to_string()], Vec::new()),
        BundleMode::On => split_bundle_internal(text, config, true),
        BundleMode::Auto => split_bundle_internal(text, config, false),
    }
}

fn split_bundle_internal(
    text: &str,
    config: &Config,
    strict: bool,
) -> (Vec<String>, Vec<ParseWarning>) {
    let mut warnings_list = Vec::new();
    let mut notes = split_on_delimiters(text, &config.bundle.delimiters);
    if notes.len() <= 1 {
        notes = split_on_dates(text);
    }

    if notes.len() <= 1 {
        if strict {
            warnings_list.push(warnings::warning(
                "bundle_not_split",
                "Bundle mode requested but no clear delimiters found".to_string(),
                1,
                text.lines().count().max(1),
                WarningSeverity::Warning,
            ));
        }
        return (vec![text.to_string()], warnings_list);
    }

    (notes, warnings_list)
}

fn split_on_delimiters(text: &str, delimiters: &[String]) -> Vec<String> {
    let mut notes = Vec::new();
    let mut current = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if delimiters.iter().any(|d| d.trim() == trimmed) {
            if !current.is_empty() {
                notes.push(current.join("\n").trim().to_string());
                current.clear();
            }
            continue;
        }
        current.push(line.to_string());
    }
    if !current.is_empty() {
        notes.push(current.join("\n").trim().to_string());
    }
    if notes.is_empty() {
        notes.push(text.to_string());
    }
    notes
}

fn split_on_dates(text: &str) -> Vec<String> {
    let mut notes = Vec::new();
    let mut current = Vec::new();
    let mut found = 0;
    for line in text.lines() {
        if DATE_RE.is_match(line) {
            if !current.is_empty() {
                notes.push(current.join("\n").trim().to_string());
                current.clear();
            }
            found += 1;
        }
        current.push(line.to_string());
    }
    if !current.is_empty() {
        notes.push(current.join("\n").trim().to_string());
    }
    if found <= 1 {
        vec![text.to_string()]
    } else {
        notes
    }
}
