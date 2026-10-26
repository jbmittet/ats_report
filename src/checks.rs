use crate::models::Analysis;
use crate::regexes::{DATE_RANGE_RE, SPACED_HEADER_RE, WEIRD_GLYPH_RE};

pub fn run_checks(a: &mut Analysis) {
    // Extractability
    if a.structure.encrypted {
        a.warn("PDF is encrypted/protected; some ATS parsers fail on protected files.");
    }
    if a.structure.total_words < 30 {
        if a.structure.image_count > 0 {
            a.error("Almost no text extracted but images are present: likely a scanned/image-only PDF. An ATS will see an empty resume.");
        } else {
            a.error("Almost no text could be extracted. An ATS will likely see an empty resume.");
        }
    } else if a.structure.image_count > 0 {
        a.info(format!(
            "{} embedded image(s) found. ATS ignores image content (logos, skill bars, icons, charts).",
            a.structure.image_count
        ));
    }

    // Per-page issues
    let page_issues: Vec<(usize, String, f32, usize)> = a
        .pages
        .iter()
        .filter(|p| p.status != "ok")
        .map(|p| (p.page, p.status.clone(), p.short_line_ratio, p.weird_glyphs))
        .collect();
    for (page, status, ratio, glyphs) in page_issues {
        match status.as_str() {
            "empty" => a.warn(format!("Page {page} yielded (almost) no text.")),
            _ => a.warn(format!(
                "Page {page} looks suspicious: {:.0}% very short lines, {} odd glyphs. Possible multi-column/table layout read out of order.",
                ratio * 100.0,
                glyphs
            )),
        }
    }

    // Layout (document-wide)
    let text = a.full_text.clone();
    let lines: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
    if lines.len() >= 10 {
        let short = lines
            .iter()
            .filter(|l| l.split_whitespace().count() <= 2)
            .count();
        let ratio = short as f32 / lines.len() as f32;
        if ratio > 0.4 {
            a.warn(format!(
                "{:.0}% of lines have 2 words or fewer: suggests multi-column/table layout being read out of order.",
                ratio * 100.0
            ));
        }
    }

    let spaced = lines
        .iter()
        .filter(|l| SPACED_HEADER_RE.is_match(l.trim()))
        .count();
    if spaced > 0 {
        a.warn(format!(
            "{spaced} letter-spaced header(s) detected (e.g. 'E X P E R I E N C E'). ATS won't recognize these as section titles."
        ));
    }

    let glyphs = WEIRD_GLYPH_RE.find_iter(&a.full_text).count();
    if glyphs > 5 {
        a.warn(format!(
            "{glyphs} private-use/unknown glyphs found (icon fonts or failed decoding). They appear as garbage to an ATS."
        ));
    }

    // Contact
    if a.contact.email.is_none() {
        a.error("No email address found. An ATS may be unable to create a candidate profile.");
    }
    if a.contact.phone.is_none() {
        a.warn("No phone number found in the header area.");
    }
    if a.contact.name_guess.is_none() {
        a.warn(
            "Could not identify a name in the first lines. Put your name alone on the first line.",
        );
    }
    // Contact info positioned too deep in the document?
    if let Some(email) = &a.contact.email {
        let head: String = a.full_text.lines().take(12).collect::<Vec<_>>().join("\n");
        if !head.contains(email.as_str()) {
            a.warn("Email is not in the first lines of the document; some parsers only scan the header area.");
        }
    }

    // Sections
    let missing: Vec<String> = a
        .sections
        .iter()
        .filter(|s| !s.found && matches!(s.name.as_str(), "Experience" | "Education" | "Skills"))
        .map(|s| s.name.clone())
        .collect();
    for m in missing {
        a.warn(format!(
            "No standard '{m}' header found; the ATS may not categorize that content."
        ));
    }

    if a.date_ranges.is_empty() {
        a.warn("No date ranges (e.g. 'Jan 2020 - Present') found; ATS can't compute years of experience.");
    }

    // Length
    if a.structure.page_count > 3 {
        a.warn(format!(
            "{} pages: unusually long for a resume.",
            a.structure.page_count
        ));
    }

    // Frontend-specific advice
    check_frontend_coverage(a);
}

