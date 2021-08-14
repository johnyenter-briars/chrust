use crate::board::{cell::chesspiece::*, coordinate::Coordinate};

pub struct PieceMove<'a> {
    pub chess_piece: &'a ChessPiece,
    pub from: Coordinate,
    pub to: Coordinate,
}

impl<'a> PieceMove<'a> {
    pub fn new(chess_piece: &'a ChessPiece, from: Coordinate, to: Coordinate) -> Self {
        PieceMove{chess_piece, from, to}
    }
}

// #[derive(Clone, Copy)]
// pub struct Move { //denotes where the piece is moving TO
//     pub x: char, //a-h
//     pub y: i32, //1-8
// }