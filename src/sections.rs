use crate::models::SectionInfo;
use std::collections::BTreeMap;

const SECTIONS: &[(&str, &[&str])] = &[
    (
        "Summary",
        &[
            "summary",
            "professional summary",
            "profile",
            "objective",
            "about me",
            "about",
        ],
    ),
    (
        "Experience",
        &[
            "experience",
            "work experience",
            "professional experience",
            "employment history",
            "work history",
            "employment",
        ],
    ),
    (
        "Education",
        &[
            "education",
            "academic background",
            "academic qualifications",
            "education & training",
            "education and training",
            "education & continuing training",
            "continuing training",
            "studies",
            "formation",
        ],
    ),
    (
        "Skills",
        &[
            "skills",
            "technical skills",
            "core competencies",
            "key skills",
            "technologies",
            "tech stack",
        ],
    ),
    (
        "Projects",
        &[
            "projects",
            "personal projects",
            "selected projects",
            "side projects",
        ],
    ),
    (
        "Certifications",
        &[
            "certifications",
            "certificates",
            "licenses",
            "licenses & certifications",
        ],
    ),
    ("Languages", &["languages"]),
];

/// Returns the canonical section name if the line looks like a section header.
pub fn detect_section_header(line: &str) -> Option<&'static str> {
    let clean = line.trim().trim_end_matches(':').trim().to_lowercase();
    if clean.is_empty() || clean.split_whitespace().count() > 4 {
        return None;
    }
    for (name, aliases) in SECTIONS {
        if aliases.iter().any(|a| *a == clean) {
            return Some(name);
        }
    }
    None
}

pub fn detect_sections(text: &str) -> Vec<SectionInfo> {
    let mut counts: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut current: Option<&'static str> = None;

    for line in text.lines() {
        if let Some(sec) = detect_section_header(line) {
            current = Some(sec);
            counts.entry(sec).or_insert(0);
            continue;
        }
        if let Some(sec) = current {
            if !line.trim().is_empty() {
                *counts.entry(sec).or_insert(0) += 1;
            }
        }
    }

    SECTIONS
        .iter()
        .map(|(name, _)| SectionInfo {
            name: name.to_string(),
            found: counts.contains_key(name),
            line_count: counts.get(name).copied().unwrap_or(0),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn section<'a>(sections: &'a [SectionInfo], name: &str) -> &'a SectionInfo {
        sections
            .iter()
            .find(|s| s.name == name)
            .unwrap_or_else(|| panic!("section {name} missing"))
    }

    #[test]
    fn detects_headers_case_insensitively_and_ignores_colon() {
        assert_eq!(detect_section_header("EXPERIENCE"), Some("Experience"));
        assert_eq!(detect_section_header("Experience:"), Some("Experience"));
        assert_eq!(
            detect_section_header("  Work History  "),
            Some("Experience")
        );
        assert_eq!(
            detect_section_header("education & training"),
            Some("Education")
        );
        assert_eq!(detect_section_header("TECH STACK"), Some("Skills"));
        assert_eq!(
            detect_section_header("Licenses & Certifications"),
            Some("Certifications")
        );
    }

    #[test]
    fn rejects_lines_that_are_not_headers() {
        assert_eq!(detect_section_header(""), None);
        assert_eq!(detect_section_header("   "), None);
        assert_eq!(detect_section_header("John Doe"), None);
        assert_eq!(
            detect_section_header("Senior Software Engineer at Acme"),
            None
        );
        assert_eq!(
            detect_section_header("A very long line that is not a header"),
            None
        );
    }

    #[test]
    fn counts_non_empty_content_lines_until_the_next_header() {
        let text = "John Doe\nExperienced engineer\n\nEXPERIENCE\nAcme 2020 - 2021\nDid things\n\nEDUCATION\nBSc CS\nSKILLS\nRust, Go\n";
        let sections = detect_sections(text);

        assert!(section(&sections, "Experience").found);
        assert_eq!(section(&sections, "Experience").line_count, 2);
        assert!(section(&sections, "Education").found);
        assert_eq!(section(&sections, "Education").line_count, 1);
        assert!(section(&sections, "Skills").found);
        assert_eq!(section(&sections, "Skills").line_count, 1);
    }

    #[test]
    fn content_before_the_first_header_is_ignored() {
        let sections = detect_sections("John Doe\nSome preamble text\nEXPERIENCE\nAcme\n");
        assert_eq!(section(&sections, "Experience").line_count, 1);
        assert!(!section(&sections, "Summary").found);
    }

    #[test]
    fn header_without_body_is_found_with_zero_lines() {
        let sections = detect_sections("SKILLS\n\nCERTIFICATIONS\n");
        assert!(section(&sections, "Skills").found);
        assert_eq!(section(&sections, "Skills").line_count, 0);
        assert!(section(&sections, "Certifications").found);
        assert_eq!(section(&sections, "Certifications").line_count, 0);
    }

    #[test]
    fn reports_every_known_section_in_canonical_order() {
        let names: Vec<String> = detect_sections("").into_iter().map(|s| s.name).collect();
        assert_eq!(
            names,
            vec![
                "Summary",
                "Experience",
                "Education",
                "Skills",
                "Projects",
                "Certifications",
                "Languages"
            ]
        );
    }
}
