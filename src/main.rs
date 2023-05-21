use std::io::{stdout, Read, Write};
use rustditor::views::open_file::OpenFileView;
use rustditor::views::view::View;
use termion::async_stdin;
use termion::raw::IntoRawMode;


fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();
    write!(stdout, "\rPress o to open file\n\r").unwrap();
    write!(stdout, "\rPress q to quit\n\r").unwrap();
    loop {

        let b = stdin.next();
        if let Some(Ok(b'q')) = b {
            break; // quit
        }
        if let Some(Ok(b'o')) = b {
            let view = OpenFileView::new();
            view.render();
        }

        stdout.flush().unwrap();
    }
}
