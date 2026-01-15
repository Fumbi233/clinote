use crate::config::Config;
use crate::interactive;
use crate::models::{BundleMode, NoteFormat};
use crate::parser::{self, ParseOptions};
use crate::render::{self, OutputFormat};
use crate::reports::BatchReport;
use crate::samples;
use crate::util;
use anyhow::{anyhow, Result};
use clap::{Args, Parser, Subcommand};
use glob::glob;
use std::path::{Path, PathBuf};
use std::time::Instant;

#[derive(Parser)]
#[command(
    name = "clinote",
    version,
    about = "Clinote CLI: deterministic clinical note structuring"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Parse(ParseArgs),
    Batch(BatchArgs),
    Sample(SampleArgs),
    Validate(ValidateArgs),
}

#[derive(Args, Debug, Clone)]
pub struct ParseArgs {
    #[arg(long)]
    pub input: PathBuf,
    #[arg(long, value_enum)]
    pub format: NoteFormat,
    #[arg(long)]
    pub out: PathBuf,
    #[arg(long, value_enum)]
    pub out_format: OutputFormat,
    #[arg(long)]
    pub config: Option<PathBuf>,
    #[arg(long, value_enum)]
    pub bundle: Option<BundleMode>,
    #[arg(long)]
    pub interactive: bool,
}

#[derive(Args, Debug, Clone)]
pub struct BatchArgs {
    #[arg(long)]
    pub input_dir: PathBuf,
    #[arg(long)]
    pub glob: Option<String>,
    #[arg(long, value_enum)]
    pub format: NoteFormat,
    #[arg(long)]
    pub out_dir: PathBuf,
    #[arg(long, value_enum)]
    pub out_format: OutputFormat,
    #[arg(long)]
    pub config: Option<PathBuf>,
    #[arg(long, value_enum)]
    pub bundle: Option<BundleMode>,
}

#[derive(Args, Debug, Clone)]
pub struct SampleArgs {
    #[arg(long)]
    pub out_dir: PathBuf,
    #[arg(long)]
    pub n: usize,
    #[arg(long)]
    pub bundles: Option<usize>,
}

#[derive(Args, Debug, Clone)]
pub struct ValidateArgs {
    #[arg(long)]
    pub config: PathBuf,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Parse(args) => run_parse(&args),
        Commands::Batch(args) => run_batch_command(&args),
        Commands::Sample(args) => run_sample(&args),
        Commands::Validate(args) => run_validate(&args),
    }
}

fn run_parse(args: &ParseArgs) -> Result<()> {
    let config = Config::load(args.config.as_deref())?;
    let input = util::read_to_string(&args.input)?;
    let bundle_mode = args.bundle.unwrap_or(config.bundle.mode_default);
    let (note_texts, bundle_warnings) = parser::split_bundle(&input, bundle_mode, &config);

    let apply_heuristics = if args.interactive {
        interactive::prompt_apply_heuristics()?
    } else {
        config.enable_fallback_heuristics
    };

    let mut notes = Vec::new();
    for (idx, note_text) in note_texts.iter().enumerate() {
        let (candidates, mut warnings) = parser::extract_candidates(
            note_text,
            args.format,
            &config,
            ParseOptions { apply_heuristics },
        );
        warnings.extend(bundle_warnings.clone());

        let selected = if args.interactive {
            interactive::review_sections(&candidates)?
        } else {
            candidates
        };

        let note = parser::build_note(
            selected,
            args.format,
            Some(args.input.display().to_string()),
            idx + 1,
            warnings,
        );
        notes.push(note);
    }

    let rendered = render::render_notes(&notes, args.out_format, config.csv.layout)?;
    util::write_string(&args.out, &rendered)?;
    Ok(())
}

fn run_batch_command(args: &BatchArgs) -> Result<()> {
    let config = Config::load(args.config.as_deref())?;
    let report = run_batch(args, &config)?;
    let report_path = args.out_dir.join("batch_report.json");
    report.write_to(&report_path)?;
    Ok(())
}

pub fn run_batch(args: &BatchArgs, config: &Config) -> Result<BatchReport> {
    let start = Instant::now();
    let mut report = BatchReport::new("clinote");
    std::fs::create_dir_all(&args.out_dir)?;

    let glob_pattern = args
        .glob
        .clone()
        .unwrap_or_else(|| config.glob_default.clone());
    let pattern = args.input_dir.join(glob_pattern);
    let pattern_str = pattern
        .to_str()
        .ok_or_else(|| anyhow!("Invalid glob pattern"))?
        .to_string();

    let bundle_mode = args.bundle.unwrap_or(config.bundle.mode_default);

    for entry in glob(&pattern_str)? {
        match entry {
            Ok(path) => {
                let file_result = process_file(&path, args, config, bundle_mode);
                match file_result {
                    Ok(notes) => {
                        report.record_ok(&notes);
                    }
                    Err(err) => {
                        report.record_failure(&path.display().to_string(), err.to_string());
                    }
                }
            }
            Err(err) => {
                report.record_failure("glob", err.to_string());
            }
        }
    }

    report.finalize();
    report.runtime_ms = start.elapsed().as_millis();
    Ok(report)
}

fn process_file(
    path: &Path,
    args: &BatchArgs,
    config: &Config,
    bundle_mode: BundleMode,
) -> Result<Vec<crate::models::StructuredNote>> {
    let content = util::read_to_string(path)?;
    let (note_texts, bundle_warnings) = parser::split_bundle(&content, bundle_mode, config);
    let mut notes = Vec::new();
    for (idx, note_text) in note_texts.iter().enumerate() {
        let (candidates, mut warnings) = parser::extract_candidates(
            note_text,
            args.format,
            config,
            ParseOptions {
                apply_heuristics: config.enable_fallback_heuristics,
            },
        );
        warnings.extend(bundle_warnings.clone());
        let note = parser::build_note(
            candidates,
            args.format,
            Some(path.display().to_string()),
            idx + 1,
            warnings,
        );
        notes.push(note);
    }

    let rendered = render::render_notes(&notes, args.out_format, config.csv.layout)?;
    let stem = util::file_stem(path);
    let out_path = args
        .out_dir
        .join(format!("{}.{}", stem, args.out_format.extension()));
    util::write_string(&out_path, &rendered)?;
    Ok(notes)
}

fn run_sample(args: &SampleArgs) -> Result<()> {
    samples::generate_samples(&args.out_dir, args.n, args.bundles.unwrap_or(0))
}

fn run_validate(args: &ValidateArgs) -> Result<()> {
    let config = Config::load(Some(&args.config))?;
    println!("{}", config.summary());
    Ok(())
}
