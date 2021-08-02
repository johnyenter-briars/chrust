use crate::board::cell::chesspiece::*;

pub struct PieceMove<'a> {
    pub chess_piece: &'a ChessPiece,
    pub chess_move: Move,
}

impl<'a> PieceMove<'a> {
    pub fn new(chess_piece: &'a ChessPiece, chess_move: Move) -> Self {
        PieceMove{chess_piece, chess_move}
    }
}

#[derive(Clone, Copy)]
pub struct Move { //denotes where the piece is moving TO
    pub x: char, //a-h
    pub y: i32, //1-8
}