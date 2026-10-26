# ats-inspector

A Rust command-line tool that inspects a PDF and shows what an Applicant Tracking System (ATS) is likely to "see": extracted text, reading order, contact fields, section headers, dates, and an overall score with findings.

## Features

- **Primary extraction with PDFium** (the engine behind Chromium's PDF viewer), with a `lopdf` and `pdf-extract` fallback
- **Per-page extraction** with word, line, and short-line statistics
- **Scoring out of 100** across five categories:
  - Text extractability (30)
  - Layout and reading order (20)
  - Contact info (15)
  - Section structure (15)
  - Dates (10)
  - Content and length (10)
- **Contact parsing**: name, email, phone, LinkedIn, GitHub, and URLs
- **Findings** that flag suspicious layouts, odd glyphs (icon fonts or custom bullets), and missing sections
- **JSON export** of the full analysis

## Requirements

- Rust (stable, 2021 edition or later)
- Internet access on first build: `build.rs` downloads the PDFium binary for your platform

No system libraries need to be installed manually.

## Supported platforms

PDFium binaries are fetched from [bblanchon/pdfium-binaries](https://github.com/bblanchon/pdfium-binaries) based on the target OS and architecture. The release is pinned in `build.rs` (`PDFIUM_RELEASE`) so builds are reproducible.

| OS      | Architectures       |
|---------|---------------------|
| Linux   | x64, arm64          |
| macOS   | x64, arm64          |
| Windows | x64, arm64          |

## Build and run

```bash
cargo build
cargo run -- path/to/resume.pdf
```

On first build, `build.rs` downloads and unpacks the PDFium library into `OUT_DIR`. Subsequent builds reuse it.

## Usage

```bash
# Human-readable report
cargo run -- resume.pdf

# Include the full extracted text (page by page) or a short per-page preview
cargo run -- resume.pdf --raw
cargo run -- resume.pdf --previews

# Compare against a job description
cargo run -- resume.pdf --job posting.txt

# Export the full analysis as JSON (to a file or to stdout)
cargo run -- resume.pdf --json out.json
cargo run -- resume.pdf --json | jq .score

# Print only the extracted text, e.g. for piping
cargo run -- resume.pdf --text-only > resume.txt

# Fail with a non-zero exit code when the score is too low (useful in CI)
cargo run -- resume.pdf --quiet --fail-under 70
```

`--json` and `--text-only` imply `--quiet`, so machine-readable output on stdout
is never mixed with the colored report. `--raw` and `--previews` add sections to
the human report, so they conflict with `--quiet`.

Run `cargo run -- --help` for the complete list of options.

## Example output

```
SCORE: 100/100  A: ATS-friendly
  Text extractability      ████████████████████ 30/30  Text extracted cleanly
  Layout & reading order   ████████████████████ 20/20  Linear, clean reading order
  Contact info             ████████████████████ 15/15  All contact fields found
  Section structure        ████████████████████ 15/15  Key sections recognized
  Dates                    ████████████████████ 10/10  5 date ranges found
  Content & length         ████████████████████ 10/10  22 skills across 6 categories
```

## Test samples

Sample PDFs at three quality levels can be generated with the Python script in `tools/make_samples.py` (requires `reportlab`). They cover low, middle, and high scores.
Create a virtual environment from the tools folder with `python3 -m venv .venv` and activate it with `source .venv/bin/activate` and then finally do a `pip install -r requirements.txt`
Finally generate samples with `python3 make_samples.py`, three files (low_score.pdf, middle_score.pdf and high_score.pdf) will be generated in the root folder of the project.

## Troubleshooting

- **`PDFium archive did not contain ...`**: the archive layout differs from what `pdfium_asset()` expects. List the archive contents with `tar tz` and update the path in `build.rs`.
- **`undefined symbol` or `LoadLibraryError`**: the `chromium/` tag in `build.rs` does not match the `pdfium-render` version. Keep them aligned (`pdfium-render` 0.8.x defaults to `pdfium_7543`).
- **Falls back to `lopdf`**: PDFium failed to load. The report includes the reason under Findings.

## Limitations

- Scoring is heuristic. It approximates common ATS behavior but does not replicate any specific vendor's parser.
- Multi-column layouts are detected from short-line ratios, not from true layout analysis.
- Scanned PDFs without a text layer will score as having no extractable text.

## License

Add your license here.