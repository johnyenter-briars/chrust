use super::chessplayer::ChessPlayer;
use crate::board::{chessboard::Board, cell::color::Color};

#[derive(Debug)]
pub struct AIPlayer {
    pub color: Color,
    pub name: String
}

impl ChessPlayer for AIPlayer {
    fn take_action(&self, board: Board) {
        println!("hello there from AI!");
    }
    fn name(&self) -> &str{
        &self.name
    }
    fn color_abbr(&self) -> char{
        self.color.color_abbr()
    }
}
