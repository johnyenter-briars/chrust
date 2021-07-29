use crate::board::cell::chesspiece::*;

pub struct PieceMove<'a> {
    chess_piece: &'a ChessPiece,
    chess_move: Move,
}

impl<'a> PieceMove<'a> {
    fn new(chess_piece: &'a ChessPiece, chess_move: Move) -> Self {
        PieceMove{chess_piece, chess_move}
    }
}

pub struct Move { //denotes where the piece is moving TO
    x: char, //a-h
    y: i32, //1-8
}