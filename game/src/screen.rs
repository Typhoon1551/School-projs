
pub struct Screen {
    surface_: [[char;100];10]
}

impl Screen {
    pub fn new(fill: Option<char>) -> Screen{
        return Screen { 
            surface_: [[fill.unwrap_or(' ');100];10] 
        };
    }

    pub fn get_pix(&self, x:usize, y:usize) -> char {
        return self.surface_[y][x];
    }

    pub fn add_pix(&mut self, x:usize, y:usize, fill: char) -> u32{
        self.surface_[y][x] = fill;
        return 0;
    }

    pub fn print_screen(&self) -> u32{
        println!("{}{}{}",'+',"-".repeat(100),'+');
        for y in self.surface_ {
            println!("|{}|",y.iter().collect::<String>());
        }
        println!("{}{}{}",'+',"-".repeat(100),'+');
        return 0;
    }

    pub fn clear_screen(&mut self) -> u32{
        self.surface_ = [[' ';100];10];
        return 0;
    }
}