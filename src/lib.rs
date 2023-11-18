use std::path::Path;
use anyhow::{Result, anyhow};

pub fn extract_from_pdf(path: &str) -> Result<String> { 
    let bytes = std::fs::read(path)?;
    let text = pdf_extract::extract_text_from_mem(&bytes)?;
    Ok(text)
}

pub fn into_plain_text(path: &Path) -> Result<String> {

    match path.extension() {
        Some(ext) => {
            match ext.to_str() {
                Some("pdf") => extract_from_pdf(path.to_str().unwrap()),
                _ => Err(anyhow!("Unsupported file extension: {}", ext.to_str().unwrap()))
            }
        },
        None => Err(anyhow!("File has no extension: {}", path.to_str().unwrap()))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_file_extension() {
        into_plain_text(Path::new("path/to/file")).expect_err("File has no extension: path/to/file");
    }

    #[test]
    fn unsupported_file_extension() {
        into_plain_text(Path::new("path/to/file.unsupported")).expect_err("Unsupported file extension: unsupported");
    }

}
