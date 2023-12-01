use crate::screen::Screen;

pub mod screen;

fn main() {
    let mut win = Screen::new(None);
    win.print_screen();
}
