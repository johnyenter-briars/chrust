#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Color {
    White = 1,
    Black = 2,
}

impl Color {
    pub fn get_opposite_color(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}