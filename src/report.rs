use crate::cli::Args;
use crate::models::Analysis;
use colored::Colorize;

pub fn bar(earned: u32, max: u32) -> String {
    let width = 20usize;
    let filled = if max == 0 {
        0
    } else {
        (earned as usize * width) / max as usize
    };
    format!("{}{}", "█".repeat(filled), "░".repeat(width - filled))
}

pub fn print_report(a: &Analysis, args: &Args) {
    let line = "═".repeat(64);
    println!("{}", line.blue());
    println!("{}", format!(" ATS VIEW OF: {}", a.file).bold());
    println!("{}", line.blue());

    // ---- Score headline ----
    let score_str = format!("{}/100  {}", a.score.total, a.score.grade);
    let colored_score = match a.score.total {
        75..=100 => score_str.green().bold(),
        50..=74 => score_str.yellow().bold(),
        _ => score_str.red().bold(),
    };
    println!("\n{} {}", "SCORE:".bold(), colored_score);
    for item in &a.score.breakdown {
        println!(
            "  {:<24} {} {:>2}/{:<2}  {}",
            item.category,
            bar(item.earned, item.max),
            item.earned,
            item.max,
            item.reason.dimmed()
        );
    }

    // ---- Structure ----
    println!("\n{}", "── DOCUMENT ──".bold());
    println!(
        "  Pages: {}   Words: {}   Chars: {}   Images: {}   Encrypted: {}",
        a.structure.page_count,
        a.structure.total_words,
        a.structure.total_chars,
        a.structure.image_count,
        if a.structure.encrypted {
            "yes".red()
        } else {
            "no".green()
        }
    );
    println!("  Extractor: {}", a.extractor);

    // ---- Pages ----
    println!("\n{}", "── PER-PAGE EXTRACTION ──".bold());
    if a.pages.is_empty() {
        println!("  (no per-page data available)");
    }
    for p in &a.pages {
        let status = match p.status.as_str() {
            "ok" => "ok".green(),
            "empty" => "empty".red(),
            _ => "suspicious".yellow(),
        };
        println!(
            "  Page {}: {} words, {} lines, {:.0}% short lines, {} odd glyphs, {} image(s) [{}]",
            p.page,
            p.words,
            p.lines,
            p.short_line_ratio * 100.0,
            p.weird_glyphs,
            p.images,
            status
        );
        if args.previews {
            let preview: Vec<&str> = p
                .text
                .lines()
                .filter(|l| !l.trim().is_empty())
                .take(8)
                .collect();
            for l in preview {
                println!("      {}", l.dimmed());
            }
            println!();
        }
    }

    // ---- Contact ----
    println!("\n{}", "── CONTACT ──".bold());
    let show = |label: &str, v: &Option<String>| match v {
        Some(s) => println!("  {:<10} {}", label, s.green()),
        None => println!("  {:<10} {}", label, "NOT FOUND".red()),
    };
    show("Name:", &a.contact.name_guess);
    show("Email:", &a.contact.email);
    show("Phone:", &a.contact.phone);
    show("LinkedIn:", &a.contact.linkedin);
    show("GitHub:", &a.contact.github);
    if !a.contact.urls.is_empty() {
        println!("  URLs:      {}", a.contact.urls.join(", "));
    }

    // ---- Sections ----
    println!("\n{}", "── SECTIONS ──".bold());
    for s in &a.sections {
        if s.found {
            println!("  {} {:<16} ({} lines)", "✓".green(), s.name, s.line_count);
        } else {
            println!("  {} {:<16}", "✗".dimmed(), s.name.dimmed());
        }
    }

    // ---- Dates ----
    println!("\n{}", "── DATE RANGES ──".bold());
    if a.date_ranges.is_empty() {
        println!("  {}", "none found".red());
    } else {
        for d in &a.date_ranges {
            println!("  {}", d);
        }
    }

    // ---- Skills ----
    println!("\n{}", "── SKILLS DETECTED ──".bold());
    if a.skills.categories.is_empty() {
        println!("  {}", "no known skill keywords found".red());
    } else {
        println!(
            "  {} skills total, {} frontend",
            a.skills.total, a.skills.frontend_count
        );
        for cat in &a.skills.categories {
            println!("  {}", cat.category.cyan());
            let list: Vec<String> = cat
                .skills
                .iter()
                .map(|h| {
                    if h.mentions > 1 {
                        format!("{} (×{})", h.name, h.mentions)
                    } else {
                        h.name.clone()
                    }
                })
                .collect();
            println!("    {}", list.join(", "));
        }
    }

    // ---- Job match ----
    if let Some(j) = &a.job_match {
        println!("\n{}", "── JOB DESCRIPTION MATCH ──".bold());
        let pct = format!("{}%", j.match_percent);
        let pct = match j.match_percent {
            70..=100 => pct.green().bold(),
            40..=69 => pct.yellow().bold(),
            _ => pct.red().bold(),
        };
        println!(
            "  Skill match: {} ({} of {} required skills)",
            pct,
            j.matched.len(),
            j.required_skills.len()
        );
        if !j.matched.is_empty() {
            println!("  {} {}", "Matched:".green(), j.matched.join(", "));
        }
        if !j.missing.is_empty() {
            println!("  {} {}", "Missing:".red(), j.missing.join(", "));
        }
        if !j.other_missing_keywords.is_empty() {
            println!(
                "  {} {}",
                "Other JD keywords not in resume:".yellow(),
                j.other_missing_keywords.join(", ")
            );
        }
    }

    // ---- Findings ----
    println!("\n{}", "── FINDINGS ──".bold());
    if a.findings.is_empty() {
        println!("  {}", "No issues detected.".green());
    }
    for f in &a.findings {
        match f.severity.as_str() {
            "error" => println!("  {} {}", "✗".red().bold(), f.message.red()),
            "warning" => println!("  {} {}", "⚠".yellow().bold(), f.message.yellow()),
            _ => println!("  {} {}", "ℹ".cyan(), f.message),
        }
    }

    // ---- Raw text ----
    if args.raw {
        println!(
            "\n{}",
            "── RAW EXTRACTED TEXT (what the ATS ingests) ──".bold()
        );
        for p in &a.pages {
            println!("{}", format!("[page {}]", p.page).dimmed());
            println!("{}", p.text);
            println!();
        }
        if a.pages.iter().all(|p| p.text.trim().is_empty()) && !a.full_text.is_empty() {
            println!("{}", a.full_text);
        }
    }

    println!("\n{}", line.blue());
}

#[cfg(test)]
mod tests {
    use super::bar;

    fn filled(bar: &str) -> usize {
        bar.chars().filter(|c| *c == '█').count()
    }

    #[test]
    fn bar_scales_to_twenty_columns() {
        assert_eq!(filled(&bar(0, 20)), 0);
        assert_eq!(filled(&bar(10, 20)), 10);
        assert_eq!(filled(&bar(20, 20)), 20);
        assert_eq!(filled(&bar(24, 30)), 16);
        assert_eq!(bar(0, 20).chars().count(), 20);
    }

    #[test]
    fn bar_with_zero_max_is_empty() {
        assert_eq!(filled(&bar(5, 0)), 0);
    }
}
