use std::{error::Error};

use termion::{event::Key};

use super::view::{HandleInputResult, View};

type Input = String;

pub struct StatusBar {
    width: u16,
    height: u16,
    pub mode: StatusBarMode,
    output: String,
    input_handler: Option<super::view::InputHandler>
}

pub enum StatusBarMode {
    Inactive,
    AwaitingInput(Input),
    AwaitingCollection
}

impl StatusBar {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            mode: StatusBarMode::Inactive,
            output: String::new(),
            input_handler: None
        }
    }

    pub fn set_input_prompt(&mut self, input: Input, handler: super::view::InputHandler) {
        self.mode = StatusBarMode::AwaitingInput(input);
        self.input_handler = Some(handler);
    }

    pub fn reset(&mut self) {
        self.mode = StatusBarMode::Inactive;
        self.output.clear();
        self.input_handler = None;
    }

    pub fn complete_input(&mut self) -> Option<Result<HandleInputResult, Box<dyn Error>>> {
        if let StatusBarMode::AwaitingCollection = self.mode {
            Some(self.input_handler.unwrap()(self.output.clone()))
        } else {
            None
        }
    }
}

impl View for StatusBar {
    fn generate_rendered_output(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        match &self.mode {
            StatusBarMode::Inactive => Ok(String::new()),
            StatusBarMode::AwaitingInput(input) => Ok(format!(
                "{}{}{}: {}",
                termion::cursor::Goto(1, self.height),
                termion::clear::CurrentLine,
                input,
                self.output
            )),
            StatusBarMode::AwaitingCollection => Ok(format!(
                "{}{}",
                termion::cursor::Goto(1, self.height),
                termion::clear::CurrentLine
            )),
        }
    }

    fn handle_input(
        &mut self,
        key: termion::event::Key,
    ) -> Result<HandleInputResult,
        Box<dyn std::error::Error>,
    > {
        match self.mode {
            StatusBarMode::AwaitingInput(_) => {
                match key {
                    Key::Esc => {
                        self.mode = StatusBarMode::Inactive;
                        self.output.clear(); 
                    }
                    Key::Char('\n') => {
                        self.mode = StatusBarMode::AwaitingCollection
                    }
                    Key::Char(c) => {
                        self.output.push(c);
                    }
                    Key::Backspace => {
                        if !self.output.is_empty() {
                            self.output.pop();
                        }
                    }
                    _ => {}
                }
                Ok(HandleInputResult::Handled)
            }
            _ => Ok(HandleInputResult::Unhandled),
        }
    }
}
