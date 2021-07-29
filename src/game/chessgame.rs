use crate::board::Board;
use crate::chessmove::{piecemove::PieceMove};
use crate::player::humanplayer::*;
use crate::player::aiplayer::*;

use crate::chessmove::action;

use std::borrow::Borrow;
use std::error::Error;


pub struct ChessGame {
    human_player: HumanPlayer,
    ai_player: AIPlayer,
    board: Board,
}


impl<'a> ChessGame {
    pub fn new(human_player: HumanPlayer, ai_player: AIPlayer, board: Board) -> Self {
        ChessGame{human_player, ai_player, board}
    }

    


    pub fn start_game(&mut self) -> Result<&str, Box<Error>> {

        //know a white pawn is at a,2
        let foreverpiece = self.board.get_piece('a', -1);
    

        println!("get piece: {:?}", foreverpiece);


        //human player moves white pawn up by one

        // let piece_move = PieceMove::new(, chess_move)

        // let action = HumanAction
        // {
        //     human_player = &self.human_player,

        // }

        // println!("board? :{:?}", self.board);








        Ok(&self.human_player.name)
    }
}