use std::{error::Error, collections::{VecDeque}};

use termion::event::Key;

use super::status_bar::{StatusBarMode, EditorMode, StatusBar};

pub type InputHandler = fn(String) -> Result<HandleInputResult, Box<dyn Error>>;

pub enum Request {
    UpdateView(Box<dyn View>),
    InputPrompt(String, InputHandler),
    ChangeStatusBarMode(StatusBarMode),
    UpdateStatusBar(Box<dyn FnOnce(&mut StatusBar)>),
}

pub enum HandleInputResult {
    Quit,
    Command, // TODO: Implement commands
    // View(Box<dyn View>),
    // Input(String, InputHandler),
    Request(Request),
    Handled,
    Unhandled,
    Failure,
}

impl HandleInputResult {
    pub fn singleton(self) -> VecDeque<Self> {
        VecDeque::from(vec![self])
    }
}


pub trait View {
    fn generate_rendered_output(&mut self) -> Result<String, Box<dyn Error>>;
    fn handle_input(&mut self, key: Key) -> Result<HandleInputResult, Box<dyn Error>>;
    fn init(&self) -> Option<HandleInputResult>; // For when a View needs to do something on init e.g. set the status bar mode
}