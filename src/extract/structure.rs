use crate::models::Analysis;
use lopdf::{Document, Object};
use std::path::PathBuf;

use super::{build_page_info, extract, normalize_text};

/// Count image XObjects referenced by a page's resources.
fn count_page_images(doc: &Document, page_id: lopdf::ObjectId) -> usize {
    let mut count = 0;

    let (resource_dict, resource_ids) = doc.get_page_resources(page_id);

    let mut dicts: Vec<&lopdf::Dictionary> = Vec::new();
    if let Some(d) = resource_dict {
        dicts.push(d);
    }
    for id in resource_ids {
        if let Ok(d) = doc.get_dictionary(id) {
            dicts.push(d);
        }
    }

    for res in dicts {
        let xobjects = match res.get(b"XObject") {
            Ok(Object::Dictionary(d)) => Some(d),
            Ok(Object::Reference(id)) => doc.get_dictionary(*id).ok(),
            _ => None,
        };
        if let Some(xobjs) = xobjects {
            for (_, obj) in xobjs.iter() {
                if let Object::Reference(id) = obj {
                    if let Ok(Object::Stream(stream)) = doc.get_object(*id) {
                        if let Ok(Object::Name(subtype)) = stream.dict.get(b"Subtype") {
                            if subtype.as_slice() == b"Image" {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    count
}

pub fn analyze_structure_and_pages(path: &PathBuf, a: &mut Analysis) -> Result<(), String> {
    let doc = Document::load(path).map_err(|e| format!("Cannot open PDF: {e}"))?;
    a.structure.encrypted = doc.is_encrypted();
    let pages = doc.get_pages();
    a.structure.page_count = pages.len();

    // PDFium is the primary extractor. The existing extractors remain fallbacks.
    let (texts, image_counts, extractor) = match extract(path) {
        Ok((texts, images)) => (texts, images, "PDFium"),
        Err(pdfium_error) => {
            a.info(format!(
                "PDFium extraction unavailable; falling back: {pdfium_error}"
            ));
            let mut lopdf_texts = Vec::new();
            let mut lopdf_images = Vec::new();
            for (&page_no, &page_id) in pages.iter() {
                lopdf_texts.push(doc.extract_text(&[page_no]).unwrap_or_default());
                lopdf_images.push(count_page_images(&doc, page_id));
            }
            if lopdf_texts.iter().any(|text| !text.trim().is_empty()) {
                (lopdf_texts, lopdf_images, "lopdf")
            } else {
                match pdf_extract::extract_text(path) {
                    Ok(full) if !full.trim().is_empty() => {
                        a.info("Per-page extraction returned no text; used whole-document extractor as fallback (per-page text unavailable).");
                        (vec![full], vec![0], "pdf-extract")
                    }
                    Ok(_) => (lopdf_texts, lopdf_images, "lopdf"),
                    Err(e) => {
                        a.error(format!("Text extraction failed: {e}"));
                        (lopdf_texts, lopdf_images, "lopdf")
                    }
                }
            }
        }
    };
    a.extractor = extractor.to_string();
    a.pages.clear();
    for (index, text) in texts.into_iter().enumerate() {
        let page_no = if extractor == "pdf-extract" {
            1
        } else {
            index + 1
        };
        let images = image_counts.get(index).copied().unwrap_or(0);
        a.pages
            .push(build_page_info(page_no, normalize_text(&text), images));
    }
    a.structure.image_count = image_counts.iter().sum();
    a.full_text = a
        .pages
        .iter()
        .map(|p| p.text.as_str())
        .collect::<Vec<_>>()
        .join("\n\n");
    a.structure.total_chars = a.full_text.chars().count();
    a.structure.total_words = a.full_text.split_whitespace().count();
    Ok(())
}
