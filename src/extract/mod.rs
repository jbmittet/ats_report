use crate::models::PageInfo;
use crate::regexes::WEIRD_GLYPH_RE;
use std::path::Path;

pub(super) mod pdfium;
pub mod structure;

use pdfium::bind_pdfium;
use pdfium_render::prelude::PdfPageObjectType;
use pdfium_render::prelude::PdfPageObjectsCommon;

pub fn extract(path: &Path) -> Result<(Vec<String>, Vec<usize>), String> {
    let pdfium = bind_pdfium()?;
    let document = pdfium
        .load_pdf_from_file(path, None)
        .map_err(|e| format!("load with PDFium: {e}"))?;
    let mut texts = Vec::new();
    let mut images = Vec::new();
    for page in document.pages().iter() {
        let text = page
            .text()
            .map_err(|e| format!("extract PDFium page text: {e}"))?
            .all();
        let image_count = page
            .objects()
            .iter()
            .filter(|object| object.object_type() == PdfPageObjectType::Image)
            .count();
        texts.push(text);
        images.push(image_count);
    }
    Ok((texts, images))
}

fn normalize_text(s: &str) -> String {
    s.replace('\r', "")
        .replace('\u{00A0}', " ")
        .lines()
        .map(|l| l.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
}

fn build_page_info(page: usize, text: String, images: usize) -> PageInfo {
    let lines: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
    let words = text.split_whitespace().count();
    let short = lines
        .iter()
        .filter(|l| l.split_whitespace().count() <= 2)
        .count();
    let ratio = if lines.is_empty() {
        0.0
    } else {
        short as f32 / lines.len() as f32
    };
    let weird = WEIRD_GLYPH_RE.find_iter(&text).count();

    let status = if words < 5 {
        "empty"
    } else if ratio > 0.5 || weird > 5 {
        "suspicious"
    } else {
        "ok"
    };

    PageInfo {
        page,
        words,
        lines: lines.len(),
        short_line_ratio: (ratio * 100.0).round() / 100.0,
        weird_glyphs: weird,
        images,
        status: status.into(),
        text,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_removes_carriage_returns_and_nbsp() {
        assert_eq!(normalize_text("a\r\nb\r\n"), "a\nb");
        assert_eq!(normalize_text("a\u{00A0}b"), "a b");
    }

    #[test]
    fn normalize_trims_trailing_whitespace_but_keeps_leading() {
        assert_eq!(normalize_text("a   \nb  \n  indented"), "a\nb\n  indented");
        assert_eq!(normalize_text(""), "");
    }

    #[test]
    fn build_page_info_counts_words_and_lines() {
        let p = build_page_info(7, "a b\n\nc d e f g".into(), 3);
        assert_eq!(p.page, 7);
        assert_eq!(p.words, 7);
        assert_eq!(p.lines, 2);
        assert_eq!(p.images, 3);
        assert_eq!(p.status, "ok");
        assert_eq!(p.text, "a b\n\nc d e f g");
    }

    #[test]
    fn pages_with_fewer_than_five_words_are_empty() {
        let p = build_page_info(1, "one two three four".into(), 0);
        assert_eq!(p.status, "empty");
        assert_eq!(build_page_info(1, "".into(), 0).status, "empty");
    }

    #[test]
    fn mostly_short_lines_mark_a_page_suspicious() {
        let p = build_page_info(1, "a b\nc d\ne f\nlong line here".into(), 0);
        assert_eq!(p.words, 9);
        assert_eq!(p.short_line_ratio, 0.75);
        assert_eq!(p.status, "suspicious");
    }

    #[test]
    fn many_weird_glyphs_mark_a_page_suspicious() {
        let text = "one two three four five \u{E000}\u{E001}\u{E002}\u{E003}\u{E004}\u{E005}";
        let p = build_page_info(1, text.into(), 0);
        assert_eq!(p.weird_glyphs, 6);
        assert_eq!(p.status, "suspicious");
    }

    #[test]
    fn short_line_ratio_is_rounded_to_two_decimals() {
        let p = build_page_info(1, "one two three four\nfive six seven eight\na b".into(), 0);
        assert_eq!(p.short_line_ratio, 0.33);
        assert_eq!(p.status, "ok");
    }

    #[test]
    fn exactly_half_short_lines_is_not_suspicious() {
        let p = build_page_info(1, "one two three\na b".into(), 0);
        assert_eq!(p.short_line_ratio, 0.5);
        assert_eq!(p.status, "ok");
    }
}
