use crate::models::SectionCandidate;
use anyhow::{anyhow, Result};
use inquire::{Confirm, MultiSelect, Text};

pub fn prompt_apply_heuristics() -> Result<bool> {
    let answer = Confirm::new("Apply fallback heuristics for missing headings?")
        .with_default(true)
        .prompt();
    map_prompt(answer)
}

pub fn review_sections(candidates: &[SectionCandidate]) -> Result<Vec<SectionCandidate>> {
    if candidates.is_empty() {
        return Err(anyhow!("No sections available for review"));
    }

    let labels: Vec<String> = candidates
        .iter()
        .enumerate()
        .map(|(idx, c)| {
            format!(
                "{}: {} (lines {}-{})",
                idx + 1,
                c.name,
                c.start_line,
                c.end_line
            )
        })
        .collect();

    let selected = MultiSelect::new("Keep which sections?", labels.clone()).prompt();
    let selections = map_prompt(selected)?;

    let mut chosen = Vec::new();
    for label in selections {
        if let Some(index_str) = label.split(':').next() {
            if let Ok(index) = index_str.trim().parse::<usize>() {
                if let Some(section) = candidates.get(index.saturating_sub(1)) {
                    chosen.push(section.clone());
                }
            }
        }
    }

    if chosen.is_empty() {
        return Err(anyhow!("Interactive review removed all sections"));
    }

    let mut renamed = Vec::new();
    for section in chosen {
        let prompt = format!("Rename section '{}' (leave as-is to keep)", section.name);
        let name = Text::new(&prompt).with_default(&section.name).prompt();
        let new_name = map_prompt(name)?;
        let mut updated = section.clone();
        updated.name = new_name;
        renamed.push(updated);
    }

    let confirm = Confirm::new("Proceed with these sections?")
        .with_default(true)
        .prompt();
    if !map_prompt(confirm)? {
        return Err(anyhow!("Interactive session canceled"));
    }

    Ok(renamed)
}

fn map_prompt<T>(result: std::result::Result<T, inquire::error::InquireError>) -> Result<T> {
    match result {
        Ok(value) => Ok(value),
        Err(inquire::error::InquireError::OperationCanceled)
        | Err(inquire::error::InquireError::OperationInterrupted) => {
            Err(anyhow!("Interactive session canceled"))
        }
        Err(err) => Err(anyhow!("Interactive prompt failed: {}", err)),
    }
}
