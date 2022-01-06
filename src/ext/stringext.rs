use crate::board::coordinate::Coordinate;

pub trait ToCoord {
    fn to_coord(&self) -> Coordinate;
}

impl ToCoord for std::string::String {
    fn to_coord(&self) -> Coordinate {
        let mut chars = self.chars();
        let x: char = chars.next().unwrap();
        let y: u32 = chars.next().unwrap().to_digit(10).unwrap();
        Coordinate { x, y: y as i32 }
    }
}
