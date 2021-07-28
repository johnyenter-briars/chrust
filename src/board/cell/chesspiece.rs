use super::color::Color;
use super::piecetype::PieceType;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ChessPiece {
    pub piece_type: PieceType,
    pub color: Color,
}