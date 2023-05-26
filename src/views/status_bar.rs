use std::error::Error;

use termion::event::Key;

use super::view::{HandleInputResult, InputHandler, View};

pub struct InputMode {
    pub input: String,
    pub on_complete: InputHandler,
    pub output: String,
    pub complete: bool,
}

pub struct StatusBar {
    width: u16,
    height: u16,
    pub mode: StatusBarMode,
}

pub enum StatusBarMode {
    Inactive,
    AwaitingInput(InputMode),
}

impl StatusBar {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            mode: StatusBarMode::Inactive,
        }
    }

    pub fn set_input_prompt(&mut self, input: String, handler: super::view::InputHandler) {
        let input_mode = InputMode {
            input,
            on_complete: handler,
            output: String::new(),
            complete: false,
        };
        self.mode = StatusBarMode::AwaitingInput(input_mode);
    }

    pub fn reset(&mut self) {
        self.mode = StatusBarMode::Inactive;
    }

    pub fn complete_input(&mut self) -> Option<Result<HandleInputResult, Box<dyn Error>>> {
        match &mut self.mode {
            StatusBarMode::AwaitingInput(input_mode) => {
                input_mode.complete = true;
                let res = (input_mode.on_complete)(input_mode.output.clone());
                Some(res)
            }
            _ => None,
        }
    }
}

impl View for StatusBar {
    fn generate_rendered_output(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        match &self.mode {
            StatusBarMode::Inactive => Ok(String::new()),
            StatusBarMode::AwaitingInput(input_mode) => Ok(format!(
                "{}{}{}: {}",
                termion::cursor::Goto(1, self.height),
                termion::clear::CurrentLine,
                input_mode.input,
                input_mode.output
            )),
        }
    }

    fn handle_input(
        &mut self,
        key: termion::event::Key,
    ) -> Result<HandleInputResult, Box<dyn std::error::Error>> {
        match self.mode {
            StatusBarMode::AwaitingInput(ref mut input_mode) => {
                match key {
                    Key::Esc => {
                        self.mode = StatusBarMode::Inactive;
                    }
                    Key::Char('\n') => input_mode.complete = true,
                    Key::Char(c) => {
                        input_mode.output.push(c);
                    }
                    Key::Backspace => {
                        if !input_mode.output.is_empty() {
                            input_mode.output.pop();
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
