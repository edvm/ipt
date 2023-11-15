use std::path::Path;

use lopdf::Document;

pub fn extract_from_pdf(path: &str) -> Result<Vec<String>, String> {

    match Document::load(path) {
        Ok(document) => {
            let pages = document.get_pages();
            let mut texts = Vec::new();

            for (i, _) in pages.iter().enumerate() {
                let page_number = (i + 1) as u32;
                let text = document.extract_text(&[page_number]);
                texts.push(text.unwrap_or_default());
            }

            Ok(texts)
        },
        Err(err) => {
            Err(err.to_string())
        }
    }

}

pub fn extract_text(path: &str) -> Result<Vec<String>, String> {
    let path = Path::new(path);
    match path.extension() {
        Some(ext) => {
            match ext.to_str() {
                Some("pdf") => extract_from_pdf(path.to_str().unwrap()),
                _ => {
                    let err = format!("Unsupported file extension: {}", ext.to_str().unwrap());
                    Err(err)
                }
            }
        },
        None => {
            let err = format!("File has no extension: {}", path.to_str().unwrap());
            Err(err)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_file_extension() {
        extract_text("path/to/file").expect_err("File has no extension: path/to/file");
    }

    #[test]
    fn unsupported_file_extension() {
        extract_text("path/to/file.unsupported").expect_err("Unsupported file extension: unsupported");
    }
}
