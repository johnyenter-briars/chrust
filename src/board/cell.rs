use super::{color::Color, piece::chesspiece::ChessPiece};

#[derive(Debug)]
pub struct Cell<'a> {
    pub space: Option<&'a dyn ChessPiece>,
    pub color: Color,
    pub x: char,
    pub y: i32,
}

impl<'a> Cell<'a> {
    pub fn new(x: char, y: i32, color: Color, space: Option<&dyn ChessPiece>) -> Self {
        Cell { x, y, color, space }
    }
}
