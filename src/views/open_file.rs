use termion::{clear, cursor::Goto, input::TermRead, event::Key};

use super::view::View;
use std::io::{Write, stdin, Read};

pub struct OpenFileView;

impl View for OpenFileView {
    type Output = String;
    fn render(&self, out: &mut impl Write) -> Self::Output {
        write!(out, "{}{}Filename: ", clear::All, Goto(1,1)).unwrap();
        out.flush().unwrap();
        let stdin = stdin();
        let mut bytes = stdin.keys();
        let mut filename = String::new();
        loop {
            let b = bytes.next().unwrap().unwrap();
            match b {
                Key::Char('\n') => return filename,
                Key::Backspace => {
                    filename.pop();
                    write!(out, "{}{}Filename: {}", clear::All, Goto(1,1), filename).unwrap();
                },
                Key::Char(b) => {
                    filename.push(b as char);
                    write!(out, "{}", b as char).unwrap();
                },
                _ => continue,
            }
            out.flush().unwrap();
        }
    }
}