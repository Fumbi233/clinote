use clinote::config::Config;
use clinote::models::SectionName;

fn full_config_toml() -> String {
    r#"
[formats.soap]
section_order = ["Subjective", "Objective", "Assessment", "Plan"]

[formats.hp]
section_order = [
  "Chief Complaint",
  "HPI",
  "PMH",
  "Medications",
  "Allergies",
  "ROS",
  "Physical Exam",
  "Assessment",
  "Plan"
]

[formats.discharge]
section_order = [
  "Admission Dx",
  "Discharge Dx",
  "Hospital Course",
  "Medications",
  "Follow-up",
  "Disposition",
  "Instructions"
]

heading_aliases = { "Hx" = "PMH", "Dx" = "Assessment" }

enable_fallback_heuristics = true

[bundle]
mode_default = "auto"
delimiters = ["----- NOTE -----", "=== VISIT ==="]

[csv]
layout = "wide"

glob_default = "*.txt"
"#
    .to_string()
}

#[test]
fn parse_config_ok() {
    let config: Config = toml::from_str(&full_config_toml()).unwrap();
    assert!(config.enable_fallback_heuristics);
    assert_eq!(
        config.formats.soap.section_order[0],
        SectionName::Subjective
    );
    assert_eq!(
        config.bundle.mode_default,
        clinote::models::BundleMode::Auto
    );
}

#[test]
fn default_config_has_delimiters() {
    let config = Config::default();
    assert!(!config.bundle.delimiters.is_empty());
}

#[test]
fn invalid_section_name_errors() {
    let toml_str = r#"
[formats.soap]
section_order = ["NotASection"]

[formats.hp]
section_order = ["Chief Complaint"]

[formats.discharge]
section_order = ["Admission Dx"]

[bundle]
mode_default = "auto"
delimiters = ["----- NOTE -----"]

[csv]
layout = "wide"

glob_default = "*.txt"
"#;
    let result: Result<Config, _> = toml::from_str(toml_str);
    assert!(result.is_err());
}
