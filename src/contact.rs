use crate::models::Contact;
use crate::regexes::{EMAIL_RE, GITHUB_RE, LINKEDIN_RE, PHONE_RE, URL_RE};
use crate::sections::detect_section_header;

pub fn extract_contact(text: &str) -> Contact {
    let email = EMAIL_RE.find(text).map(|m| m.as_str().to_string());
    let linkedin = LINKEDIN_RE.find(text).map(|m| m.as_str().to_string());
    let github = GITHUB_RE.find(text).map(|m| m.as_str().to_string());

    // Phone: only look in the top part of the document, and require >= 9 digits
    let head: String = text.lines().take(40).collect::<Vec<_>>().join("\n");
    let phone = PHONE_RE
        .find_iter(&head)
        .map(|m| m.as_str().split_whitespace().collect::<Vec<_>>().join(" "))
        .find(|p| p.chars().filter(|c| c.is_ascii_digit()).count() >= 9);

    let mut urls = URL_RE
        .find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect::<Vec<_>>();
    urls.sort();
    urls.dedup();

    // Name guess: first short, alphabetic-looking line that isn't contact info/header
    let name_guess = text
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .take(6)
        .find(|l| {
            let words = l.split_whitespace().count();
            (2..=4).contains(&words)
                && !l.contains('@')
                && !l.chars().any(|ch| ch.is_ascii_digit())
                && l.chars().all(|ch| {
                    ch.is_alphabetic() || ch.is_whitespace() || ch == '-' || ch == '\'' || ch == '.'
                })
                && detect_section_header(l).is_none()
        })
        .map(|s| s.to_string());

    Contact {
        name_guess,
        email,
        phone,
        linkedin,
        github,
        urls,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEADER: &str = "\
    Jean-Baptiste Mittet
    jean.mittet@example.com | +1 (555) 010-9999 | linkedin.com/in/jbmittet
    github.com/jbmittet
    https://jbm.dev

    EXPERIENCE
    Senior Engineer, Acme
    ";

    #[test]
    fn extracts_a_typical_header_block() {
        let c = extract_contact(HEADER);
        assert_eq!(c.email.as_deref(), Some("jean.mittet@example.com"));
        assert_eq!(c.linkedin.as_deref(), Some("linkedin.com/in/jbmittet"));
        assert_eq!(c.github.as_deref(), Some("github.com/jbmittet"));
        assert_eq!(c.name_guess.as_deref(), Some("Jean-Baptiste Mittet"));
        assert_eq!(c.urls, vec!["https://jbm.dev"]);
        let phone = c.phone.expect("phone should be found");
        assert!(
            phone.chars().filter(|ch| ch.is_ascii_digit()).count() >= 9,
            "unexpected phone: {phone}"
        );
        assert!(phone.contains("555"));
    }

    #[test]
    fn normalizes_phone_whitespace() {
        let c = extract_contact("Jane Doe\n555\t010\t9999\n");
        assert_eq!(c.phone.as_deref(), Some("555 010 9999"));
    }

    #[test]
    fn rejects_phone_numbers_with_fewer_than_nine_digits() {
        let c = extract_contact("Jane Doe\nCall me on 12-34\n");
        assert_eq!(c.phone, None);
    }

    #[test]
    fn only_scans_the_first_forty_lines_for_a_phone() {
        let mut text = String::from("Jane Doe\n");
        for _ in 0..45 {
            text.push_str("filler line\n");
        }
        text.push_str("+1 555 010 9999\n");
        assert_eq!(extract_contact(&text).phone, None);
    }

    #[test]
    fn deduplicates_and_sorts_urls() {
        let c =
            extract_contact("Jane Doe\nhttps://b.example https://a.example https://a.example\n");
        assert_eq!(c.urls, vec!["https://a.example", "https://b.example"]);
    }

    #[test]
    fn name_guess_skips_headers_emails_and_lines_with_digits() {
        let text = "EXPERIENCE\njohn@example.com\n2020 - 2024\nJane Doe\n";
        assert_eq!(
            extract_contact(text).name_guess.as_deref(),
            Some("Jane Doe")
        );
    }

    #[test]
    fn name_guess_allows_hyphens_apostrophes_and_periods() {
        assert_eq!(
            extract_contact("Anne-Marie O'Connor Jr.\n")
                .name_guess
                .as_deref(),
            Some("Anne-Marie O'Connor Jr.")
        );
    }

    #[test]
    fn missing_contact_fields_stay_none() {
        let c = extract_contact("Summary\n2020 - 2024\njohn@example.com\n");
        assert_eq!(c.name_guess, None);
        assert_eq!(c.phone, None);
        assert_eq!(c.linkedin, None);
        assert_eq!(c.github, None);
        assert!(c.urls.is_empty());
        assert_eq!(c.email.as_deref(), Some("john@example.com"));
    }
}
