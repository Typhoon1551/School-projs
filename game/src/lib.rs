use rand::random;

const HEIGHT:usize = 10;
const WIDTH:usize = 100;

pub struct Screen {
    surface_: [[char;WIDTH];HEIGHT]
}

impl Screen {
    pub fn new(fill: Option<char>) -> Screen{
        return Screen { 
            surface_: [[fill.unwrap_or(' ');WIDTH];HEIGHT] 
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

pub struct Map {
    map_: [[bool;WIDTH];HEIGHT],
    char_: char,
}

impl Map {
    pub fn new(char_: Option<char>) -> Map {
        let mut map_ = [[false;WIDTH];HEIGHT];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                map_[y][x] = random();
            }
        }
        
        return Map {
            map_,
            char_: char_.unwrap_or('#'),
        }
    }

    pub fn from(map_: [[bool;WIDTH];HEIGHT], char_: Option<char>) -> Map{
        return Map {
            map_ : map_,
            char_: char_.unwrap_or('#'),
        }
    }

    pub fn add_to_surface(&self, screen: &mut Screen) -> u32 {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.map_[y][x] {
                    screen.add_pix(x, y, self.char_);
                }
            }
        }
        return 0;
    }
}