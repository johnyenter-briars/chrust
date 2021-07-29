use crate::board::Board;
use crate::player::humanplayer::*;
use crate::player::aiplayer::*;

use crate::chessmove::action;

use std::error::Error;


pub struct ChessGame {
    human_player: HumanPlayer,
    ai_player: AIPlayer,
    board: Board,
}


impl ChessGame {
    pub fn new(human_player: HumanPlayer, ai_player: AIPlayer, board: Board) -> Self {
        ChessGame{human_player, ai_player, board}
    }

    pub fn start_game(&mut self) -> Result<&str, Box<Error>> {
        Ok(&self.human_player.name)
    }
}