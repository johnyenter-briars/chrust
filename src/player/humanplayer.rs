use super::chessplayer::ChessPlayer;
use crate::board::{chessboard::Board, cell::color::Color};

#[derive(Debug)]
pub struct HumanPlayer {
    pub name: String,
    pub color: Color,
}

impl ChessPlayer for HumanPlayer {
    fn take_action(&self, board: Board) {
        println!("hello there from human!");
    }
    fn name(&self) -> &str{
        &self.name
    }
    fn color_abbr(&self) -> char{
        self.color.color_abbr()
    }
}