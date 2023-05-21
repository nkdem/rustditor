use std::io::{ Read, Write, self};
use std::{thread, time};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;
use super::view::View;

pub struct OpenFileView;

impl OpenFileView {
    pub fn new() -> OpenFileView {
        OpenFileView
    }
}

impl View for OpenFileView {
    fn render(&self) {
        let mut screen = io::stdout().into_raw_mode().unwrap().into_alternate_screen().unwrap();
        let mut filename = String::new();
        write!(screen, "{}Filename: {}{}",termion::cursor::Goto(1, 1),filename,termion::cursor::BlinkingBar).unwrap();
        screen.flush().unwrap();
        
        let stdin = io::stdin();
        
        for c in stdin.keys() {
            match c.unwrap() {
                termion::event::Key::Char('\n') => break,
                termion::event::Key::Char(c) => {
                    filename.push(c);
                }
                _ => {}
            }
            write!(screen, "{}Filename: {}{}",termion::cursor::Goto(1, 1), filename, termion::cursor::BlinkingBar).unwrap();
            screen.flush().unwrap();
        }
    }
}