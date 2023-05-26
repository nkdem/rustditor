use std::{error::Error, collections::{VecDeque}};

use termion::event::Key;

pub type InputHandler = fn(String) -> Result<HandleInputResult, Box<dyn Error>>;

pub enum HandleInputResult {
    Quit,
    Command, // TODO: Implement commands
    View(Box<dyn View>),
    Input(String, InputHandler),
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
}