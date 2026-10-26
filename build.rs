use flate2::read::GzDecoder;
use std::env;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use tar::Archive;

// Pin a release so builds are reproducible. Check the latest tag at
// https://github.com/bblanchon/pdfium-binaries/releases and update as needed.
const PDFIUM_RELEASE: &str = "chromium/7543";

/// Returns (archive name, library path inside the archive) for the build target.
fn pdfium_asset() -> Result<(&'static str, &'static str), String> {
    let os = env::var("CARGO_CFG_TARGET_OS").map_err(|e| e.to_string())?;
    let arch = env::var("CARGO_CFG_TARGET_ARCH").map_err(|e| e.to_string())?;
    match (os.as_str(), arch.as_str()) {
        ("linux", "x86_64") => Ok(("pdfium-linux-x64.tgz", "lib/libpdfium.so")),
        ("linux", "aarch64") => Ok(("pdfium-linux-arm64.tgz", "lib/libpdfium.so")),
        ("linux", "arm") => Ok(("pdfium-linux-arm.tgz", "lib/libpdfium.so")),
        ("macos", "x86_64") => Ok(("pdfium-mac-x64.tgz", "lib/libpdfium.dylib")),
        ("macos", "aarch64") => Ok(("pdfium-mac-arm64.tgz", "lib/libpdfium.dylib")),
        ("windows", "x86_64") => Ok(("pdfium-win-x64.tgz", "bin/pdfium.dll")),
        ("windows", "aarch64") => Ok(("pdfium-win-arm64.tgz", "bin/pdfium.dll")),
        _ => Err(format!("no PDFium binary available for {os}/{arch}")),
    }
}

fn main() {
    let (asset, lib_rel) = match pdfium_asset() {
        Ok(v) => v,
        Err(e) => panic!("build.rs: {e}"),
    };
    let url = format!(
        "https://github.com/bblanchon/pdfium-binaries/releases/download/{PDFIUM_RELEASE}/{asset}"
    );

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is not set"));
    let pdfium_dir = out_dir.join("pdfium");
    let library = pdfium_dir.join(lib_rel);

    if !library.exists() {
        fs::create_dir_all(&pdfium_dir).expect("create PDFium output directory");
        let response = ureq::get(&url)
            .call()
            .unwrap_or_else(|e| panic!("download PDFium from {url}: {e}"));
        let mut bytes = Vec::new();
        response
            .into_reader()
            .read_to_end(&mut bytes)
            .expect("read PDFium archive");
        let mut archive = Archive::new(GzDecoder::new(bytes.as_slice()));
        archive.unpack(&pdfium_dir).expect("unpack PDFium archive");
    }

    if !library.exists() {
        panic!("PDFium archive did not contain {}", library.display());
    }
    let library = fs::canonicalize(&library).unwrap_or(library);
    println!("cargo:rustc-env=PDFIUM_SO_PATH={}", library.display());
    println!("cargo:rerun-if-changed=build.rs");
}
