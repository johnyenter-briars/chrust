use crate::board::{coordinate::Coordinate, piece::chesspiece::ChessPiece};


#[derive(Clone, Copy)]
pub struct PieceMove<'a> {
    pub chess_piece: &'a dyn ChessPiece,
    pub from: Coordinate,
    pub to: Coordinate,
}

impl<'a> PieceMove<'a> {
    pub fn new(chess_piece: &'a dyn ChessPiece, from: Coordinate, to: Coordinate) -> Self {
        PieceMove {
            chess_piece,
            from,
            to,
        }
    }
}
