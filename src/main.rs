mod checks;
mod cli;
mod contact;
mod extract;
mod job;
mod models;
mod regexes;
mod report;
mod scoring;
mod sections;
mod skills;

// Vendor
use clap::Parser;
use colored::*;
use std::path::Path;

// Project code
use checks::{extract_date_ranges, run_checks};
use cli::Args;
use contact::extract_contact;
use extract::structure::analyze_structure_and_pages;
use job::job_match;
use models::Analysis;
use report::print_report;
use scoring::compute_score;
use sections::detect_sections;
use skills::detect_skills;

fn main() {
    let args = Args::parse();

    let json_to_stdout = args.json.as_deref() == Some(Path::new("-"));
    if args.text_only && json_to_stdout {
        eprintln!(
            "{} --text-only and --json - both write to stdout; send the JSON to a file instead",
            "error:".red().bold()
        );
        std::process::exit(2);
    }
    if json_to_stdout && (args.raw || args.previews) {
        eprintln!(
            "{} --raw/--previews are part of the human-readable report, which --json - suppresses",
            "error:".red().bold()
        );
        std::process::exit(2);
    }

    let mut a = Analysis {
        file: args.pdf.display().to_string(),
        ..Default::default()
    };

    if let Err(e) = analyze_structure_and_pages(&args.pdf, &mut a) {
        eprintln!("{} {}", "error:".red().bold(), e);
        std::process::exit(1);
    }

    a.contact = extract_contact(&a.full_text);
    a.sections = detect_sections(&a.full_text);
    a.date_ranges = extract_date_ranges(&a.full_text);
    a.skills = detect_skills(&a.full_text);

    if let Some(jd_path) = &args.job {
        match std::fs::read_to_string(jd_path) {
            Ok(jd) => a.job_match = Some(job_match(&jd, &a.skills, &a.full_text)),
            Err(e) => {
                eprintln!(
                    "{} could not read job description '{}': {}",
                    "error:".red().bold(),
                    jd_path.display(),
                    e
                );
                std::process::exit(1);
            }
        }
    }

    run_checks(&mut a);
    compute_score(&mut a);

    if args.text_only {
        let private: std::collections::BTreeSet<String> = a
            .full_text
            .chars()
            .filter(|c| ('\u{E000}'..='\u{F8FF}').contains(c))
            .map(|c| format!("U+{:04X}", c as u32))
            .collect();
        use std::io::Write;
        let mut out = std::io::stdout();
        writeln!(out, "{}", a.full_text).ok();
        if !private.is_empty() {
            eprintln!(
                "Private-use characters found: {}",
                private.into_iter().collect::<Vec<_>>().join(", ")
            );
        }
    }

    let quiet = args.quiet || args.text_only || json_to_stdout;
    if !quiet {
        print_report(&a, &args);
    }

    if let Some(target) = &args.json {
        match serde_json::to_string_pretty(&a) {
            Ok(json) => {
                if target == Path::new("-") {
                    println!("{json}");
                } else if let Err(e) = std::fs::write(target, json) {
                    eprintln!(
                        "{} could not write JSON to '{}': {}",
                        "error:".red().bold(),
                        target.display(),
                        e
                    );
                    std::process::exit(1);
                } else if !quiet {
                    println!("JSON report written to {}", target.display());
                }
            }
            Err(e) => {
                eprintln!("{} could not serialize JSON: {}", "error:".red().bold(), e);
                std::process::exit(1);
            }
        }
    }

    if let Some(min) = args.fail_under {
        if a.score.total < u32::from(min) {
            eprintln!(
                "{} score {} is below the required minimum of {}",
                "error:".red().bold(),
                a.score.total,
                min
            );
            std::process::exit(1);
        }
    }
}
