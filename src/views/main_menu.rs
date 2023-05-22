use std::io::{stdin, Read, Write};

use termion::{input::TermRead, event::Key};

use crate::document::Document;

use super::{open_file::OpenFileView, view::View};

pub struct MainMenuView;

fn main_menu_message(out: &mut impl Write) {
    write!(
        out,
        "{}{}{}Press 'o' to open file, or q to exit program{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::style::Bold,
        termion::style::Reset,
        termion::cursor::Goto(1, 2)
    )
    .unwrap();
    out.flush().unwrap();
}

impl View for MainMenuView {
    type Output = ();
    fn render(&self, out: &mut impl Write) {
        let stdin = stdin();

        let mut bytes = stdin.keys();
        main_menu_message(out);
        loop {
            let b = bytes.next().unwrap().unwrap();

            match b {
                // Quit
                Key::Char('q') => break,
                Key::Char('o') => {
                    let filename = OpenFileView.render(out);
                    let doc = Document::open(&filename);
                    main_menu_message(out);
                }
                _ => continue,
            }

            out.flush().unwrap();
        }
    }
}