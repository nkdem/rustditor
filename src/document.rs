use std::{fs::File};

pub struct Document {
    filename: String,
    file: File,
    content: String
}

impl Document {
    fn new(filename: String) -> Document {
        let file = File::create(&filename).expect("Unable to create file");
        Document {
            filename,
            file,
            content: String::new()
        }
    }

    fn delete(&self) {
        std::fs::remove_file(&self.filename).expect("Unable to delete file");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document() {
        let filename = String::from("test.txt");
        let doc = Document::new(filename.clone());
        assert_eq!(doc.filename, filename);
        assert_eq!(doc.content, "");

        assert!(std::path::Path::new(&filename).exists());
        doc.delete();
        assert!(!std::path::Path::new(&filename).exists());
    }
}