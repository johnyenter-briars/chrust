use crate::board::coordinate::Coordinate;

pub trait ToCoord {
    fn to_coord(&self) -> Coordinate;
}

impl ToCoord for std::string::String {
    fn to_coord(&self) -> Coordinate {
        let x: char = self.chars().nth(0).unwrap();
        let y: u32 = self.chars().nth(1).unwrap().to_digit(10).unwrap();
        Coordinate { x, y: y as i32 }
    }
}
