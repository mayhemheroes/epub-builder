use honggfuzz::fuzz;
use epub_builder::{EpubBuilder, Result, ZipLibrary, EpubContent};

fn try_fuzz(s: &str) -> Result<Vec<u8>> {
    let mut output = Vec::<u8>::new();
    EpubBuilder::new(ZipLibrary::new()?)?
        .add_content(EpubContent::new("notes.xhtml", s.as_bytes()))?
        .generate(&mut output)?;
    Ok(output)
}

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if let Ok(s) = std::str::from_utf8(data) {
                let _ = try_fuzz(s);
            }
        });
    }
}