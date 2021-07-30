use crate::board::cell::chesspiece::ChessPiece;
use crate::board::chessboard::Board;
use crate::chessmove::piecemove::Move;
use crate::chessmove::piecemove::PieceMove;
use crate::player::aiplayer::*;
use crate::player::humanplayer::{self, *};

use crate::chessmove::action::{self, Action, HumanAction};

use std::borrow::Borrow;
use std::error::Error;


pub struct ChessGame<'a> {
    human_player: HumanPlayer,
    ai_player: AIPlayer,
    board: Board,
    history: Vec<&'a dyn Action<'a>>,
}

impl<'a> ChessGame<'a> {
    pub fn new(human_player: HumanPlayer, ai_player: AIPlayer, board: Board) -> Self {
        ChessGame {
            human_player,
            ai_player,
            board,
            history: Vec::new(),
        }
    }

    // pub fn get_move(&self) -> &dyn Action {

    // }

    pub fn start_game(&mut self) -> Result<&str, Box<dyn Error>> {
        // let idkyet = self.board.test_move_piece('a', 2, &self.human_player);
        
        self.board.print_to_screen();

        Ok(&self.human_player.name)
    }
}
