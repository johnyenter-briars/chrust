pub mod chesspiece;
pub mod color;
pub mod piecetype;

use std::cell::RefCell;

use chesspiece::ChessPiece;
use color::Color;

#[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Cell{
    pub space: Option<ChessPiece>,
    pub color: Color,
    pub x: char,
    pub y: i32,
}
