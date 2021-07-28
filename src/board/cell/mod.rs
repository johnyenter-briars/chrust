pub mod chesspiece;
pub mod color;
pub mod piecetype;

use chesspiece::ChessPiece;
use color::Color;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Cell{
    pub space: Option<ChessPiece>,
    pub color: Color,
}
