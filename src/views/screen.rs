use std::{
    collections::HashMap,
    error::Error,
    io::{stdin, stdout, Write},
};

use termion::{input::TermRead, raw::IntoRawMode, screen::IntoAlternateScreen};

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
        let mut status_bar = StatusBar::new();
        status_bar.toggle(); // Activate status bar
        loop {
            write!(self.stdout, "{}", self.active_view.generate_rendered_output().unwrap());
            self.stdout.flush().unwrap();
            let key = bytes.next().unwrap().unwrap();
            let mut queue = self.active_view.handle_input(key).unwrap();

            while !queue.is_empty() {
                let result = queue.pop_front().unwrap();
                match result {
                    HandleInputResult::Quit => return Ok(()),
                    HandleInputResult::View(new_view) => {
                        self.active_view = new_view;
                    }
                    HandleInputResult::Input(inputs, f) => {
                        let mut hashmap = HashMap::new();
                        for input in inputs {
                            // TODO: Need to fix this so that status bar does not get out
                            // Thinking of maybe making status_bar a view 
                            // So we make active_view the status_bar
                            // and we add to the queue the old view so that we can go back to it
                            let result = status_bar.get_input(input.clone(), &mut self.stdout); 
                                if let Ok(result) = result {
                                    hashmap.insert(input, result);
                                }
                        }
                        queue.extend(f(hashmap).unwrap());
                    }
                    _ => {}
                }
            }
        }
    }
}