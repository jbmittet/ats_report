use crate::models::{Analysis, Score, ScoreItem};
use crate::regexes::{SPACED_HEADER_RE, WEIRD_GLYPH_RE};

pub fn compute_score(a: &mut Analysis) {
    let mut items: Vec<ScoreItem> = vec![];
    let mut push = |category: &str, earned: u32, max: u32, reason: String| {
        items.push(ScoreItem {
            category: category.into(),
            earned: earned.min(max),
            max,
            reason,
        });
    };

    // 1. Text extractability (30)
    {
        let words = a.structure.total_words;
        let (pts, reason) = if words < 30 {
            (
                0,
                "No usable text extracted (scanned/image PDF?)".to_string(),
            )
        } else if words < 150 {
            (12, format!("Only {words} words extracted (very thin)"))
        } else {
            let bad_pages = a.pages.iter().filter(|p| p.status == "empty").count();
            if bad_pages > 0 {
                (
                    22,
                    format!("{words} words, but {bad_pages} page(s) yielded no text"),
                )
            } else {
                (30, "Text extracted cleanly".to_string())
            }
        };
        let mut pts: u32 = pts;
        if a.structure.encrypted {
            pts = pts.saturating_sub(5);
        }
        push("Text extractability", pts, 30, reason);
    }

    // 2. Layout / reading order (20)
    {
        let lines: Vec<&str> = a
            .full_text
            .lines()
            .filter(|l| !l.trim().is_empty())
            .collect();
        let short = lines
            .iter()
            .filter(|l| l.split_whitespace().count() <= 2)
            .count();
        let ratio = if lines.is_empty() {
            1.0
        } else {
            short as f32 / lines.len() as f32
        };
        let glyphs = WEIRD_GLYPH_RE.find_iter(&a.full_text).count();
        let spaced = lines
            .iter()
            .filter(|l| SPACED_HEADER_RE.is_match(l.trim()))
            .count();
        let suspicious_pages = a.pages.iter().filter(|p| p.status == "suspicious").count();

        let mut pts: i32 = 20;
        let mut notes = vec![];
        if ratio > 0.5 {
            pts -= 10;
            notes.push(format!(
                "{:.0}% very short lines (columns/tables?)",
                ratio * 100.0
            ));
        } else if ratio > 0.4 {
            pts -= 6;
            notes.push(format!("{:.0}% short lines", ratio * 100.0));
        }
        if glyphs > 5 {
            pts -= 5;
            notes.push(format!("{glyphs} odd glyphs"));
        }
        if spaced > 0 {
            pts -= 3;
            notes.push("letter-spaced headers".into());
        }
        if suspicious_pages > 0 {
            pts -= 2 * suspicious_pages as i32;
            notes.push(format!("{suspicious_pages} suspicious page(s)"));
        }
        if a.structure.image_count > 3 {
            pts -= 2;
            notes.push(format!("{} images", a.structure.image_count));
        }

        let reason = if notes.is_empty() {
            "Linear, clean reading order".into()
        } else {
            notes.join("; ")
        };
        push("Layout & reading order", pts.max(0) as u32, 20, reason);
    }

    // 3. Contact info (15)
    {
        let mut pts = 0;
        let mut notes = vec![];
        if a.contact.email.is_some() {
            pts += 6
        } else {
            notes.push("no email")
        }
        if a.contact.phone.is_some() {
            pts += 4
        } else {
            notes.push("no phone")
        }
        if a.contact.name_guess.is_some() {
            pts += 3
        } else {
            notes.push("name not identified")
        }
        if a.contact.linkedin.is_some() || a.contact.github.is_some() {
            pts += 2
        } else {
            notes.push("no LinkedIn/GitHub")
        }
        let reason = if notes.is_empty() {
            "All contact fields found".into()
        } else {
            format!("Missing: {}", notes.join(", "))
        };
        push("Contact info", pts, 15, reason);
    }

    // 4. Section structure (15)
    {
        let found = |n: &str| a.sections.iter().any(|s| s.name == n && s.found);
        let mut pts = 0;
        let mut notes = vec![];
        if found("Experience") {
            pts += 6
        } else {
            notes.push("Experience")
        }
        if found("Education") {
            pts += 4
        } else {
            notes.push("Education")
        }
        if found("Skills") {
            pts += 4
        } else {
            notes.push("Skills")
        }
        if found("Summary") || found("Projects") {
            pts += 1
        }
        let reason = if notes.is_empty() {
            "Key sections recognized".into()
        } else {
            format!("Missing headers: {}", notes.join(", "))
        };
        push("Section structure", pts, 15, reason);
    }

    // 5. Dates (10)
    {
        let n = a.date_ranges.len();
        let (pts, reason) = match n {
            0 => (0, "No date ranges found".to_string()),
            1 => (4, "Only 1 date range found".to_string()),
            2 => (7, "2 date ranges found".to_string()),
            _ => (10, format!("{n} date ranges found")),
        };
        push("Dates", pts, 10, reason);
    }

    // 6. Content & length (10)
    {
        let mut pts: i32 = 0;
        let mut notes = vec![];
        let n = a.skills.total;
        let cats = a.skills.categories.len();

        match n {
            0 => notes.push("no known skill keywords".to_string()),
            1..=4 => {
                pts += 3;
                notes.push(format!("{n} skills (few)"))
            }
            5..=9 => {
                pts += 4;
                notes.push(format!("{n} skills"))
            }
            10..=19 => {
                pts += 5;
                notes.push(format!("{n} skills across {cats} categories"))
            }
            _ => {
                pts += 6;
                notes.push(format!("{n} skills across {cats} categories"))
            }
        }
        match a.structure.page_count {
            1..=2 => {
                pts += 4;
                notes.push("good length".into())
            }
            3 => {
                pts += 2;
                notes.push("3 pages (long)".into())
            }
            _ => notes.push("too long".into()),
        }
        push("Content & length", pts.max(0) as u32, 10, notes.join("; "));
    }

    let total: u32 = items.iter().map(|i| i.earned).sum();
    a.score = Score {
        total,
        grade: grade_for(total).into(),
        breakdown: items,
    };
}

