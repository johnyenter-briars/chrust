use std::vec;


// use super::{chesspiece::ChessPiece, color::Color};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PieceType {
    Pawn = 1,
    Rook = 2,
    Knight = 3,
    Bishop = 4,
    Queen = 5,
    King = 6,
}