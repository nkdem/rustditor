use termion::input::TermRead;
use std::{io::Write};

type Out = termion::raw::RawTerminal<termion::screen::AlternateScreen<std::io::Stdout>>;

pub struct StatusBar {
    width: u16,
    height: u16,
    active: bool,
}

impl StatusBar {
    pub fn new() -> Self {
        let (width, height) = termion::terminal_size().unwrap();
        Self {
            width,
            height,
            active: false,
        }
    }
    pub fn toggle(&mut self) {
        self.active = !self.active;
    }
    
    pub fn get_input(&mut self, input_type: String, out: &mut Out) -> Result<String, Box<dyn std::error::Error>> {
        write!(out, "{}{}: ", termion::cursor::Goto(1, self.height), input_type)?;
        out.flush()?;
        let stdin = std::io::stdin();
        let mut bytes = stdin.keys();

        let mut string = String::new();
        loop {
            match bytes.next().unwrap()? {
                termion::event::Key::Esc => return Ok(String::new()),
                termion::event::Key::Char('\n') => return Ok(string),
                termion::event::Key::Char(c) => {
                    string.push(c);
                    write!(out, "{}", c)?;
                },
                termion::event::Key::Backspace => {
                    string.pop();
                    write!(out, "{}{} ", termion::cursor::Left(1), termion::cursor::Left(1))?;
                },
                _ => {},
            }
            out.flush()?;
        }
        
    }
}

