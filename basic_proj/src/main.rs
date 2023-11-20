use rand;

fn main() {
    const SIZE_X:usize = 1000;
    const SIZE_Y:usize = 1000;

    let mut map:[[char; SIZE_X]; SIZE_Y] = [[' ';SIZE_X];SIZE_Y];

    for x in map.iter_mut() {
        for y in x.iter_mut() {
            *y = if rand::random::<bool>() {'#'} else {' '};
        }
    }
    
    for x in map.iter() {
        for y in x.iter() {
            print!("{}", y);
        }
        println!("");
    }
}
