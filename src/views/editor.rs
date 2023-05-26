use std::error::Error;

use log::trace;
use termion::event::Key;

use crate::document::Document;

use super::{view::{HandleInputResult, View}, status_bar::{EditorMode, StatusBarMode}};
use super::{
    main_menu::MainMenuView,
    // status_bar::{EditorMode, StatusBarMode},
    view::Request,
};

pub struct EditorView {
    pub filename: String,
    pub cursor: (u16, u16),
    doc: Document,
}

impl EditorView {
    pub fn new(filename: String) -> Self {
        let doc = Document::open(&filename);
        Self {
            filename,
            cursor: (1, 1),
            doc
        }
    }
}

impl View for EditorView {

    fn init(&self) -> Option<HandleInputResult>{
        // TODO: Need to fix so that cursor are in sync in status_bar and here
        let editor_mode = EditorMode {
            cursor: (self.cursor.0, self.cursor.1),
            filename: self.filename.clone(),
        };
        Some(HandleInputResult::Request(Request::ChangeStatusBarMode(
            StatusBarMode::EditorMode(editor_mode),
        )))
    }

    fn generate_rendered_output(&mut self) -> Result<String, Box<dyn Error>> {
        let content = self.doc
            .content
            .lines()
            .map(|line| format!("{}\r\n", line))
            .collect::<String>();
        let cursor_placement = format!("{}", termion::cursor::Goto(self.cursor.0, self.cursor.1));
        let output = format!(
            "{}{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            content,
            cursor_placement
        );
        Ok(output)
    }

    fn handle_input(
        &mut self,
        key: termion::event::Key,
    ) -> Result<HandleInputResult, Box<dyn Error>> {
        let (width, height) = termion::terminal_size().unwrap();
        match key {
            Key::Ctrl(c) => match c {
                'w' => Ok(HandleInputResult::Request(Request::UpdateView(Box::new(
                    MainMenuView,
                )))),
                's' => todo!("Save"),
                _ => Ok(HandleInputResult::Unhandled),
            },
            Key::Right => {
                let old = self.cursor;
                if self.cursor.0 < width {
                    self.cursor.0 += 1;
                } else {
                    self.cursor.0 = 1;
                    self.cursor.1 += 1;
                }
                trace!("Moving cursor \u{2192} [before {:?}] [after {:?}]", old, self.cursor);
                Ok(HandleInputResult::Request(Request::UpdateStatusBar(EditorMode {
                    cursor: (self.cursor.0, self.cursor.1),
                    filename: self.filename.clone(),
                })))
            }
            Key::Left => {
                if self.cursor.0 > 1 {
                    self.cursor.0 -= 1;
                } else {
                    self.cursor.0 = width;
                    self.cursor.1 -= 1;
                }
                Ok(HandleInputResult::Request(Request::UpdateStatusBar(EditorMode {
                    cursor: (self.cursor.0, self.cursor.1),
                    filename: self.filename.clone(),
                })))
            }
            Key::Up => {
                if self.cursor.1 > 1 {
                    self.cursor.1 -= 1;
                } else {
                    self.cursor = (1, 1);
                }
                Ok(HandleInputResult::Request(Request::UpdateStatusBar(EditorMode {
                    cursor: (self.cursor.0, self.cursor.1),
                    filename: self.filename.clone(),
                })))
            }
            Key::Down => {
                if self.cursor.1 < height {
                    self.cursor.1 += 1;
                } else {
                    self.cursor = (1, 1)
                }
                Ok(HandleInputResult::Request(Request::UpdateStatusBar(EditorMode {
                    cursor: (self.cursor.0, self.cursor.1),
                    filename: self.filename.clone(),
                })))
            }
            _ => Ok(HandleInputResult::Handled),
        }
    }
}
