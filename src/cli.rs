use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "ats-inspector",
    version,
    about = "See your PDF resume the way an ATS does",
    after_help = "EXAMPLES:\n  ats-inspector resume.pdf\n  ats-inspector resume.pdf --job posting.txt --json report.json\n  ats-inspector resume.pdf --json | jq .score\n  ats-inspector resume.pdf --text-only > resume.txt"
)]
pub struct Args {
    /// Path to the PDF file
    pub pdf: PathBuf,

    /// Include the full extracted text, page by page, in the report
    #[arg(short, long, conflicts_with = "quiet")]
    pub raw: bool,

    /// Include a short preview of each page in the report
    #[arg(long, conflicts_with = "quiet")]
    pub previews: bool,

    /// Optional job description file (plain text) to compute keyword match
    #[arg(short, long)]
    pub job: Option<PathBuf>,

    /// Write JSON report to FILE ("-" or no value = stdout, which implies --quiet)
    #[arg(long, value_name = "FILE", num_args = 0..=1, default_missing_value = "-")]
    pub json: Option<PathBuf>,

    /// Print the extracted text only, nothing else (implies --quiet)
    #[arg(long = "text-only", conflicts_with_all = ["raw", "previews"])]
    pub text_only: bool,

    /// Suppress the human-readable report
    #[arg(short, long)]
    pub quiet: bool,

    /// Exit non-zero if the score is below this
    #[arg(
        long,
        value_name = "SCORE",
        value_parser = clap::value_parser!(u8).range(0..=100)
    )]
    pub fail_under: Option<u8>,
}
