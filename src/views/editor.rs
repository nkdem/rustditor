use std::error::Error;
use std::{io::Write};

use termion::event::Key;

use crate::document::Document;

use super::main_menu::MainMenuView;
use super::view::{View, HandleInputResult};

type Out = termion::raw::RawTerminal<termion::screen::AlternateScreen<std::io::Stdout>>;

pub struct EditorView {
    pub filename: String,
    pub cursor : (u16, u16)
}

impl EditorView {
    pub fn new(filename: String) -> Self { Self { filename, cursor: (1,1) } }
}



impl View for EditorView {
    fn render(&mut self, out: &mut termion::raw::RawTerminal<termion::screen::AlternateScreen<std::io::Stdout>>) -> Result<(), Box<dyn Error>> {
        let doc = Document::open(&self.filename);
        
        // File content
        write!(out, "{}{}", termion::clear::All,termion::cursor::Goto(1,1)).unwrap();
        doc.content.lines().for_each(|line| {
            write!(out, "{}\r\n", line).unwrap();
        });

        // Cursor placement
        write!(out, "{}", termion::cursor::Goto(self.cursor.0, self.cursor.1)).unwrap();
        out.flush().unwrap();


        Ok(())
    }

    fn handle_input(&mut self, status_bar: &mut super::status_bar::StatusBar, key: termion::event::Key, out: &mut Out) -> Result<super::view::HandleInputResult, Box<dyn Error>> {
        let (width,height) = termion::terminal_size().unwrap();
        match key {
            Key::Ctrl(c) => {
                match c {
                    'w' => Ok(HandleInputResult::View(Box::new(MainMenuView))),
                    's' => todo!("Save"),
                    _ => Ok(HandleInputResult::Unhandled)
                }
            }
            Key::Right => {
                if self.cursor.0 < width {
                    self.cursor.0 += 1;
                } else {
                    self.cursor.0 = 1;
                    self.cursor.1 += 1;
                }
                Ok(HandleInputResult::Handled)
            },
            Key::Left => {
                if self.cursor.0 > 1 {
                    self.cursor.0 -= 1;
                } else {
                    self.cursor.0 = width;
                    self.cursor.1 -= 1;
                }
                Ok(HandleInputResult::Handled)
            },
            Key::Up => {
                if self.cursor.1 > 1 {
                    self.cursor.1 -= 1;
                } else {
                    self.cursor = (1, 1);
                }
                Ok(HandleInputResult::Handled)
            },
            Key::Down => {
                if self.cursor.1 < height {
                    self.cursor.1 += 1;
                } else {
                    self.cursor = (1,1)
                }
                Ok(HandleInputResult::Handled)
            }
            _ => Ok(HandleInputResult::Unhandled)
        }
    }

}