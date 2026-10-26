use crate::models::{JobMatch, SkillsReport};
use crate::skills::{count_term, detect_skills};
use std::collections::BTreeMap;

const STOPWORDS: &[&str] = &[
    "the",
    "and",
    "for",
    "with",
    "you",
    "your",
    "our",
    "are",
    "will",
    "have",
    "has",
    "that",
    "this",
    "from",
    "they",
    "their",
    "about",
    "who",
    "can",
    "all",
    "any",
    "not",
    "but",
    "was",
    "were",
    "been",
    "being",
    "into",
    "out",
    "over",
    "such",
    "than",
    "then",
    "them",
    "these",
    "those",
    "what",
    "when",
    "where",
    "which",
    "while",
    "also",
    "etc",
    "per",
    "use",
    "using",
    "work",
    "working",
    "team",
    "teams",
    "role",
    "job",
    "years",
    "year",
    "experience",
    "strong",
    "ability",
    "skills",
    "skill",
    "knowledge",
    "including",
    "responsibilities",
    "requirements",
    "preferred",
    "required",
    "plus",
    "new",
    "more",
    "must",
    "should",
    "would",
    "able",
    "well",
    "build",
    "building",
    "help",
    "join",
    "company",
    "looking",
    "candidate",
    "position",
];

pub fn job_match(jd_text: &str, resume_skills: &SkillsReport, resume_text: &str) -> JobMatch {
    let jd_skills = detect_skills(jd_text);
    let required: Vec<String> = jd_skills.all.clone();

    let (matched, missing): (Vec<String>, Vec<String>) = required
        .iter()
        .cloned()
        .partition(|s| resume_skills.all.contains(s));

    let match_percent = if required.is_empty() {
        0
    } else {
        ((matched.len() as f32 / required.len() as f32) * 100.0).round() as u32
    };

    // Other frequent JD words (non-skill, non-stopword) missing from the resume.
    let resume_lower = resume_text.to_lowercase();
    let mut freq: BTreeMap<String, usize> = BTreeMap::new();
    for w in jd_text
        .to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| w.len() >= 4 && !STOPWORDS.contains(w))
    {
        *freq.entry(w.to_string()).or_insert(0) += 1;
    }
    let mut candidates: Vec<(String, usize)> = freq
        .into_iter()
        .filter(|(w, n)| *n >= 2 && !contains_word(&resume_lower, w))
        .collect();
    candidates.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    let other_missing: Vec<String> = candidates.into_iter().take(15).map(|(w, _)| w).collect();

    JobMatch {
        required_skills: required,
        matched,
        missing,
        match_percent,
        other_missing_keywords: other_missing,
    }
}

fn contains_word(haystack_lower: &str, word: &str) -> bool {
    count_term(haystack_lower, word) > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESUME: &str =
        "Skills: Rust, Docker, PostgreSQL, React\nI know Rust, Docker, PostgreSQL and React";

    #[test]
    fn computes_skill_match_percentage() {
        let resume = detect_skills(RESUME);
        let jd = "We need Rust, Kubernetes, Docker, PostgreSQL and GraphQL.";
        let m = job_match(jd, &resume, RESUME);

        // Order follows the skill taxonomy, not the order in the job description.
        assert_eq!(
            m.required_skills,
            vec!["GraphQL", "Rust", "PostgreSQL", "Docker", "Kubernetes"]
        );
        assert_eq!(m.matched, vec!["Rust", "PostgreSQL", "Docker"]);
        assert_eq!(m.missing, vec!["GraphQL", "Kubernetes"]);
        assert_eq!(m.match_percent, 60);
    }

    #[test]
    fn empty_jd_has_zero_match_and_no_keywords() {
        let resume = detect_skills(RESUME);
        let m = job_match("We are looking for a great person.", &resume, RESUME);
        assert!(m.required_skills.is_empty());
        assert_eq!(m.match_percent, 0);
        assert!(m.other_missing_keywords.is_empty());
    }

    #[test]
    fn reports_frequent_jd_words_missing_from_the_resume() {
        let resume = detect_skills("Rust only");
        let jd = "Rust, Kubernetes, GraphQL. Kubernetes and GraphQL again. Kubernetes.";
        let m = job_match(jd, &resume, "I write Rust");
        assert_eq!(m.other_missing_keywords, vec!["kubernetes", "graphql"]);
    }

    #[test]
    fn caps_other_missing_keywords_at_fifteen() {
        let words = [
            "alpha", "bravo", "charlie", "delta", "echo", "foxtrot", "golf", "hotel", "india",
            "juliet", "kilo", "lima", "mike", "november", "oscar", "papa", "quebec", "romeo",
            "sierra", "tango",
        ];
        let jd = words
            .iter()
            .chain(words.iter())
            .copied()
            .collect::<Vec<_>>()
            .join(" ");
        let resume = detect_skills("");
        let m = job_match(&jd, &resume, "");
        assert_eq!(m.other_missing_keywords.len(), 15);
    }
}
