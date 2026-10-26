use once_cell::sync::Lazy;
use regex::Regex;

pub static EMAIL_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)[a-z0-9._%+\-]+@[a-z0-9.\-]+\.[a-z]{2,}").unwrap());

pub static PHONE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?x)
        (?:\+\d{1,3}[\s.\-]?)?        # optional +country code
        (?:\(?\d{1,4}\)?[\s.\-]?)     # first group
        (?:\d{2,4}[\s.\-]?){2,4}      # remaining groups
        \d{2,4}
    ",
    )
    .unwrap()
});

pub static LINKEDIN_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)linkedin\.com/in/[a-z0-9\-_%]+").unwrap());

pub static GITHUB_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)github\.com/[a-z0-9\-_]+").unwrap());

pub static URL_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(?i)https?://[^\s)>\]"']+"#).unwrap());

pub static DATE_RANGE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?i)((jan|feb|mar|apr|may|jun|jul|aug|sep|sept|oct|nov|dec)[a-z]*\.?\s+)?(19|20)\d{2}\s*[-–—to]+\s*(((jan|feb|mar|apr|may|jun|jul|aug|sep|sept|oct|nov|dec)[a-z]*\.?\s+)?(19|20)\d{2}|present|current|now)",
    )
        .unwrap()
});

pub static WEIRD_GLYPH_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[\u{E000}-\u{F8FF}\u{FFFD}]").unwrap());

// Headers like "E X P E R I E N C E" (letter-spaced)
pub static SPACED_HEADER_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?:[A-Za-z]\s){4,}[A-Za-z]$").unwrap());

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_matches_without_trailing_punctuation() {
        let m = EMAIL_RE
            .find("Contact: John.Doe+tag@example.co.uk.")
            .unwrap();
        assert_eq!(m.as_str(), "John.Doe+tag@example.co.uk");
        assert!(!EMAIL_RE.is_match("not-an-email@x"));
    }

    #[test]
    fn phone_matches_have_enough_digits() {
        for candidate in ["+1 (555) 010-9999", "555-010-9999", "+33 6 12 34 56 78"] {
            let m = PHONE_RE.find(candidate).expect(candidate);
            assert!(
                m.as_str().chars().filter(|c| c.is_ascii_digit()).count() >= 9,
                "{candidate} matched as {}",
                m.as_str()
            );
        }
    }

    #[test]
    fn linkedin_and_github_ignore_the_scheme() {
        assert_eq!(
            LINKEDIN_RE
                .find("See https://www.linkedin.com/in/jb-mittet/ now")
                .unwrap()
                .as_str(),
            "linkedin.com/in/jb-mittet"
        );
        assert_eq!(
            GITHUB_RE.find("github.com/foo-bar/baz").unwrap().as_str(),
            "github.com/foo-bar"
        );
    }

    #[test]
    fn urls_are_matched_without_surrounding_text() {
        let urls: Vec<&str> = URL_RE
            .find_iter("https://a.example/x and http://b.example")
            .map(|m| m.as_str())
            .collect();
        assert_eq!(urls, vec!["https://a.example/x", "http://b.example"]);
    }

    #[test]
    fn date_ranges_cover_month_years_dashes_and_present() {
        for value in [
            "Jan 2020 - Present",
            "2018–2021",
            "Sept 2019 to Dec 2020",
            "January 2020 — December 2021",
        ] {
            assert!(DATE_RANGE_RE.is_match(value), "should match: {value}");
        }
        assert!(!DATE_RANGE_RE.is_match("2020"));
        assert!(!DATE_RANGE_RE.is_match("Present"));
    }

    #[test]
    fn weird_glyphs_include_private_use_and_replacement_char() {
        assert_eq!(WEIRD_GLYPH_RE.find_iter("\u{E000}x\u{FFFD}").count(), 2);
        assert!(!WEIRD_GLYPH_RE.is_match("plain ASCII"));
    }

    #[test]
    fn spaced_headers_are_recognised() {
        assert!(SPACED_HEADER_RE.is_match("E X P E R I E N C E"));
        assert!(SPACED_HEADER_RE.is_match("A B C D E"));
        assert!(!SPACED_HEADER_RE.is_match("EXPERIENCE"));
    }
}
