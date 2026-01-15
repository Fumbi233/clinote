use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[clap(rename_all = "lowercase")]
pub enum NoteFormat {
    Soap,
    Hp,
    Discharge,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[clap(rename_all = "lowercase")]
pub enum BundleMode {
    Auto,
    On,
    Off,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CsvLayout {
    Wide,
    Long,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WarningSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SectionName {
    #[serde(rename = "Subjective", alias = "S", alias = "SUBJECTIVE")]
    Subjective,
    #[serde(rename = "Objective", alias = "O", alias = "OBJECTIVE")]
    Objective,
    #[serde(
        rename = "Assessment",
        alias = "A",
        alias = "ASSESSMENT",
        alias = "DX",
        alias = "DIAGNOSIS"
    )]
    Assessment,
    #[serde(rename = "Plan", alias = "P", alias = "PLAN")]
    Plan,
    #[serde(
        rename = "Chief Complaint",
        alias = "CC",
        alias = "CHIEF COMPLAINT",
        alias = "Chief complaint"
    )]
    ChiefComplaint,
    #[serde(
        rename = "HPI",
        alias = "History of Present Illness",
        alias = "HISTORY OF PRESENT ILLNESS"
    )]
    Hpi,
    #[serde(
        rename = "PMH",
        alias = "Past Medical History",
        alias = "PAST MEDICAL HISTORY",
        alias = "HX",
        alias = "Hx"
    )]
    Pmh,
    #[serde(rename = "Medications", alias = "Meds", alias = "MEDS")]
    Medications,
    #[serde(rename = "Allergies", alias = "Allergy", alias = "ALLERGIES")]
    Allergies,
    #[serde(
        rename = "ROS",
        alias = "Review of Systems",
        alias = "REVIEW OF SYSTEMS"
    )]
    Ros,
    #[serde(
        rename = "Physical Exam",
        alias = "PE",
        alias = "Physical Examination",
        alias = "PHYSICAL EXAM"
    )]
    PhysicalExam,
    #[serde(
        rename = "Admission Dx",
        alias = "Admission Diagnosis",
        alias = "Admit Dx",
        alias = "ADMISSION DX"
    )]
    AdmissionDx,
    #[serde(
        rename = "Discharge Dx",
        alias = "Discharge Diagnosis",
        alias = "Discharge DX"
    )]
    DischargeDx,
    #[serde(
        rename = "Hospital Course",
        alias = "Course",
        alias = "HOSPITAL COURSE"
    )]
    HospitalCourse,
    #[serde(
        rename = "Follow-up",
        alias = "Follow Up",
        alias = "Followup",
        alias = "FOLLOW-UP"
    )]
    FollowUp,
    #[serde(rename = "Disposition", alias = "Dispo", alias = "DISPOSITION")]
    Disposition,
    #[serde(
        rename = "Instructions",
        alias = "Discharge Instructions",
        alias = "INSTRUCTIONS"
    )]
    Instructions,
    #[serde(rename = "Narrative", alias = "Other")]
    Narrative,
}

impl SectionName {
    pub fn as_str(&self) -> &'static str {
        match self {
            SectionName::Subjective => "Subjective",
            SectionName::Objective => "Objective",
            SectionName::Assessment => "Assessment",
            SectionName::Plan => "Plan",
            SectionName::ChiefComplaint => "Chief Complaint",
            SectionName::Hpi => "HPI",
            SectionName::Pmh => "PMH",
            SectionName::Medications => "Medications",
            SectionName::Allergies => "Allergies",
            SectionName::Ros => "ROS",
            SectionName::PhysicalExam => "Physical Exam",
            SectionName::AdmissionDx => "Admission Dx",
            SectionName::DischargeDx => "Discharge Dx",
            SectionName::HospitalCourse => "Hospital Course",
            SectionName::FollowUp => "Follow-up",
            SectionName::Disposition => "Disposition",
            SectionName::Instructions => "Instructions",
            SectionName::Narrative => "Narrative",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub name: String,
    pub content: String,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseWarning {
    pub code: String,
    pub message: String,
    pub line_start: usize,
    pub line_end: usize,
    pub severity: WarningSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub generated_at: String,
    pub tool_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredNote {
    pub id: String,
    pub format: NoteFormat,
    pub source_file: Option<String>,
    pub note_index: usize,
    pub sections: Vec<Section>,
    pub warnings: Vec<ParseWarning>,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingLine {
    pub line_num: usize,
    pub raw: String,
    pub heading: String,
    pub inline_content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionCandidate {
    pub name: String,
    pub raw_heading: String,
    pub content: String,
    pub start_line: usize,
    pub end_line: usize,
    pub confidence: f32,
}
