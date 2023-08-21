use crate::board::coordinate::Coordinate;

pub trait ToCoord {
    fn to_coord(&self) -> Coordinate;
}

impl ToCoord for std::string::String {
    /// Returns a `Coordinate` object equal to the string representation of 'a7'
    fn to_coord(&self) -> Coordinate {
        let mut chars = self.chars();
        let x: char = chars.next().unwrap();
        let y: i32 = chars
            .next()
            .unwrap()
            .to_digit(10)
            .unwrap()
            .try_into()
            .unwrap();
        Coordinate { x, y }
    }
}
