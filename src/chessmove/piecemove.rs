use crate::board::{piece::chesspiece::ChessPiece, coordinate::Coordinate};


#[derive(Debug, Clone, Copy)]
pub struct PieceMove<'a> {
    pub chess_piece: &'a ChessPiece,
    pub from: Coordinate,
    pub to: Coordinate,
}

impl<'a> PieceMove<'a> {
    pub fn new(chess_piece: &'a ChessPiece, from: Coordinate, to: Coordinate) -> Self {
        PieceMove {
            chess_piece,
            from,
            to,
        }
    }
}
