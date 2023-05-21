use rustditor::{config::Config, document::Document};
use termion::raw::IntoRawMode;
use std::{io::{Write, stdout}};

fn main() {
    // Enter raw mode.
    let mut stdout = stdout().into_raw_mode().unwrap();
    
    
    if let Some(config) = Config::new(std::env::args()) {
        let document = Document::open(&config.filename);
        println!("Document: {:?}", document.content);
    } else {
        todo!("Main Screen")
    }

    // Write to stdout (note that we don't use `println!`)
    write!(stdout, "Hey there.").unwrap();
}



