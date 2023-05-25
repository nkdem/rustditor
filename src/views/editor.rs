use std::{error::Error, collections::VecDeque};

use termion::event::Key;

use crate::document::Document;

use super::main_menu::MainMenuView;
use super::view::{View, HandleInputResult};


pub struct EditorView {
    pub filename: String,
    pub cursor : (u16, u16)
}

impl EditorView {
    pub fn new(filename: String) -> Self { Self { filename, cursor: (1,1) } }
}



impl View for EditorView {
    fn generate_rendered_output(&mut self) -> Result<String, Box<dyn Error>> {
        let doc = Document::open(&self.filename);
        let content = doc.content.lines().map(|line| format!("{}\r\n", line)).collect::<String>();
        let cursor_placement = format!("{}", termion::cursor::Goto(self.cursor.0, self.cursor.1));
        let output = format!("{}{}{}{}", termion::clear::All, termion::cursor::Goto(1,1), content, cursor_placement);
        Ok(output)
    }

    fn handle_input(&mut self,key: termion::event::Key) -> Result<VecDeque<HandleInputResult>, Box<dyn Error>> {
        let (width,height) = termion::terminal_size().unwrap();
        match key {
            Key::Ctrl(c) => {
                match c {
                    'w' => Ok(HandleInputResult::singleton(HandleInputResult::View(Box::new(MainMenuView)))),
                    's' => todo!("Save"),
                    _ => Ok(HandleInputResult::singleton(HandleInputResult::Unhandled))
                }
            }
            Key::Right => {
                if self.cursor.0 < width {
                    self.cursor.0 += 1;
                } else {
                    self.cursor.0 = 1;
                    self.cursor.1 += 1;
                }
                Ok(HandleInputResult::singleton(HandleInputResult::Handled))
            },
            Key::Left => {
                if self.cursor.0 > 1 {
                    self.cursor.0 -= 1;
                } else {
                    self.cursor.0 = width;
                    self.cursor.1 -= 1;
                }
                Ok(HandleInputResult::singleton(HandleInputResult::Handled))
            },
            Key::Up => {
                if self.cursor.1 > 1 {
                    self.cursor.1 -= 1;
                } else {
                    self.cursor = (1, 1);
                }
                Ok(HandleInputResult::singleton(HandleInputResult::Handled))
            },
            Key::Down => {
                if self.cursor.1 < height {
                    self.cursor.1 += 1;
                } else {
                    self.cursor = (1,1)
                }
                Ok(HandleInputResult::singleton(HandleInputResult::Handled))
            }
            _ => Ok(HandleInputResult::singleton(HandleInputResult::Handled))
        }
    }

}