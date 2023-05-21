use std::{fs::File, io::{Read, Write}};

pub struct Document {
    pub filename: String,
    pub file: File,
    pub content: String
}

impl Document {
    pub fn delete(&self) {
        std::fs::remove_file(&self.filename).expect("Unable to delete file");
    }

    pub fn open(filename: &String) -> Self{
        let mut file = File::open(filename).or_else(|_| File::create(filename)).expect("Unable to open file");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Unable to read file");
        Self {
            filename: filename.to_string(),
            file,
            content}
    }

    pub fn update(&mut self, content: String) {
        self.content = content;
        self.file.write_all(self.content.as_bytes()).expect("Unable to write file");
        // TODO: Keep track of undo history
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