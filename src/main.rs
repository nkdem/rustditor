use rustditor::{views::{ main_menu::MainMenuView}, screen::Screen};
use log::{LevelFilter};
use std::fs::OpenOptions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("trace.log")?;

    let logger = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Trace)
        .chain(log_file);

    logger.apply()?;
    let mut screen = Screen::new(Box::new(MainMenuView));
    screen.run().unwrap();
    Ok(())
}