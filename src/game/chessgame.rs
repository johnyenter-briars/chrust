use crate::player::humanplayer::*;
use crate::player::aiplayer::*;

use std::error::Error;

pub struct ChessGame {
    human_player: HumanPlayer,
    ai_player: AIPlayer,
}


impl ChessGame {
    pub fn new(human_player: HumanPlayer, ai_player: AIPlayer) -> Self {
        ChessGame{human_player, ai_player}
    }

    pub fn start_game(&mut self) -> Result<&str, Box<Error>> {
        Ok(&self.human_player.name)
    }
}