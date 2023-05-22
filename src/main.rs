use rustditor::views::{main_menu::MainMenuView, view::View};
use termion::raw::IntoRawMode;
use std::io::{stdout};

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.into_raw_mode().unwrap();

    MainMenuView.render(&mut stdout);
}