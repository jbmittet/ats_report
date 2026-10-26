//! End-to-end tests: they spawn the real binary, which runs the real pipeline
//! (regexes, scoring, checks and PDFium extraction) on generated PDFs.

mod common;

use common::{TempFile, TempPdf};
use std::process::{Command, Output};

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ats-inspector"))
}

fn stdout(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn stderr(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

#[test]
fn help_documents_every_flag() {
    let out = bin().arg("--help").output().expect("run binary");
    assert!(out.status.success());
    let help = stdout(&out);
    for flag in [
        "--raw",
        "--previews",
        "--job",
        "--json",
        "--text-only",
        "--quiet",
        "--fail-under",
        "--version",
    ] {
        assert!(help.contains(flag), "help is missing {flag}:\n{help}");
    }
    assert!(help.contains("EXAMPLES:"));
}

#[test]
fn version_prints_the_package_version() {
    let out = bin().arg("--version").output().expect("run binary");
    assert!(out.status.success());
    let version = stdout(&out);
    assert!(version.contains(env!("CARGO_PKG_VERSION")), "{version}");
}

#[test]
fn default_run_prints_a_human_report() {
    let pdf = TempPdf::sample();
    let out = bin().arg(pdf.path()).output().expect("run binary");

    assert!(out.status.success(), "stderr: {}", stderr(&out));
    let report = stdout(&out);
    assert!(report.contains("SCORE:"));
    assert!(report.contains("── CONTACT ──"));
    assert!(report.contains("── SECTIONS ──"));
    assert!(report.contains("John Doe"));
    assert!(report.contains("Experience"));
}

#[test]
fn json_to_stdout_is_only_json() {
    let pdf = TempPdf::sample();
    let out = bin()
        .arg(pdf.path())
        .args(["--json", "-"])
        .output()
        .expect("run binary");

    assert!(out.status.success(), "stderr: {}", stderr(&out));
    let text = stdout(&out);
    assert!(!text.contains("SCORE:"), "report leaked into JSON output");

    let json: serde_json::Value = serde_json::from_str(&text).expect("stdout must be valid JSON");
    assert_eq!(json["extractor"], "PDFium");
    assert_eq!(json["structure"]["page_count"], 1);
    assert!(json["score"]["total"].as_u64().unwrap() > 0);
    let full_text = json["full_text"].as_str().unwrap();
    assert!(full_text.contains("John Doe"), "{full_text}");
    assert!(full_text.contains("Kubernetes"), "{full_text}");
}

#[test]
fn json_with_no_value_also_goes_to_stdout() {
    let pdf = TempPdf::sample();
    let out = bin()
        .arg(pdf.path())
        .arg("--json")
        .output()
        .expect("run binary");

    assert!(out.status.success(), "stderr: {}", stderr(&out));
    let json: serde_json::Value =
        serde_json::from_str(&stdout(&out)).expect("stdout must be valid JSON");
    assert_eq!(json["structure"]["page_count"], 1);
}

#[test]
fn text_only_writes_plain_text() {
    let pdf = TempPdf::sample();
    let out = bin()
        .arg(pdf.path())
        .arg("--text-only")
        .output()
        .expect("run binary");

    assert!(out.status.success(), "stderr: {}", stderr(&out));
    let text = stdout(&out);
    assert!(text.contains("John Doe"), "{text}");
    assert!(text.contains("EXPERIENCE"), "{text}");
    assert!(!text.contains("SCORE:"), "report leaked into text output");
    assert!(stderr(&out).is_empty(), "stderr: {}", stderr(&out));
}

#[test]
fn text_only_with_json_file_keeps_stdout_plain() {
    let pdf = TempPdf::sample();
    let json_path = TempFile::empty_named("json");
    let out = bin()
        .arg(pdf.path())
        .arg("--text-only")
        .arg("--json")
        .arg(json_path.path())
        .output()
        .expect("run binary");

    assert!(out.status.success(), "stderr: {}", stderr(&out));
    let text = stdout(&out);
    assert!(text.contains("John Doe"));
    assert!(
        !text.contains("JSON report written"),
        "status message polluted stdout: {text}"
    );
    let json = std::fs::read_to_string(json_path.path()).expect("json file written");
    serde_json::from_str::<serde_json::Value>(&json).expect("valid JSON file");
}

#[test]
fn raw_conflicts_with_quiet() {
    let pdf = TempPdf::sample();
    let out = bin()
        .arg(pdf.path())
        .args(["--raw", "--quiet"])
        .output()
        .expect("run binary");

    assert_eq!(out.status.code(), Some(2));
    assert!(stderr(&out).contains("cannot be used with"));
}

#[test]
fn raw_conflicts_with_json_to_stdout() {
    let pdf = TempPdf::sample();
    let out = bin()
        .arg(pdf.path())
        .args(["--json", "-", "--raw"])
        .output()
        .expect("run binary");

    assert_eq!(out.status.code(), Some(2));
    assert!(stderr(&out).contains("human-readable report"));
}

#[test]
fn text_only_conflicts_with_json_to_stdout() {
    let pdf = TempPdf::sample();
    let out = bin()
        .arg(pdf.path())
        .args(["--text-only", "--json", "-"])
        .output()
        .expect("run binary");

    assert_eq!(out.status.code(), Some(2));
    assert!(stderr(&out).contains("both write to stdout"));
}

#[test]
fn fail_under_sets_the_exit_code() {
    let sample = TempPdf::sample();
    let out = bin()
        .arg(sample.path())
        .args(["--quiet", "--fail-under", "1"])
        .output()
        .expect("run binary");
    assert!(out.status.success(), "stderr: {}", stderr(&out));

    let tiny = TempPdf::tiny();
    let out = bin()
        .arg(tiny.path())
        .args(["--quiet", "--fail-under", "50"])
        .output()
        .expect("run binary");
    assert_eq!(out.status.code(), Some(1));
    assert!(stderr(&out).contains("below the required minimum"));
}

#[test]
fn fail_under_rejects_out_of_range_values() {
    let pdf = TempPdf::sample();
    let out = bin()
        .arg(pdf.path())
        .args(["--fail-under", "101"])
        .output()
        .expect("run binary");
    assert_eq!(out.status.code(), Some(2));
    assert!(stderr(&out).contains("not in 0..=100"));
}

#[test]
fn missing_job_file_is_an_error() {
    let pdf = TempPdf::sample();
    let missing = TempFile::empty_named("txt");
    let out = bin()
        .arg(pdf.path())
        .args(["--quiet", "--job"])
        .arg(missing.path())
        .output()
        .expect("run binary");

    assert_eq!(out.status.code(), Some(1));
    assert!(stderr(&out).contains("could not read job description"));
}

#[test]
fn job_description_match_is_reported() {
    let pdf = TempPdf::sample();
    let jd = TempFile::with_text("We need Rust, Kubernetes, Docker and GraphQL.", "txt");
    let out = bin()
        .arg(pdf.path())
        .arg("--job")
        .arg(jd.path())
        .output()
        .expect("run binary");

    assert!(out.status.success(), "stderr: {}", stderr(&out));
    let report = stdout(&out);
    assert!(report.contains("JOB DESCRIPTION MATCH"));
    assert!(report.contains("Missing:"));
}

#[test]
fn nonexistent_pdf_is_an_error() {
    let missing = TempFile::empty_named("pdf");
    let out = bin().arg(missing.path()).output().expect("run binary");
    assert_eq!(out.status.code(), Some(1));
    assert!(stderr(&out).contains("Cannot open PDF"));
}

#[test]
fn previews_add_the_first_lines_of_each_page() {
    let pdf = TempPdf::sample();

    let without = stdout(&bin().arg(pdf.path()).output().expect("run binary"));
    assert!(!without.lines().any(|l| l.trim() == "John Doe"));

    let with = stdout(
        &bin()
            .arg(pdf.path())
            .arg("--previews")
            .output()
            .expect("run binary"),
    );
    assert!(with.contains("PER-PAGE EXTRACTION"));
    assert!(
        with.lines().any(|l| l.trim() == "John Doe"),
        "expected a page preview line:\n{with}"
    );
}
