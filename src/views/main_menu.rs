use std::{error::Error, collections::{VecDeque, HashMap}};
use termion::event::Key;

use crate::views::editor::EditorView;

use super::{
    view::{HandleInputResult, View, self},
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
    ) -> Result<VecDeque<HandleInputResult>, Box<dyn Error>> {
        match key {
            termion::event::Key::Char('o') => {
                let inputs = vec!["Filename".to_string()];
                let callback: view::InputHandler = |hashmap| {
                    let filename = hashmap.get("Filename").unwrap();
                    if filename.len() > 0 {
                        Ok(HandleInputResult::singleton(HandleInputResult::View(Box::new(EditorView::new(filename.to_string())))))
                    } else {
                        Ok(HandleInputResult::singleton(HandleInputResult::Failure))
                    }
                };

                Ok(HandleInputResult::singleton(HandleInputResult::Input(inputs,callback)))
            }
            termion::event::Key::Char('q') => Ok(HandleInputResult::singleton(HandleInputResult::Quit)),
            _ => Ok(HandleInputResult::singleton(HandleInputResult::Unhandled)),
        }
    }
}