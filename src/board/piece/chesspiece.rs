use std::fmt::Debug;

use crate::board::{chessboard::ChessBoard, coordinate::Coordinate, piecetype::PieceType, color::Color};

pub trait ChessPiece : Debug {
    fn available_moves(
        &self,
        board: &ChessBoard,
    ) -> Vec<Coordinate>;
    fn fen_code(&self) -> &str;
    fn str(&self) -> &str;
    fn piece_type(&self) -> PieceType;
    fn coordinate(&self) -> Coordinate;
    fn color(&self) -> Color;
}
