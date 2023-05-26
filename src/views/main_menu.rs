use std::{error::Error};
use termion::event::Key;

use crate::views::editor::EditorView;

use super::{
    view::{HandleInputResult, View, self, Request},
};

pub struct MainMenuView;
impl View for MainMenuView {
    fn generate_rendered_output(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(format!(
            "{}{}{}Press 'o' to open file, or q to exit program{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::style::Bold,
            termion::style::Reset,
            termion::cursor::Goto(1, 2)
        ))
    }

    fn handle_input(
        &mut self,
        key: Key,
    ) -> Result<HandleInputResult, Box<dyn Error>> {
        match key {
            termion::event::Key::Char('o') => {
                let callback: view::InputHandler = |filename| {
                    if !filename.is_empty() {
                        Ok(HandleInputResult::Request(Request::UpdateView(Box::new(EditorView::new(filename)))))
                    } else {
                        Ok(HandleInputResult::Failure)
                    }
                };
                Ok(HandleInputResult::Request(Request::InputPrompt("Filename".to_string(),callback)))
            }
            termion::event::Key::Char('q') => Ok(HandleInputResult::Quit),
            _ => Ok(HandleInputResult::Unhandled),
        }
    }

    fn init(&self) -> Option<HandleInputResult> {
        None
    }
    
}