use crate::board::{
    color::Color, coordinate::Coordinate, piece::chesspiece::ChessPiece, types::MoveType,
};

#[derive(Clone, Copy)]
pub struct PieceMove<'a> {
    pub chess_piece: &'a dyn ChessPiece,
    pub from: Coordinate,
    pub to: Coordinate,
    pub color_moving: Color,
    pub move_type: MoveType,
}

impl<'a> PieceMove<'a> {
    pub fn new(
        chess_piece: &'a dyn ChessPiece,
        from: Coordinate,
        to: Coordinate,
        color_moving: Color,
        move_type: MoveType,
    ) -> Self {
        PieceMove {
            chess_piece,
            from,
            to,
            color_moving,
            move_type,
        }
    }
}
