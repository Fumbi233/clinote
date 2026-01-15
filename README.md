# Clinote CLI

Clinote CLI converts unstructured clinical notes into deterministic, standardized formats (SOAP, H&P, Discharge Summary) for documentation quality, interoperability, and data consistency. It uses rules and configuration only (no ML summarization) and ships with synthetic demo data for exams and coursework.

## Problem statement
Clinical notes arrive in wildly inconsistent shapes. As a student working with synthetic notes, I kept reformatting the same data to evaluate documentation quality and export to JSON/CSV. The manual cleanup was slow and error-prone, so I built a deterministic CLI that standardizes text with transparent rules and configurable mappings.

## Features
- Deterministic parsing into SOAP, H&P, and Discharge Summary structures
- Markdown, JSON, and CSV exports (wide or long)
- Bundle-aware parsing for multi-note files with warnings
- Interactive review mode to confirm sections, rename headings, and control heuristics
- Batch processing with per-file failure tracking and summary report
- Configurable heading aliases and section ordering via TOML

## Install and run
```bash
cargo build --release
./target/release/clinote --help
```

### Parse a single note
```bash
clinote parse --input notes/sample.txt --format soap \
  --out output.json --out-format json --bundle auto
```

### Batch process a folder
```bash
clinote batch --input-dir notes --glob "*.txt" \
  --format hp --out-dir outputs --out-format csv
```

### Generate synthetic samples
```bash
clinote sample --out-dir samples --n 6 --bundles 2
```

### Validate config
```bash
clinote validate --config clinote.toml
```

## Example
**Before (input)**
```text
CC - chest pain
HPI: started after exercise
PMH: HTN, asthma
Assessment: likely MSK strain
Plan: NSAIDs, follow-up
```

**After (JSON)**
```json
{
  "format": "hp",
  "sections": [
    {"name": "Chief Complaint", "content": "chest pain"},
    {"name": "HPI", "content": "started after exercise"},
    {"name": "PMH", "content": "HTN, asthma"},
    {"name": "Assessment", "content": "likely MSK strain"},
    {"name": "Plan", "content": "NSAIDs, follow-up"}
  ]
}
```

## Bundle mode (complex bundles)
Bundled files are tricky because delimiters can be ambiguous and formats can be mixed. Clinote mitigates this by:
- Splitting only on explicit delimiters or repeated timestamps in auto mode.
- Warning when bundle mode is forced but no clear split is found.
- Allowing interactive review to remove or rename sections.
- Capturing warnings in JSON output and batch reports.

## Generate samples
Use `clinote sample` to generate synthetic notes plus gold JSON outputs in a folder. Bundle files are also generated if `--bundles` is provided.

## GitHub Pages deployment (/docs)
1. Build the static website in `docs/` (already provided).
2. In GitHub repo settings, enable Pages from branch `main` and folder `/docs`.
3. Confirm your Pages URL works: `https://<your-username>.github.io/<repo-name>/`.

## Release workflow
Trigger a GitHub Release by tagging and pushing:
```bash
git tag v0.1.0
git push --tags
```
Artifacts will appear under GitHub Releases. The GitHub Pages binary remains separate for the exam requirement.

## Binary hosting for downloads
Place compiled binaries in `docs/downloads/` so the download page can serve them. Required file:
- `docs/downloads/clinote-aarch64-linux`

Optional:
- `docs/downloads/clinote-x86_64-linux`
- `docs/downloads/clinote-macos`
- `docs/downloads/clinote-windows.exe`

## GitHub Repo Setup Checklist
- Set the GitHub repo **About** website to: `https://<your-username>.github.io/<repo-name>/`
- Description suggestion: "Deterministic CLI to structure synthetic clinical notes into SOAP/H&P/Discharge formats (Rust)."
- Topics to add: `rust`, `cli`, `health-informatics`, `clinical-notes`, `data-quality`, `serialization`, `deterministic`, `education`

## Business model
**Target users:** informatics students, clinical documentation QA teams, research labs, health data engineers.

**Pricing tiers:**
- Free/Student: full CLI with deterministic parsing.
- Pro Lab: batch templates + config review + priority support.
- Enterprise: on-prem pilots, compliance guidance, and training.

**Acquisition channels:** GitHub, university courses, informatics conferences, lab partnerships.

## Disclaimer
Clinote CLI is an educational tool using synthetic data only. It is not a medical device and does not guarantee legal or regulatory compliance.

## License
MIT
