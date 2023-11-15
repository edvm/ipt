use std::path::Path;

pub fn extract_from_pdf(path: &str) -> String { 
    let bytes = std::fs::read(path).unwrap();
    pdf_extract::extract_text_from_mem(&bytes).expect("Error extracting text from PDF")
}

pub fn extract_text(path: &str) -> Result<String, String> {

    let path = Path::new(path);

    match path.extension() {
        Some(ext) => {
            match ext.to_str() {
                Some("pdf") => Ok(extract_from_pdf(path.to_str().unwrap())),
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