/// Letter grade shown next to the numeric score.
pub fn grade_for(total: u32) -> &'static str {
    match total {
        90..=100 => "A: ATS-friendly",
        75..=89 => "B: Mostly fine, minor issues",
        60..=74 => "C: Risky, several problems",
        40..=59 => "D: Poor, significant content may be lost",
        _ => "F: ATS will likely fail to parse this",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checks::extract_date_ranges;
    use crate::contact::extract_contact;
    use crate::models::{Analysis, PageInfo, Structure};
    use crate::sections::detect_sections;
    use crate::skills::detect_skills;

    const PERFECT_RESUME: &str = "\
    John Doe
    john.doe@example.com | +1 555 010-9999 | linkedin.com/in/johndoe

    SUMMARY
    Senior software engineer with ten years of experience building web applications and distributed systems for large teams across many industries and regions.

    EXPERIENCE
    Senior Software Engineer, Acme Corp, Jan 2020 - Present
    Led migration of twelve services to Kubernetes, reducing deployment time by sixty percent and improving reliability across the platform.
    Mentored four engineers and introduced code review standards that raised overall code quality and reduced production incidents.
    Software Engineer, Beta Inc, Mar 2018 - Dec 2019
    Built REST APIs in Python and Go serving two million requests per day for customers around the world.
    Designed and implemented a caching layer with Redis that cut response times significantly for the most popular endpoints.

    EDUCATION
    BSc Computer Science, University of Somewhere, Sept 2014 - Jun 2017

    SKILLS
    HTML, CSS, JavaScript, TypeScript, React, Next.js, Vue, Node.js, Express, Python, Django, Go, Rust, PostgreSQL, MySQL, MongoDB, Redis, Docker, Kubernetes, AWS, Terraform, Git, CI/CD, Jest, Playwright, Agile, Leadership, Communication
    ";

    /// Builds an Analysis the same way main.rs does, then scores it.
    fn analyze(text: &str) -> Analysis {
        let mut a = Analysis {
            full_text: text.into(),
            structure: Structure {
                page_count: 1,
                total_words: text.split_whitespace().count(),
                total_chars: text.chars().count(),
                ..Default::default()
            },
            contact: extract_contact(text),
            sections: detect_sections(text),
            date_ranges: extract_date_ranges(text),
            skills: detect_skills(text),
            ..Default::default()
        };
        compute_score(&mut a);
        a
    }

    fn page(status: &str) -> PageInfo {
        PageInfo {
            page: 1,
            words: 50,
            lines: 5,
            short_line_ratio: 0.1,
            weird_glyphs: 0,
            images: 0,
            status: status.into(),
            text: String::new(),
        }
    }

    #[test]
    fn a_clean_resume_scores_one_hundred() {
        let a = analyze(PERFECT_RESUME);
        assert_eq!(a.score.total, 100, "breakdown: {:?}", a.score.breakdown);
        assert_eq!(a.score.grade, "A: ATS-friendly");
        assert_eq!(a.score.breakdown.len(), 6);
        assert_eq!(
            a.score
                .breakdown
                .iter()
                .map(|i| i.category.as_str())
                .collect::<Vec<_>>(),
            vec![
                "Text extractability",
                "Layout & reading order",
                "Contact info",
                "Section structure",
                "Dates",
                "Content & length",
            ]
        );
        let sum: u32 = a.score.breakdown.iter().map(|i| i.earned).sum();
        assert_eq!(sum, a.score.total);
        for item in &a.score.breakdown {
            assert!(item.earned <= item.max, "{item:?}");
        }
    }

    #[test]
    fn thin_text_earns_partial_extractability_points() {
        let a = analyze(&"word ".repeat(100));
        assert_eq!(a.score.breakdown[0].earned, 12);
        assert!(a.score.breakdown[0].reason.contains("Only 100 words"));
    }

    #[test]
    fn no_text_at_all_earns_zero_extractability() {
        let a = analyze("short text only");
        assert_eq!(a.score.breakdown[0].earned, 0);
        assert!(a.score.breakdown[0].reason.contains("No usable text"));
    }

    #[test]
    fn empty_document_stays_at_the_floor() {
        let mut a = Analysis::default();
        compute_score(&mut a);
        assert_eq!(a.score.total, 10);
        assert_eq!(a.score.grade, "F: ATS will likely fail to parse this");
    }

    #[test]
    fn encryption_costs_five_extractability_points() {
        let mut a = analyze(PERFECT_RESUME);
        a.structure.encrypted = true;
        compute_score(&mut a);
        assert_eq!(a.score.breakdown[0].earned, 25);
        assert_eq!(a.score.total, 95);
    }

    #[test]
    fn a_page_without_text_costs_eight_extractability_points() {
        let mut a = analyze(PERFECT_RESUME);
        a.pages.push(page("empty"));
        compute_score(&mut a);
        assert_eq!(a.score.breakdown[0].earned, 22);
        assert!(a.score.breakdown[0]
            .reason
            .contains("1 page(s) yielded no text"));
    }

    #[test]
    fn suspicious_pages_and_images_reduce_the_layout_score() {
        let mut a = analyze(PERFECT_RESUME);
        a.pages.push(page("suspicious"));
        a.structure.image_count = 4;
        compute_score(&mut a);
        assert_eq!(a.score.breakdown[1].earned, 16);
        assert!(a.score.breakdown[1].reason.contains("1 suspicious page(s)"));
        assert!(a.score.breakdown[1].reason.contains("4 images"));
    }

    #[test]
    fn grades_follow_the_documented_boundaries() {
        assert_eq!(grade_for(100), "A: ATS-friendly");
        assert_eq!(grade_for(90), "A: ATS-friendly");
        assert_eq!(grade_for(89), "B: Mostly fine, minor issues");
        assert_eq!(grade_for(75), "B: Mostly fine, minor issues");
        assert_eq!(grade_for(74), "C: Risky, several problems");
        assert_eq!(grade_for(60), "C: Risky, several problems");
        assert_eq!(grade_for(59), "D: Poor, significant content may be lost");
        assert_eq!(grade_for(40), "D: Poor, significant content may be lost");
        assert_eq!(grade_for(39), "F: ATS will likely fail to parse this");
        assert_eq!(grade_for(0), "F: ATS will likely fail to parse this");
    }
}
