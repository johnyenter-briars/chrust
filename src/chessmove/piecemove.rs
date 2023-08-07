use crate::board::{coordinate::Coordinate, piece::achesspiece::AChessPiece};


#[derive(Clone, Copy)]
pub struct PieceMove<'a> {
    pub chess_piece: &'a dyn AChessPiece,
    pub from: Coordinate,
    pub to: Coordinate,
}

impl<'a> PieceMove<'a> {
    pub fn new(chess_piece: &'a dyn AChessPiece, from: Coordinate, to: Coordinate) -> Self {
        PieceMove {
            chess_piece,
            from,
            to,
        }
    }
}
