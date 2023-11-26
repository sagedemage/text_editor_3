use std::fs::File;
use std::io::{BufReader, Write, Read};
use std::path::PathBuf;

pub struct FileStream {}

impl FileStream {
    pub fn open(filename: PathBuf) -> String {
        let file = File::open(filename).expect("Couldn't open file");

        let mut reader = BufReader::new(file);
        let mut text_content = String::new();
        let _ = reader.read_to_string(&mut text_content);

        text_content
    }

    pub fn save(filename: PathBuf, text_content: &str) {
        let mut file = File::create(filename).expect("Couldn't open file");

        file.write_all(text_content.as_bytes())
            .expect("Could not save file");
    }
}
