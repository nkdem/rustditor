use std::{error::Error};

use termion::event::Key;

use super::status_bar::StatusBar;
type Out = termion::raw::RawTerminal<termion::screen::AlternateScreen<std::io::Stdout>>;

pub enum HandleInputResult {
    Quit,
    Command, // TODO: Implement commands
    View(Box<dyn View>),
    Handled,
    Unhandled,
    Failure
}

pub trait View {
    fn render(&mut self, out: &mut Out) -> Result<(), Box<dyn Error>>;
    fn handle_input(&mut self, status_bar: &mut StatusBar, key: Key, out: &mut Out) -> Result<HandleInputResult, Box<dyn Error>>;
}