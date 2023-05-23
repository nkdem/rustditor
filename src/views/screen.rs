use std::{
    error::Error,
    io::{stdin, stdout, Write},
    rc::Rc,
    sync::Mutex,
};

use termion::{input::TermRead, raw::IntoRawMode, screen::IntoAlternateScreen};

use super::{
    status_bar::StatusBar,
    view::{View, HandleInputResult},
};

pub struct Screen {
    pub active_view: Box<dyn View>,
    pub stdout: termion::raw::RawTerminal<termion::screen::AlternateScreen<std::io::Stdout>>
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
            stdout
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let stdin = stdin();
        let mut bytes = stdin.keys();
        let mut status_bar = StatusBar::new();
        status_bar.toggle(); // Activate status bar
        loop {
            self.active_view.render(&mut self.stdout)?;
            let key = bytes.next().unwrap().unwrap();
            match self.active_view.handle_input(&mut status_bar, key, &mut self.stdout)? {
                HandleInputResult::Quit => return Ok(()),
                HandleInputResult::View(new_view) => {
                    self.active_view = new_view;
                },
                _ => {},
            }
        }
    }
}
