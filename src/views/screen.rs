use std::{
    error::Error,
    io::{stdin, stdout, Write},
};

use termion::{input::TermRead, raw::IntoRawMode, screen::IntoAlternateScreen};

use crate::views::status_bar::StatusBarMode;

use super::{
    status_bar::StatusBar,
    view::{HandleInputResult, View},
};

pub struct Screen {
    pub active_view: Box<dyn View>,
    pub stdout: termion::raw::RawTerminal<termion::screen::AlternateScreen<std::io::Stdout>>,
}

impl Screen {
    pub fn new(active_view: Box<dyn View>) -> Self {
        let stdout = stdout()
            .into_alternate_screen()
            .unwrap()
            .into_raw_mode()
            .unwrap();
        Self {
            active_view,
            stdout,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let stdin = stdin();
        let mut bytes = stdin.keys();
        let (width, height) = termion::terminal_size().unwrap();
        let mut status_bar = Box::new(StatusBar::new(width, height));

        loop {
            write!(self.stdout, "{}", self.active_view.generate_rendered_output().unwrap()).unwrap();
            write!(self.stdout, "{}", status_bar.generate_rendered_output().unwrap()).unwrap();
            self.stdout.flush().unwrap();
            
            let key = bytes.next().unwrap().unwrap();

            status_bar.handle_input(key).unwrap();
            let result: HandleInputResult = match status_bar.mode {
                StatusBarMode::AwaitingInput(ref input_mode) => {
                    if input_mode.complete {
                        Some(status_bar.complete_input().unwrap().unwrap())
                    } else {
                    continue
                    }
                },
                _ => {
                    None
                },
            }.unwrap_or_else(|| self.active_view.handle_input(key).unwrap());
            match result {
                HandleInputResult::Quit => return Ok(()),
                HandleInputResult::View(new_view) => {
                    self.active_view = new_view;
                    if let StatusBarMode::AwaitingInput(ref input_mode) = status_bar.mode {
                        if input_mode.complete {
                            status_bar.reset();
                        }
                    }
                }
                HandleInputResult::Input(ref input, f) => {
                    if let StatusBarMode::Inactive = status_bar.mode {
                        status_bar.set_input_prompt(input.clone(), f);
                    } else {
                        panic!("Tried to set input prompt while one was already active");
                    }
                },
                _ => {}
            }
            
        };
    }
}