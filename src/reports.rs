use crate::models::StructuredNote;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchFailure {
    pub file: String,
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchReport {
    pub tool_name: String,
    pub version: String,
    pub total_files: usize,
    pub ok_files: usize,
    pub failed_files: usize,
    pub counts_by_section: HashMap<String, usize>,
    pub warnings_count: usize,
    pub failures: Vec<BatchFailure>,
    pub runtime_ms: u128,
}

impl BatchReport {
    pub fn new(tool_name: &str) -> Self {
        Self {
            tool_name: tool_name.to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            total_files: 0,
            ok_files: 0,
            failed_files: 0,
            counts_by_section: HashMap::new(),
            warnings_count: 0,
            failures: Vec::new(),
            runtime_ms: 0,
        }
    }

    pub fn record_ok(&mut self, notes: &[StructuredNote]) {
        self.ok_files += 1;
        for note in notes {
            for section in &note.sections {
                *self
                    .counts_by_section
                    .entry(section.name.clone())
                    .or_insert(0) += 1;
            }
            self.warnings_count += note.warnings.len();
        }
    }

    pub fn record_failure(&mut self, file: &str, error: String) {
        self.failed_files += 1;
        self.failures.push(BatchFailure {
            file: file.to_string(),
            error,
        });
    }

    pub fn finalize(&mut self) {
        self.total_files = self.ok_files + self.failed_files;
    }

    pub fn write_to(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        crate::util::write_string(path, &json)?;
        Ok(())
    }
}
