use std::error::Error;
use std::{io::Write};

use super::view::View;

type Out = termion::raw::RawTerminal<termion::screen::AlternateScreen<std::io::Stdout>>;

pub struct EditorView {
    pub filename: String,
}

impl View for EditorView {
    fn render(&mut self, out: &mut termion::raw::RawTerminal<termion::screen::AlternateScreen<std::io::Stdout>>) -> Result<(), Box<dyn Error>> {
        write!(out, "{}{}The filename is {}", termion::clear::All, termion::cursor::Goto(1,1), self.filename).unwrap();
        out.flush().unwrap();
        Ok(())
    }

    fn handle_input(&mut self, status_bar: &mut super::status_bar::StatusBar, key: termion::event::Key, out: &mut Out) -> Result<super::view::HandleInputResult, Box<dyn Error>> {
        todo!()
    }

}