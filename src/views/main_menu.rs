use std::{error::Error, io::Write};
use termion::event::Key;

use crate::views::editor::EditorView;

use super::{
    status_bar::StatusBar,
    view::{HandleInputResult, View},
};
type Out = termion::raw::RawTerminal<termion::screen::AlternateScreen<std::io::Stdout>>;

pub struct MainMenuView;
impl View for MainMenuView {
    fn render(&mut self, out: &mut Out) -> Result<(), Box<dyn Error>> {
        write!(
            out,
            "{}{}{}Press 'o' to open file, or q to exit program{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::style::Bold,
            termion::style::Reset,
            termion::cursor::Goto(1, 2)
        )
        .unwrap();
        out.flush().unwrap();
        Ok(())
    }

    fn handle_input(
        &mut self,
        status_bar: &mut StatusBar,
        key: Key,
        out: &mut Out,
    ) -> Result<HandleInputResult, Box<dyn Error>> {
        match key {
            termion::event::Key::Char('o') => {
                let filename = status_bar.get_input("Filename".to_string(), out)?;
                if filename.len() > 0 {
                    return Ok(HandleInputResult::View(Box::new(EditorView::new(filename))));
                } else {
                    Ok(HandleInputResult::Failure)
                }
            }
            termion::event::Key::Char('q') => Ok(HandleInputResult::Quit),
            _ => Ok(HandleInputResult::Unhandled),
        }
    }
}