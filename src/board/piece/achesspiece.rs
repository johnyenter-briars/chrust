use crate::board::{chessboard::Board, coordinate::Coordinate};

use super::chesspiece::ChessPiece;

pub trait AChessPiece {
    fn available_moves(
        &self,
        board: &Board,
    ) -> Vec<Coordinate>;
    fn fen_code(&self) -> &str;
    fn str(&self) -> &str;
}
