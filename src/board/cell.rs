use super::{piece::chesspiece::ChessPiece, color::Color};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cell {
    pub space: Option<ChessPiece>,
    pub color: Color,
    pub x: char,
    pub y: i32,
}