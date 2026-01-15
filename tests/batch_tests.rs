use clinote::cli::{run_batch, BatchArgs};
use clinote::config::Config;
use clinote::models::NoteFormat;
use clinote::render::OutputFormat;
use std::fs;

#[test]
fn batch_continues_on_failure() {
    let temp_dir = std::env::temp_dir().join("clinote_batch_test");
    let input_dir = temp_dir.join("in");
    let out_dir = temp_dir.join("out");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&input_dir).unwrap();
    fs::create_dir_all(&out_dir).unwrap();

    fs::write(input_dir.join("good.txt"), "Subjective:\nAll good").unwrap();
    fs::write(input_dir.join("bad.txt"), [0xff]).unwrap();

    let args = BatchArgs {
        input_dir: input_dir.clone(),
        glob: Some("*.txt".to_string()),
        format: NoteFormat::Soap,
        out_dir: out_dir.clone(),
        out_format: OutputFormat::Json,
        config: None,
        bundle: None,
    };

    let report = run_batch(&args, &Config::default()).unwrap();
    assert_eq!(report.ok_files, 1);
    assert_eq!(report.failed_files, 1);
    assert_eq!(report.failures.len(), 1);
    assert!(out_dir.join("good.json").exists());

    let _ = fs::remove_dir_all(&temp_dir);
}
