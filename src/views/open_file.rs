use termion::{clear, cursor::Goto};

use super::view::View;
use std::io::{Write, stdin, Read};

pub struct OpenFileView;

impl View for OpenFileView {
    fn render(&self, out: &mut impl Write) {
        write!(out, "{}{}Filename: ", clear::All, Goto(1,1)).unwrap();
        out.flush().unwrap();
        let stdin = stdin();
        let mut bytes = stdin.bytes();
        loop {
            let b = bytes.next().unwrap().unwrap();
            match b {
                b'\n' => return,
                b => write!(out, "{}", b as char).unwrap(),
            }
            out.flush().unwrap();
        }
    }
}