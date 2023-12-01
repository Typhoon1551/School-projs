use game::Screen;
use game::Map;

fn main() {
    let mut win = Screen::new(None);
    let terrain = Map::new(None);

    terrain.add_to_surface(&mut win);
    win.print_screen();
}
