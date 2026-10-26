//! Test support: generates tiny but real PDFs, so the CLI tests exercise the
//! actual extraction pipeline (PDFium included) instead of a mock.

#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

/// A realistic, ATS-friendly resume split into the lines of one PDF page.
pub const SAMPLE_RESUME: &[&str] = &[
    "John Doe",
    "john.doe@example.com | +1 555 010-9999 | linkedin.com/in/johndoe",
    "SUMMARY",
    "Senior software engineer with ten years of experience building web applications and distributed systems for large teams across many industries and regions.",
    "EXPERIENCE",
    "Senior Software Engineer, Acme Corp, Jan 2020 - Present",
    "Led migration of twelve services to Kubernetes, reducing deployment time by sixty percent and improving reliability across the platform.",
    "Mentored four engineers and introduced code review standards that raised overall code quality and reduced production incidents.",
    "Software Engineer, Beta Inc, Mar 2018 - Dec 2019",
    "Built REST APIs in Python and Go serving two million requests per day for customers around the world.",
    "Designed and implemented a caching layer with Redis that cut response times significantly for the most popular endpoints.",
    "EDUCATION",
    "BSc Computer Science, University of Somewhere, Sept 2014 - Jun 2017",
    "SKILLS",
    "HTML, CSS, JavaScript, TypeScript, React, Next.js, Vue, Node.js, Express, Python, Django, Go, Rust, PostgreSQL, MySQL, MongoDB, Redis, Docker, Kubernetes, AWS, Terraform, Git, CI/CD, Jest, Playwright, Agile, Leadership, Communication",
];

pub const TINY_RESUME: &[&str] = &["Jane Doe", "jane@example.com"];

fn unique_path(extension: &str) -> PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "ats-inspector-test-{}-{}.{}",
        std::process::id(),
        n,
        extension
    ))
}

/// Removes its file when the test ends.
pub struct TempFile {
    pub path: PathBuf,
}

impl TempFile {
    pub fn with_text(contents: &str, extension: &str) -> Self {
        let path = unique_path(extension);
        std::fs::write(&path, contents).expect("write temp file");
        Self { path }
    }

    pub fn empty_named(extension: &str) -> Self {
        let path = unique_path(extension);
        Self { path }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

pub struct TempPdf {
    pub path: PathBuf,
}

impl TempPdf {
    pub fn from_pages(pages: &[&[&str]]) -> Self {
        let path = unique_path("pdf");
        std::fs::write(&path, text_pdf(pages)).expect("write temp pdf");
        Self { path }
    }

    pub fn sample() -> Self {
        Self::from_pages(&[SAMPLE_RESUME])
    }

    pub fn tiny() -> Self {
        Self::from_pages(&[TINY_RESUME])
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempPdf {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

fn escape_pdf_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('(', "\\(")
        .replace(')', "\\)")
}

/// Builds a minimal, standards-compliant PDF with one text line per entry.
/// Object 1 = catalog, 2 = pages, 3 = font, then page/content pairs.
pub fn text_pdf(pages: &[&[&str]]) -> Vec<u8> {
    assert!(!pages.is_empty(), "a PDF needs at least one page");

    let mut objects: Vec<(u32, Vec<u8>)> = Vec::new();
    let mut kids = String::new();

    for (index, lines) in pages.iter().enumerate() {
        let page_id = 4 + 2 * index as u32;
        let content_id = page_id + 1;
        kids.push_str(&format!("{page_id} 0 R "));

        let mut content = String::new();
        for (line_no, line) in lines.iter().enumerate() {
            let y = 760 - 16 * line_no as i32;
            content.push_str(&format!(
                "BT /F1 12 Tf 72 {y} Td ({}) Tj ET\n",
                escape_pdf_string(line)
            ));
        }

        objects.push((
            page_id,
            format!(
                "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] \
                 /Resources << /Font << /F1 3 0 R >> >> /Contents {content_id} 0 R >>"
            )
            .into_bytes(),
        ));

        let mut stream = format!("<< /Length {} >>\nstream\n", content.len()).into_bytes();
        stream.extend_from_slice(content.as_bytes());
        stream.extend_from_slice(b"endstream");
        objects.push((content_id, stream));
    }

    objects.push((1, b"<< /Type /Catalog /Pages 2 0 R >>".to_vec()));
    objects.push((
        2,
        format!("<< /Type /Pages /Kids [{kids}] /Count {} >>", pages.len()).into_bytes(),
    ));
    objects.push((
        3,
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_vec(),
    ));
    objects.sort_by_key(|(id, _)| *id);

    let mut out = b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n".to_vec();
    let mut offsets = Vec::new();
    for (id, body) in &objects {
        offsets.push((*id, out.len()));
        out.extend_from_slice(format!("{id} 0 obj\n").as_bytes());
        out.extend_from_slice(body);
        out.extend_from_slice(b"\nendobj\n");
    }

    let startxref = out.len();
    let max_id = objects
        .iter()
        .map(|(id, _)| *id)
        .max()
        .expect("at least one object");

    out.extend_from_slice(format!("xref\n0 {}\n", max_id + 1).as_bytes());
    out.extend_from_slice(b"0000000000 65535 f \n");
    for id in 1..=max_id {
        let offset = offsets
            .iter()
            .find(|(object_id, _)| *object_id == id)
            .map(|(_, offset)| *offset)
            .unwrap_or(0);
        out.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(
        format!(
            "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{startxref}\n%%EOF\n",
            max_id + 1
        )
        .as_bytes(),
    );

    out
}
