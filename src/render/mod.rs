pub mod csv;
pub mod json;
pub mod markdown;

use crate::models::{CsvLayout, StructuredNote};
use anyhow::Result;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[clap(rename_all = "lowercase")]
pub enum OutputFormat {
    Md,
    Json,
    Csv,
}

impl OutputFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            OutputFormat::Md => "md",
            OutputFormat::Json => "json",
            OutputFormat::Csv => "csv",
        }
    }
}

pub fn render_notes(
    notes: &[StructuredNote],
    format: OutputFormat,
    layout: CsvLayout,
) -> Result<String> {
    match format {
        OutputFormat::Md => Ok(markdown::render_notes(notes)),
        OutputFormat::Json => json::render_notes(notes),
        OutputFormat::Csv => csv::render_notes(notes, layout),
    }
}
