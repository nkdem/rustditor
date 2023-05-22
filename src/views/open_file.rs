use termion::screen::{IntoAlternateScreen};

use super::view::View;
use std::io::{Write};

pub struct OpenFileView;

impl View for OpenFileView {
    fn render(&self, out: &mut impl Write) {
        let mut screen = out.into_alternate_screen().unwrap();
        writeln!(screen, "Hello").unwrap();
        screen.flush().unwrap();
    }
}