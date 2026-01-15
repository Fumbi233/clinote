use clinote::config::Config;
use clinote::models::{BundleMode, NoteFormat};
use clinote::parser;
use clinote::parser::headings;
use clinote::parser::sectionize;

#[test]
fn detects_heading_with_alias() {
    let mut config = Config::default();
    config
        .heading_aliases
        .insert("Hx".to_string(), "PMH".to_string());
    let line = "Hx:";
    let heading = headings::detect_heading(line, &config).unwrap();
    assert_eq!(heading.0, "PMH");
}

#[test]
fn fallback_heuristics_detects_dash_heading() {
    let config = Config::default();
    let lines = vec!["CC - chest pain".to_string(), "Other line".to_string()];
    let (sections, warnings) =
        sectionize::extract_sections(&lines, &[], NoteFormat::Hp, &config, true);
    assert!(!sections.is_empty());
    assert!(warnings.iter().any(|w| w.code == "fallback_heuristics"));
}

#[test]
fn bundle_splits_on_delimiter() {
    let config = Config::default();
    let text = "Note one\n----- NOTE -----\nNote two";
    let (notes, _warnings) = parser::split_bundle(text, BundleMode::On, &config);
    assert_eq!(notes.len(), 2);
}
