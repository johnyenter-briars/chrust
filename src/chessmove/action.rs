use super::piecemove::*;
use crate::{
    board::cell::chesspiece::ChessPiece,
    player::{aiplayer::AIPlayer, humanplayer::HumanPlayer},
};

pub trait Action<'a> {
    fn get_piece_(&self) -> &ChessPiece;
    fn get_x(&self) -> char;
    fn get_y(&self) -> i32;
    // fn get_player(&self) -> &'a dyn ChessPlayer;
}

pub struct HumanAction<'a> {
    pub piece_move: PieceMove<'a>,
    pub player: &'a HumanPlayer,
}

impl<'a> HumanAction<'a> {
    // pub fn new(piece_move: PieceMove<'a>, player: &'a HumanPlayer) -> Self {
    //     HumanAction{piece_move, player}
    // }
}

pub struct AIAction<'a> {
    pub piece_move: PieceMove<'a>,
    pub player: &'a AIPlayer,
}

impl<'a> Action<'a> for HumanAction<'a> {
    fn get_piece_(&self) -> &ChessPiece {
        self.piece_move.chess_piece
    }

    fn get_x(&self) -> char {
        'a'
    }

    fn get_y(&self) -> i32 {
        1
    }
}

impl<'a> Action<'a> for AIAction<'a> {
    fn get_piece_(&self) -> &ChessPiece {
        self.piece_move.chess_piece
    }

    fn get_x(&self) -> char {
        todo!()
    }

    fn get_y(&self) -> i32 {
        todo!()
    }
}
