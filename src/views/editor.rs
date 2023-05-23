use std::error::Error;
use std::{io::Write};

use termion::event::Key;

use super::main_menu::MainMenuView;
use super::view::{View, HandleInputResult};

type Out = termion::raw::RawTerminal<termion::screen::AlternateScreen<std::io::Stdout>>;

pub struct EditorView {
    pub filename: String,
}

impl View for EditorView {
    fn render(&mut self, out: &mut termion::raw::RawTerminal<termion::screen::AlternateScreen<std::io::Stdout>>) -> Result<(), Box<dyn Error>> {
        write!(out, "{}{}The filename is {}{}CTRL-w to exit", termion::clear::All, termion::cursor::Goto(1,1), self.filename, termion::cursor::Goto(1,2)).unwrap();
        out.flush().unwrap();
        Ok(())
    }

    fn handle_input(&mut self, status_bar: &mut super::status_bar::StatusBar, key: termion::event::Key, out: &mut Out) -> Result<super::view::HandleInputResult, Box<dyn Error>> {
        match key {
            Key::Ctrl(c) => {
                match c {
                    'w' => Ok(HandleInputResult::View(Box::new(MainMenuView))),
                    's' => todo!("Save"),
                    _ => Ok(HandleInputResult::Unhandled)
                }
            }
            Key::Right => {
                todo!("Move cursor right")
            },
            Key::Left => {
                todo!("Move cursor left")
            },
            Key::Up => {
                todo!("Move cursor up")
            },
            Key::Down => {
                todo!("Move cursor down")
            }
            _ => Ok(HandleInputResult::Unhandled)
        }
    }

}