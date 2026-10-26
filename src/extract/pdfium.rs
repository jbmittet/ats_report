use pdfium_render::prelude::*;
use std::fs;
use std::path::PathBuf;

static PDFIUM_SO: &[u8] = include_bytes!(env!("PDFIUM_SO_PATH"));

pub fn library_path() -> PathBuf {
    std::env::temp_dir().join(format!(
        "ats-inspector-pdfium-{}-{}/libpdfium.so",
        env!("CARGO_PKG_VERSION"),
        PDFIUM_SO.len()
    ))
}

pub fn bind_pdfium() -> Result<Pdfium, String> {
    let path = library_path();
    if !path.exists() {
        let dir = path
            .parent()
            .ok_or_else(|| "invalid PDFium library path".to_string())?;
        fs::create_dir_all(dir).map_err(|e| format!("create PDFium temp directory: {e}"))?;
        let tmp = dir.join(format!("libpdfium.so.tmp-{}", std::process::id()));
        fs::write(&tmp, PDFIUM_SO).map_err(|e| format!("write embedded PDFium: {e}"))?;
        fs::rename(&tmp, &path).map_err(|e| {
            let _ = fs::remove_file(&tmp);
            format!("install embedded PDFium: {e}")
        })?;
    }
    let bindings = Pdfium::bind_to_library(&path).map_err(|e| format!("bind PDFium: {e}"))?;
    Ok(Pdfium::new(bindings))
}
