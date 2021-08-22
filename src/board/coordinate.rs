use core::panic;

use super::chessboard::Board;

#[derive(Debug, Clone, Copy)]
pub struct Coordinate {
    pub x: char, //a-h
    pub y: i32,  //1-8
}

//NO ERROR HANDLING - call is_valid if you want to test validity of a coordinate
impl Coordinate {
    pub fn new(x: char, y: i32) -> Self {
        Coordinate { x, y }
    }

    pub fn is_valid(&self, board: &Board) -> bool {
        (1..9).any(|ele| ele == self.y)
            && "abcdefgh"
                .to_string()
                .chars()
                .into_iter()
                .any(|ele| ele == self.x)
            && !board.test_cell_at(*self).is_none()
    }

    pub fn up_by(&self, ammount: i32) -> Self {
        Coordinate::new(self.x, self.y + ammount)
    }

    pub fn down_by(&self, ammount: i32) -> Self {
        Coordinate::new(self.x, self.y - ammount)
    }

    pub fn left_by(&self, ammount: i32) -> Self {
        Coordinate::new((self.x as u8 - ammount as u8) as char, self.y)
    }

    pub fn right_by(&self, ammount: i32) -> Self {
        Coordinate::new((self.x as u8 + ammount as u8) as char, self.y)
    }

    pub fn diagonal_up_right_by(&self, ammount: i32) -> Self {
        Coordinate::new((self.x as u8 + ammount as u8) as char, self.y + ammount)
    }

    pub fn diagonal_up_left_by(&self, ammount: i32) -> Self {
        Coordinate::new((self.x as u8 - ammount as u8) as char, self.y + ammount)
    }

    pub fn diagonal_down_right_by(&self, ammount: i32) -> Self {
        Coordinate::new((self.x as u8 + ammount as u8) as char, self.y - ammount)
    }

    pub fn diagonal_down_left_by(&self, ammount: i32) -> Self {
        Coordinate::new((self.x as u8 - ammount as u8) as char, self.y - ammount)
    }
}
