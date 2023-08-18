use crate::board::{color::Color, coordinate::Coordinate, piece::chesspiece::ChessPiece};

#[derive(Clone, Copy)]
pub struct PieceMove<'a> {
    pub chess_piece: &'a Box<dyn ChessPiece>,
    pub from: Coordinate,
    pub to: Coordinate,
    pub color_moving: Color,
}

impl<'a> PieceMove<'a> {
    pub fn new(
        chess_piece: &'a Box<dyn ChessPiece>,
        from: Coordinate,
        to: Coordinate,
        color_moving: Color,
    ) -> Self {
        PieceMove {
            chess_piece,
            from,
            to,
            color_moving,
        }
    }
}