fn check_frontend_coverage(a: &mut Analysis) {
    let fe_total = a.skills.frontend_count;
    if fe_total == 0 {
        return; // not a frontend resume; stay quiet
    }

    let has_cat = |cat: &str| a.skills.categories.iter().any(|c| c.category == cat);
    let has_skill = |name: &str| a.skills.all.iter().any(|s| s == name);

    let mut gaps = vec![];
    if !has_skill("HTML") {
        gaps.push("HTML");
    }
    if !(has_skill("CSS") || has_skill("Sass/SCSS") || has_skill("Tailwind CSS")) {
        gaps.push("CSS/styling");
    }
    if !(has_skill("JavaScript") || has_skill("TypeScript")) {
        gaps.push("JavaScript/TypeScript");
    }
    if !has_cat("Frontend: Frameworks & Libraries") {
        gaps.push("a framework (React/Vue/Angular...)");
    }
    if !has_cat("Frontend: Testing") {
        gaps.push("frontend testing tools");
    }

    let gap_msg = if gaps.is_empty() {
        None
    } else {
        Some(format!(
            "Frontend profile detected ({fe_total} FE skills) but no mention of: {}. ATS keyword filters often look for these explicitly.",
            gaps.join(", ")
        ))
    };

    let aliased: Vec<String> = a
        .skills
        .categories
        .iter()
        .flat_map(|c| c.skills.iter())
        .filter(|h| {
            matches!(
                h.name.as_str(),
                "React" | "Vue" | "Next.js" | "Node.js" | "Angular"
            ) && h.matched_as.contains("js")
        })
        .map(|h| h.name.clone())
        .collect();

    if let Some(m) = gap_msg {
        a.info(m);
    }
    if !aliased.is_empty() {
        a.info(format!(
            "Tip: ATS matching is literal. Consider listing both spellings once (e.g. 'React / React.js', 'Next.js / NextJS') for: {}.",
            aliased.join(", ")
        ));
    }
}

