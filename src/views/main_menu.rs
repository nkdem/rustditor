use std::io::{Write, stdin, Read};

use super::{view::View, open_file::OpenFileView};

pub struct MainMenuView;

impl View for MainMenuView {
    fn render(&self, out: &mut impl Write) {
        let stdin = stdin();

        write!(out,
               "{}{}{}Press 'o' to open file, or q to exit program{}{}",
               termion::clear::All,
               termion::cursor::Goto(1, 1),
               termion::style::Bold,
               termion::style::Reset,
               termion::cursor::Goto(1, 2))
                .unwrap();
        out.flush().unwrap();
    
        let mut bytes = stdin.bytes();
        loop {
            let b = bytes.next().unwrap().unwrap();
    
            match b {
                    // Quit
                    b'q' => return,
                    b'o' => {
                        OpenFileView.render(out);
                    }
                    _ => continue,
                }
    
            out.flush().unwrap();
        }
    }
}