struct Screen {
    surface_: [[char;20];10]
}

impl Screen {
    fn new(fill: Option<char>) -> screen{
        screen { surface_: [[fill.unwrap_or(' ');20];10] }
    }

    fn get_pix(&self, x:usize, y:usize) -> char {
        self.surface_[y][x];
    }

    fn add_pix(&mut self, x:usize, y:usize, fill: char) {
        self.surface_[y][x] = fill;
    }

    fn print_screen(&self) {
        println!("{}{}{}",'+',"-".repeat(20),'+');
        for y in self.surface_ {
            println!("|{}|",y.iter().collect::<String>());
        }
        println!("{}{}{}",'+',"-".repeat(20),'+');
    }
}