pub fn extract_date_ranges(text: &str) -> Vec<String> {
    DATE_RANGE_RE
        .find_iter(text)
        .map(|m| m.as_str().trim().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Contact, PageInfo, Structure};
    use crate::sections::detect_sections;
    use crate::skills::detect_skills;

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

    /// An analysis that should trigger no findings at all.
    fn clean() -> Analysis {
        Analysis {
            structure: Structure {
                page_count: 1,
                encrypted: false,
                image_count: 0,
                total_words: 200,
                total_chars: 1200,
            },
            pages: vec![page("ok")],
            contact: Contact {
                name_guess: Some("John Doe".into()),
                email: Some("john@example.com".into()),
                phone: Some("555 010 9999".into()),
                linkedin: Some("linkedin.com/in/john".into()),
                github: None,
                urls: vec![],
            },
            sections: detect_sections("SUMMARY\nx\nEXPERIENCE\nx\nEDUCATION\nx\nSKILLS\nx\n"),
            date_ranges: vec!["2020 - 2021".into(), "2018 - 2020".into()],
            skills: detect_skills(""),
            full_text: "John Doe\njohn@example.com\nSenior engineer\n".into(),
            ..Default::default()
        }
    }

    fn checked(mut a: Analysis) -> Analysis {
        run_checks(&mut a);
        a
    }

    fn has(a: &Analysis, needle: &str) -> bool {
        a.findings.iter().any(|f| f.message.contains(needle))
    }

    fn severity<'a>(a: &'a Analysis, needle: &str) -> Option<&'a str> {
        a.findings
            .iter()
            .find(|f| f.message.contains(needle))
            .map(|f| f.severity.as_str())
    }

    #[test]
    fn clean_resume_produces_no_findings() {
        let a = checked(clean());
        assert!(
            a.findings.is_empty(),
            "unexpected findings: {:?}",
            a.findings
        );
    }

    #[test]
    fn encrypted_pdf_warns() {
        let mut a = clean();
        a.structure.encrypted = true;
        let a = checked(a);
        assert_eq!(severity(&a, "encrypted/protected"), Some("warning"));
    }

    #[test]
    fn image_only_pdf_is_an_error() {
        let mut a = clean();
        a.structure.total_words = 10;
        a.structure.image_count = 2;
        let a = checked(a);
        assert_eq!(severity(&a, "scanned/image-only"), Some("error"));
    }

    #[test]
    fn almost_no_text_without_images_is_an_error() {
        let mut a = clean();
        a.structure.total_words = 10;
        let a = checked(a);
        assert_eq!(
            severity(&a, "Almost no text could be extracted"),
            Some("error")
        );
    }

    #[test]
    fn embedded_images_are_reported_as_info() {
        let mut a = clean();
        a.structure.image_count = 2;
        let a = checked(a);
        assert_eq!(severity(&a, "embedded image(s)"), Some("info"));
    }

    #[test]
    fn empty_and_suspicious_pages_warn() {
        let mut a = clean();
        let mut empty = page("empty");
        empty.page = 2;
        let mut suspicious = page("suspicious");
        suspicious.page = 3;
        suspicious.short_line_ratio = 0.8;
        suspicious.weird_glyphs = 9;
        a.pages = vec![page("ok"), empty, suspicious];
        let a = checked(a);
        assert!(has(&a, "Page 2 yielded"));
        assert!(has(&a, "Page 3 looks suspicious"));
    }

    #[test]
    fn short_lines_suggest_a_column_layout() {
        let mut a = clean();
        a.full_text = (0..10).map(|_| "one two\n").collect();
        let a = checked(a);
        assert!(has(&a, "2 words or fewer"));
    }

    #[test]
    fn letter_spaced_headers_warn() {
        let mut a = clean();
        a.full_text = "E X P E R I E N C E\nsome text\n".into();
        let a = checked(a);
        assert!(has(&a, "letter-spaced header(s)"));
    }

    #[test]
    fn many_private_use_glyphs_warn() {
        let mut a = clean();
        a.full_text = "\u{E000}\u{E001}\u{E002}\u{E003}\u{E004}\u{E005} text".into();
        let a = checked(a);
        assert!(has(&a, "private-use/unknown glyphs"));
    }

    #[test]
    fn missing_contact_fields_are_flagged() {
        let mut a = clean();
        a.contact.email = None;
        a.contact.phone = None;
        a.contact.name_guess = None;
        let a = checked(a);
        assert_eq!(severity(&a, "No email address"), Some("error"));
        assert_eq!(severity(&a, "No phone number"), Some("warning"));
        assert_eq!(severity(&a, "Could not identify a name"), Some("warning"));
    }

    #[test]
    fn email_deep_in_the_document_warns() {
        let mut a = clean();
        let mut text = String::new();
        for _ in 0..12 {
            text.push_str("some filler text here\n");
        }
        text.push_str("john@example.com\n");
        a.full_text = text;
        let a = checked(a);
        assert!(has(&a, "Email is not in the first lines"));
    }

    #[test]
    fn missing_key_sections_warn() {
        let mut a = clean();
        a.sections = detect_sections("SKILLS\nRust\n");
        let a = checked(a);
        assert!(has(&a, "No standard 'Experience' header"));
        assert!(has(&a, "No standard 'Education' header"));
        assert!(!has(&a, "No standard 'Skills' header"));
    }

    #[test]
    fn no_dates_and_long_documents_warn() {
        let mut a = clean();
        a.date_ranges.clear();
        a.structure.page_count = 4;
        let a = checked(a);
        assert!(has(&a, "No date ranges"));
        assert!(has(&a, "unusually long"));
    }

    #[test]
    fn frontend_profiles_get_gap_advice() {
        let mut a = clean();
        a.skills = detect_skills("Skills: React, JavaScript, CSS");
        let a = checked(a);
        assert!(has(&a, "Frontend profile detected"));
        assert!(has(&a, "HTML"));
        assert!(has(&a, "frontend testing tools"));
    }

    #[test]
    fn aliased_frontend_spellings_get_a_literal_matching_tip() {
        let mut a = clean();
        a.skills = detect_skills("Skills: Next.js, React, Jest");
        let a = checked(a);
        assert!(has(&a, "Consider listing both spellings"));
    }

    #[test]
    fn extract_date_ranges_handles_common_formats() {
        let dates =
            extract_date_ranges("Jan 2020 - Present\nMar 2018 – Dec 2019\nSept 2014 to Jun 2017\n");
        assert_eq!(
            dates,
            vec![
                "Jan 2020 - Present",
                "Mar 2018 – Dec 2019",
                "Sept 2014 to Jun 2017"
            ]
        );
        assert!(extract_date_ranges("no dates here").is_empty());
    }
}
