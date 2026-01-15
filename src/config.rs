use crate::models::{BundleMode, CsvLayout, NoteFormat, SectionName};
use crate::util;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub formats: FormatsConfig,
    #[serde(default)]
    pub heading_aliases: HashMap<String, String>,
    #[serde(default = "default_true")]
    pub enable_fallback_heuristics: bool,
    #[serde(default)]
    pub bundle: BundleConfig,
    #[serde(default)]
    pub csv: CsvConfig,
    #[serde(default = "default_glob")]
    pub glob_default: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatsConfig {
    pub soap: FormatSpec,
    pub hp: FormatSpec,
    pub discharge: FormatSpec,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatSpec {
    pub section_order: Vec<SectionName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleConfig {
    pub mode_default: BundleMode,
    pub delimiters: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsvConfig {
    pub layout: CsvLayout,
}

fn default_true() -> bool {
    true
}

fn default_glob() -> String {
    "*.txt".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            formats: FormatsConfig::default(),
            heading_aliases: HashMap::new(),
            enable_fallback_heuristics: true,
            bundle: BundleConfig::default(),
            csv: CsvConfig::default(),
            glob_default: default_glob(),
        }
    }
}

impl Default for FormatsConfig {
    fn default() -> Self {
        Self {
            soap: FormatSpec {
                section_order: vec![
                    SectionName::Subjective,
                    SectionName::Objective,
                    SectionName::Assessment,
                    SectionName::Plan,
                ],
            },
            hp: FormatSpec {
                section_order: vec![
                    SectionName::ChiefComplaint,
                    SectionName::Hpi,
                    SectionName::Pmh,
                    SectionName::Medications,
                    SectionName::Allergies,
                    SectionName::Ros,
                    SectionName::PhysicalExam,
                    SectionName::Assessment,
                    SectionName::Plan,
                ],
            },
            discharge: FormatSpec {
                section_order: vec![
                    SectionName::AdmissionDx,
                    SectionName::DischargeDx,
                    SectionName::HospitalCourse,
                    SectionName::Medications,
                    SectionName::FollowUp,
                    SectionName::Disposition,
                    SectionName::Instructions,
                ],
            },
        }
    }
}

impl Default for BundleConfig {
    fn default() -> Self {
        Self {
            mode_default: BundleMode::Auto,
            delimiters: vec!["----- NOTE -----".to_string(), "=== VISIT ===".to_string()],
        }
    }
}

impl Default for CsvConfig {
    fn default() -> Self {
        Self {
            layout: CsvLayout::Wide,
        }
    }
}

impl Config {
    pub fn load(path: Option<&Path>) -> Result<Self> {
        let candidate = match path {
            Some(path) => PathBuf::from(path),
            None => PathBuf::from("clinote.toml"),
        };
        if candidate.exists() {
            let content = fs::read_to_string(&candidate)?;
            let config: Config = toml::from_str(&content).map_err(|err| {
                anyhow!("Failed to parse config {}: {}", candidate.display(), err)
            })?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    pub fn section_order(&self, format: NoteFormat) -> Vec<String> {
        let list = match format {
            NoteFormat::Soap => &self.formats.soap.section_order,
            NoteFormat::Hp => &self.formats.hp.section_order,
            NoteFormat::Discharge => &self.formats.discharge.section_order,
        };
        list.iter().map(|s| s.as_str().to_string()).collect()
    }

    pub fn resolve_heading_alias(&self, raw: &str) -> Option<String> {
        let raw_key = util::normalize_heading_key(raw);
        self.heading_aliases.iter().find_map(|(k, v)| {
            if util::normalize_heading_key(k) == raw_key {
                Some(v.clone())
            } else {
                None
            }
        })
    }

    pub fn summary(&self) -> String {
        let mut out = String::new();
        out.push_str("Resolved section order:\n");
        out.push_str(&format!(
            "SOAP: {}\n",
            self.section_order(NoteFormat::Soap).join(", ")
        ));
        out.push_str(&format!(
            "H&P: {}\n",
            self.section_order(NoteFormat::Hp).join(", ")
        ));
        out.push_str(&format!(
            "Discharge: {}\n",
            self.section_order(NoteFormat::Discharge).join(", ")
        ));
        out.push_str("\nHeading aliases:\n");
        if self.heading_aliases.is_empty() {
            out.push_str("(none)\n");
        } else {
            for (k, v) in &self.heading_aliases {
                out.push_str(&format!("{} => {}\n", k, v));
            }
        }
        out.push_str("\nBundle delimiters:\n");
        for delimiter in &self.bundle.delimiters {
            out.push_str(&format!("- {}\n", delimiter));
        }
        out
    }
}
