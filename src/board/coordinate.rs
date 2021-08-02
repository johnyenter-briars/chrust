use core::panic;

#[derive(Debug, Clone, Copy)]
pub struct Coordinate { 
    pub x: char, //a-h
    pub y: i32, //1-8
}

impl Coordinate {
    pub fn new(x: char, y: i32) -> Self {
        if ! (1..9).any(|ele| ele == y) {
            panic!("Invalid y value: {}", y);
        }
        
        if ! "abcdefgh".to_string().chars().into_iter().any(|ele| ele == x) {
            panic!("Invalid x value: {}", x);
        }

        Coordinate{x, y}
    }

    pub fn forward_by(&self, ammount: i32) -> Self {
        Coordinate::new(self.x, self.y + ammount)
    }

    pub fn back_by(&self, ammount: i32) -> Self {
        Coordinate::new(self.x, self.y - ammount)
    }
}

fn inc_x(x: char) -> char {
    match x {
        'a' => 'b',
        'b' => 'c',
        'c' => 'd',
        'e' => 'f',
        'f' => 'g', 
        'g' => 'h',
        _ => panic!("You have an invalid x value: {}", x)
    }
}
