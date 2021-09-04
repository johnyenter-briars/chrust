use std::fmt;

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Color {
    White = 1,
    Black = 2,
}

impl Color {
    pub fn opposite_color(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
    pub fn color_abbr(&self) -> char {
        self.to_string().chars().next().unwrap()
    }
}


impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}