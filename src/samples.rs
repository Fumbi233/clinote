use crate::models::{Metadata, NoteFormat, Section, StructuredNote};
use crate::util;
use anyhow::Result;
use std::path::Path;

pub fn generate_samples(out_dir: &Path, n: usize, bundles: usize) -> Result<()> {
    std::fs::create_dir_all(out_dir)?;

    let mut samples = Vec::new();
    for i in 0..n {
        let format = match i % 3 {
            0 => NoteFormat::Soap,
            1 => NoteFormat::Hp,
            _ => NoteFormat::Discharge,
        };
        let (text, note) = synthetic_note(format, i + 1);
        let txt_path = out_dir.join(format!("sample_{}.txt", i + 1));
        let json_path = out_dir.join(format!("sample_{}.gold.json", i + 1));
        util::write_string(&txt_path, &text)?;
        util::write_string(&json_path, &serde_json::to_string_pretty(&note)?)?;
        samples.push(text);
    }

    if bundles > 0 {
        for bundle_idx in 0..bundles {
            let start = bundle_idx * 2;
            let end = std::cmp::min(start + 3, samples.len());
            if start >= end {
                break;
            }
            let mut bundle_text = String::new();
            for (idx, note) in samples[start..end].iter().enumerate() {
                if idx > 0 {
                    bundle_text.push_str("\n----- NOTE -----\n");
                }
                bundle_text.push_str(note);
            }
            let bundle_path = out_dir.join(format!("bundle_{}.txt", bundle_idx + 1));
            util::write_string(&bundle_path, &bundle_text)?;
        }
    }

    Ok(())
}

fn synthetic_note(format: NoteFormat, index: usize) -> (String, StructuredNote) {
    let mut sections = Vec::new();
    let mut text = String::new();
    text.push_str(&format!("Patient: Synthetic Demo {}\n", index));
    text.push_str("DOB: 1990-01-01\n\n");

    let section_defs = match format {
        NoteFormat::Soap => vec![
            ("Subjective", vec!["Subjective:", "S:", "SUBJECTIVE"]),
            ("Objective", vec!["Objective:", "O:", "OBJECTIVE"]),
            ("Assessment", vec!["Assessment:", "A:", "DX:"]),
            ("Plan", vec!["Plan:", "P:", "PLAN"]),
        ],
        NoteFormat::Hp => vec![
            ("Chief Complaint", vec!["CC:", "Chief Complaint:"]),
            ("HPI", vec!["HPI:", "History of Present Illness:"]),
            ("PMH", vec!["PMH:", "Past Medical History:"]),
            ("Medications", vec!["Meds:", "Medications:"]),
            ("Allergies", vec!["Allergies:", "Allergy:"]),
            ("ROS", vec!["ROS:", "Review of Systems:"]),
            ("Physical Exam", vec!["Physical Exam:", "PE:"]),
            ("Assessment", vec!["Assessment:", "DX:"]),
            ("Plan", vec!["Plan:", "P:"]),
        ],
        NoteFormat::Discharge => vec![
            (
                "Admission Dx",
                vec!["Admission Dx:", "Admission Diagnosis:"],
            ),
            (
                "Discharge Dx",
                vec!["Discharge Dx:", "Discharge Diagnosis:"],
            ),
            ("Hospital Course", vec!["Hospital Course:", "Course:"]),
            ("Medications", vec!["Medications:", "Meds:"]),
            ("Follow-up", vec!["Follow-up:", "Follow Up:"]),
            ("Disposition", vec!["Disposition:", "Dispo:"]),
            (
                "Instructions",
                vec!["Instructions:", "Discharge Instructions:"],
            ),
        ],
    };

    for (idx, (name, variants)) in section_defs.iter().enumerate() {
        let heading = variants[idx % variants.len()];
        let content = format!(
            "Synthetic {} content for note {}.\n- Bullet A\n- Bullet B",
            name, index
        );
        if idx % 2 == 0 {
            text.push_str(&format!("{} {}\n\n", heading, content));
        } else {
            text.push_str(&format!("{}\n{}\n\n", heading, content));
        }
        sections.push(Section {
            name: name.to_string(),
            content,
            confidence: 0.95,
        });
    }

    let note = StructuredNote {
        id: format!("sample-{}", index),
        format,
        source_file: None,
        note_index: index,
        sections,
        warnings: Vec::new(),
        metadata: Metadata {
            generated_at: util::now_iso(),
            tool_version: env!("CARGO_PKG_VERSION").to_string(),
        },
    };

    (text.trim().to_string(), note)
}
