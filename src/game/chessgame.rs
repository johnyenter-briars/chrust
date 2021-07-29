use crate::board::chessboard::Board;
use crate::chessmove::piecemove::Move;
use crate::chessmove::{piecemove::PieceMove};
use crate::player::humanplayer::*;
use crate::player::aiplayer::*;

use crate::chessmove::action::{self, Action, HumanAction};

use std::borrow::Borrow;
use std::error::Error;


pub struct ChessGame<'a> {
    human_player: HumanPlayer,
    ai_player: AIPlayer,
    board: Board,
    history: Vec<&'a dyn Action<'a>>
}


impl<'a> ChessGame<'a> {
    pub fn new(human_player: HumanPlayer, ai_player: AIPlayer, board: Board) -> Self {
        ChessGame{human_player, ai_player, board, history:Vec::new()}
    }

    pub fn start_game(&mut self) -> Result<&str, Box<Error>> {

        //know a white pawn is at a,2
        let first_move = Move {x: 'a', y: 2};
        let target_piece = self.board.get_piece(first_move.x, first_move.y);

        let piece_move = PieceMove::new(target_piece, first_move);

        let human_action = HumanAction::new(piece_move, &self.human_player);

        let my_action: &Action = &human_action;

        self.board.apply_action(my_action);

        // println!("get piece: {:?}", target_piece);


        Ok(&self.human_player.name)
    }
}