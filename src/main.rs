use rustditor::views::{ screen::Screen, main_menu::MainMenuView};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut screen = Screen::new(Box::new(MainMenuView));
    screen.run().unwrap();
    Ok(())
}