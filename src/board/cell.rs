use super::{color::Color, piece::chesspiece::ChessPiece};

#[derive(Debug)]
pub struct Cell {
    pub space: Option<Box<dyn ChessPiece>>,
    pub color: Color,
    pub x: char,
    pub y: u32,
}

impl Cell {
    pub fn new(x: char, y: u32, color: Color, space: Option<Box<dyn ChessPiece>>) -> Self {
        Cell { x, y, color, space }
    }
}
